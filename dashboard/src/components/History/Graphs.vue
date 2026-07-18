<template>
  <ScrollArea class="h-full min-h-0">
    <div class="flex min-h-0 flex-col gap-6 p-4">

    <Alert v-if="streamError" variant="destructive">
      <AlertTitle>Telemetry stream error</AlertTitle>
      <AlertDescription>{{ streamError }}</AlertDescription>
    </Alert>

    <Card class="gap-3 py-4">
      <CardHeader class="flex flex-row items-center justify-between gap-3 px-4">
        <div>
          <h3 class="text-lg font-semibold">Environment</h3>
          <p class="text-sm text-muted-foreground">
            Well temperature, ambient temperature, pressure, and humidity on one
            scatter chart.
          </p>
        </div>
        <p class="text-sm text-muted-foreground">
          {{ environmentPointCount }} points
        </p>
      </CardHeader>

      <CardContent class="grid gap-4 px-4 xl:grid-cols-2">
        <div class="rounded-2xl border bg-background/60 p-3">
          <VChart class="chart" :option="temperatureOption" autoresize />
        </div>
        <div class="rounded-2xl border bg-background/60 p-3">
          <VChart class="chart" :option="pressureOption" autoresize />
        </div>
        <div class="rounded-2xl border bg-background/60 p-3 xl:col-span-2">
          <VChart class="chart" :option="humidityOption" autoresize />
        </div>
      </CardContent>
    </Card>

    <Card class="gap-3 py-4">
      <CardHeader class="flex flex-row items-center justify-between gap-3 px-4">
        <div>
          <h3 class="text-lg font-semibold">Photosensors</h3>
          <p class="text-sm text-muted-foreground">
            All 14 sensor channels on one scatter chart. Latest wavelength:
            {{ sensorWavelengthLabel ?? 'unknown' }} nm
          </p>
        </div>
        <p class="text-sm text-muted-foreground">
          {{ sensorPointCount }} points
        </p>
      </CardHeader>

      <CardContent class="px-4">
        <VChart class="chart" :option="sensorOption" autoresize />
      </CardContent>
    </Card>
    </div>
  </ScrollArea>
</template>

<script setup lang="ts">
import { computed, onMounted, provide, watch } from 'vue'
import { toast } from 'vue-sonner'
import { CanvasRenderer } from 'echarts/renderers'
import { ScatterChart } from 'echarts/charts'
import {
  GridComponent,
  LegendComponent,
  TitleComponent,
  TooltipComponent,
} from 'echarts/components'
import { registerTheme, use } from 'echarts/core'
import VChart, { THEME_KEY } from 'vue-echarts'
import { useExperimentStore } from '@/stores/experiments'
import { useHistoryStore } from '@/stores/history'
import light from '@/theme/echarts/light.json'

registerTheme('light', light)

use([
  CanvasRenderer,
  ScatterChart,
  GridComponent,
  LegendComponent,
  TitleComponent,
  TooltipComponent,
])

provide(THEME_KEY, 'dark')

type HistoryPoint = [number, number]
type EChartsSeries = {
  name: string
  type: 'scatter'
  data: HistoryPoint[]
  symbolSize: number
}

const historyStore = useHistoryStore()
const experimentStore = useExperimentStore()
const streamError = computed(() => historyStore.streamError)

async function loadActiveExperimentHistory() {
  const experimentId = experimentStore.activeExperiment?.id
  if (experimentId === null || experimentId === undefined) {
    historyStore.clearHistory()
    return
  }

  try {
    await historyStore.loadExperimentHistory(experimentId)
  } catch (error) {
    const message =
      error instanceof Error ? error.message : 'Failed to load experiment history'
    toast.error('Failed to load experiment history', { description: message })
  }
}

onMounted(() => {
  void loadActiveExperimentHistory()
})

watch(
  () => experimentStore.activeExperiment?.id,
  () => {
    historyStore.clearHistory()
    void loadActiveExperimentHistory()
  },
)

const formatTimestamp = (value: number) =>
  new Date(value).toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })

const buildMultiSeriesOption = (
  title: string,
  unit: string,
  series: EChartsSeries[],
) => ({
  title: {
    text: title,
    left: 'center',
    top: 0,
    textStyle: {
      fontSize: 14,
      fontWeight: 600,
    },
  },
  legend: {
    type: 'scroll',
    top: 28,
    left: 10,
    right: 10,
  },
  grid: {
    left: 46,
    right: 18,
    top: 72,
    bottom: 40,
    containLabel: true,
  },
  tooltip: {
    trigger: 'item',
    formatter: (params: { seriesName: string; value: [number, number] }) => {
      const [timestamp, value] = params.value
      return `${params.seriesName}<br/>${formatTimestamp(timestamp)}<br/>${value.toLocaleString()} ${unit}`
    },
  },
  xAxis: {
    type: 'time',
    axisLabel: {
      formatter: (value: number) => formatTimestamp(value),
    },
  },
  yAxis: {
    type: 'value',
    name: unit,
    nameLocation: 'middle',
    nameGap: 34,
    splitLine: {
      lineStyle: {
        color: 'rgba(148, 163, 184, 0.18)',
      },
    },
  },
  series,
})

const temperatureOption = computed(() =>
  buildMultiSeriesOption('Temperature history', '°C', [
    {
      name: 'Well temp',
      type: 'scatter',
      data: historyStore.environmentHistory.wellTempC,
      symbolSize: 7,
    },
    {
      name: 'Ambient temp',
      type: 'scatter',
      data: historyStore.environmentHistory.ambientTempC,
      symbolSize: 7,
    },
  ]),
)

const pressureOption = computed(() =>
  buildMultiSeriesOption('Ambient pressure', 'hPa', [
    {
      name: 'Pressure',
      type: 'scatter',
      data: historyStore.environmentHistory.ambientPressureHpa,
      symbolSize: 7,
    },
  ]),
)

const humidityOption = computed(() =>
  buildMultiSeriesOption('Ambient humidity', '%RH', [
    {
      name: 'Humidity',
      type: 'scatter',
      data: historyStore.environmentHistory.ambientHumidityPct,
      symbolSize: 7,
    },
  ]),
)

const sensorOption = computed(() =>
  buildMultiSeriesOption(
    'Photosensor history',
    'raw',
    historyStore.sensorHistory.map((points, index) => ({
      name: `Sensor ${index + 1}`,
      type: 'scatter' as const,
      data: points,
      symbolSize: 6,
    })),
  ),
)

const environmentPointCount = computed(() =>
  Object.values(historyStore.environmentHistory).reduce(
    (sum, series) => sum + series.length,
    0,
  ),
)

const sensorPointCount = computed(() =>
  historyStore.sensorHistory.reduce((sum, series) => sum + series.length, 0),
)

const sensorWavelengthLabel = computed(() => historyStore.sensorWavelengthNm)

</script>

<style scoped>
.chart {
  height: 360px;
  width: 100%;
}
</style>
