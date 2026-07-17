import { Channel } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, reactive, ref } from 'vue'
import { commands, type EnvironmentFrame, type SensorFrame } from '@/bindings'
import { useSerialStore } from '@/stores/serial'

type HistoryPoint = [number, number]

const MAX_HISTORY_POINTS = 180
const SENSOR_COUNT = 14

function appendPoint(series: HistoryPoint[], point: HistoryPoint) {
  series.push(point)

  if (series.length > MAX_HISTORY_POINTS) {
    series.splice(0, series.length - MAX_HISTORY_POINTS)
  }
}

export function wavelengthEnumToNm(wavelength: number): number {
  switch (wavelength) {
    case 0:
      return 470
    case 1:
      return 570
    case 2:
      return 630
    case 3:
      return 850
    default:
      return 470
  }
}

export const useHistoryStore = defineStore('history', () => {
  const serialStore = useSerialStore()
  const isStreaming = ref(false)
  const startPromise = ref<Promise<void> | null>(null)
  const streamError = ref('')
  const latestSensorWavelength = ref<number | null>(null)
  const latestEnvironmentFrame = ref<EnvironmentFrame | null>(null)
  const latestEnvironmentReceivedAt = ref<number | null>(null)
  const latestSensorFrame = ref<SensorFrame | null>(null)

  const environmentHistory = reactive({
    wellTempC: [] as HistoryPoint[],
    ambientTempC: [] as HistoryPoint[],
    ambientPressureHpa: [] as HistoryPoint[],
    ambientHumidityPct: [] as HistoryPoint[],
  })

  const sensorHistory = reactive(
    Array.from({ length: SENSOR_COUNT }, () => [] as HistoryPoint[]),
  )

  const sensorWavelengthNm = computed(() =>
    latestSensorWavelength.value === null
      ? null
      : wavelengthEnumToNm(latestSensorWavelength.value),
  )

  async function ensureStreaming(): Promise<void> {
    if (startPromise.value) {
      return startPromise.value
    }

    if (isStreaming.value) {
      return
    }

    streamError.value = ''
    startPromise.value = (async () => {
      await serialStore.ensureConnected()

      const sensorChannel = new Channel<SensorFrame>()
      sensorChannel.onmessage = frame => {
        latestSensorFrame.value = frame
        latestSensorWavelength.value = frame.wavelength

        const timestamp = Date.now()
        frame.values.slice(0, SENSOR_COUNT).forEach((value, index) => {
          appendPoint(sensorHistory[index], [timestamp, value])
        })
      }

      const environmentChannel = new Channel<EnvironmentFrame>()
      environmentChannel.onmessage = frame => {
        latestEnvironmentFrame.value = frame

        const timestamp = Date.now()
        latestEnvironmentReceivedAt.value = timestamp
        appendPoint(environmentHistory.wellTempC, [timestamp, frame.wellTempC])
        appendPoint(environmentHistory.ambientTempC, [
          timestamp,
          frame.ambientTempRaw / 100,
        ])
        appendPoint(environmentHistory.ambientPressureHpa, [
          timestamp,
          frame.ambientPressureRaw / 100,
        ])
        appendPoint(environmentHistory.ambientHumidityPct, [
          timestamp,
          frame.ambientHumidityRaw / 1024,
        ])
      }

      serialStore.unwrapCommandResult(
        await commands.subscribeSensorFrames(sensorChannel),
        'Failed to subscribe to photosensor telemetry',
      )
      serialStore.unwrapCommandResult(
        await commands.subscribeEnvironmentFrames(environmentChannel),
        'Failed to subscribe to environment telemetry',
      )

      isStreaming.value = true
    })()

    try {
      await startPromise.value
    } catch (error) {
      streamError.value =
        error instanceof Error ? error.message : 'Failed to start telemetry stream'
      throw error
    } finally {
      startPromise.value = null
    }
  }

  function clearHistory() {
    environmentHistory.wellTempC.splice(0)
    environmentHistory.ambientTempC.splice(0)
    environmentHistory.ambientPressureHpa.splice(0)
    environmentHistory.ambientHumidityPct.splice(0)
    sensorHistory.forEach(series => series.splice(0))
    latestEnvironmentFrame.value = null
    latestEnvironmentReceivedAt.value = null
    latestSensorFrame.value = null
    latestSensorWavelength.value = null
  }

  return {
    clearHistory,
    ensureStreaming,
    environmentHistory,
    isStreaming,
    latestEnvironmentFrame,
    latestEnvironmentReceivedAt,
    latestSensorFrame,
    latestSensorWavelength,
    sensorHistory,
    sensorWavelengthNm,
    streamError,
  }
})
