import { Channel } from '@tauri-apps/api/core'
import { ref, watch } from 'vue'
import { defineStore } from 'pinia'
import {
  commands,
  type InstrumentFrame,
  type LedAction,
  type SerialRequest,
  type SerialResponse,
} from '@/bindings'
import { useSerialStore } from '@/stores/serial'

const TELECOMMAND_TIMEOUT_MS = 1500

const resultLabels: Record<number, string> = {
  0: 'Accepted',
  1: 'Rejected: the payload is not idle',
  2: 'Rejected: unknown command',
  3: 'Rejected: unknown experiment phase',
  4: 'Rejected: too many arguments',
  5: 'Rejected: too few arguments',
  6: 'Rejected: payload timeout',
}

export function telecommandResultLabel(result: number): string {
  return (
    resultLabels[result] ??
    `Unknown telecommand result (0x${result.toString(16).padStart(2, '0')})`
  )
}

export const useInstrumentStore = defineStore('instrument', () => {
  const serialStore = useSerialStore()
  const latestFrame = ref<InstrumentFrame | null>(null)
  const lastReceivedAt = ref<number | null>(null)
  const isStreaming = ref(false)
  const streamError = ref('')
  const pendingCommand = ref('')
  const lastCommandMessage = ref('')
  const lastCommandSucceeded = ref<boolean | null>(null)
  const startPromise = ref<Promise<void> | null>(null)

  async function ensureStreaming(): Promise<void> {
    if (startPromise.value) {
      return startPromise.value
    }
    if (isStreaming.value) {
      return
    }
    if (!serialStore.isConnected) {
      throw new Error(
        'Connect a serial device before starting instrument telemetry',
      )
    }

    streamError.value = ''
    const start = (async () => {
      const channel = new Channel<InstrumentFrame>()
      channel.onmessage = frame => {
        latestFrame.value = frame
        lastReceivedAt.value = Date.now()
        streamError.value = frame.lastError ?? ''
      }
      serialStore.unwrapCommandResult(
        await commands.subscribeInstrumentFrames(channel),
        'Failed to subscribe to instrument telemetry',
      )
      isStreaming.value = true
    })()

    startPromise.value = start
    try {
      await start
    } catch (error) {
      streamError.value =
        error instanceof Error
          ? error.message
          : 'Failed to start instrument telemetry'
      throw error
    } finally {
      startPromise.value = null
    }
  }

  function stopStreaming() {
    isStreaming.value = false
    startPromise.value = null
  }

  async function sendTelecommand(
    request: SerialRequest,
    expectedTcId: number,
    label: string,
  ): Promise<void> {
    if (!serialStore.isConnected) {
      throw new Error('Connect a serial device before sending commands')
    }
    if (pendingCommand.value) {
      throw new Error(`Wait for ${pendingCommand.value} to finish`)
    }

    pendingCommand.value = label
    lastCommandMessage.value = ''
    lastCommandSucceeded.value = null
    try {
      const response = serialStore.unwrapCommandResult(
        await commands.serialSend(request, TELECOMMAND_TIMEOUT_MS),
        `${label} failed`,
      )
      const ack = telecommandAck(response)
      if (!ack) {
        throw new Error(`${label} returned no telecommand acknowledgement`)
      }
      if (ack.tc_id !== expectedTcId) {
        throw new Error(
          `${label} acknowledgement ID 0x${ack.tc_id.toString(16).padStart(2, '0')} did not match command 0x${expectedTcId.toString(16).padStart(2, '0')}`,
        )
      }

      const message = telecommandResultLabel(ack.tc_result)
      lastCommandMessage.value = `${label}: ${message}`
      lastCommandSucceeded.value = ack.tc_result === 0
      if (ack.tc_result !== 0) {
        throw new Error(message)
      }
    } catch (error) {
      lastCommandSucceeded.value = false
      lastCommandMessage.value =
        error instanceof Error
          ? `${label}: ${error.message}`
          : `${label} failed`
      throw error
    } finally {
      pendingCommand.value = ''
    }
  }

  function nextExperimentPhase() {
    return sendTelecommand('NextExperimentPhase', 0x01, 'Next phase')
  }

  function runExperimentPhase(phase: number) {
    return sendTelecommand({ RunExperimentPhase: { phase } }, 0x02, 'Run phase')
  }

  function setStateIdle() {
    return sendTelecommand('SetStateIdle', 0x05, 'Return to idle')
  }

  function testSystemHealth() {
    return sendTelecommand('TestSystemHealth', 0x04, 'Test system health')
  }

  function resetPayload() {
    return sendTelecommand('Reset', 0x55, 'Reset payload')
  }

  function setLedState(
    wavelength: number,
    action: LedAction,
    brightness: number,
  ) {
    return sendTelecommand(
      { SetLedState: { wavelength, action, brightness } },
      0x03,
      `${wavelengthLabel(wavelength)} LED`,
    )
  }

  watch(
    () => serialStore.isConnected,
    connected => {
      if (!connected) {
        stopStreaming()
      }
    },
  )

  return {
    ensureStreaming,
    isStreaming,
    lastCommandMessage,
    lastCommandSucceeded,
    lastReceivedAt,
    latestFrame,
    nextExperimentPhase,
    pendingCommand,
    resetPayload,
    runExperimentPhase,
    setLedState,
    setStateIdle,
    stopStreaming,
    streamError,
    testSystemHealth,
  }
})

function telecommandAck(
  response: SerialResponse,
): { tc_id: number; tc_result: number } | null {
  if (
    typeof response === 'object' &&
    response !== null &&
    'TelecommandAck' in response
  ) {
    return response.TelecommandAck
  }
  return null
}

function wavelengthLabel(wavelength: number): string {
  return `${[470, 570, 630, 850][wavelength] ?? wavelength} nm`
}
