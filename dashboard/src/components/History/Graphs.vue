<template>
  <div class="flex h-full min-h-0 flex-col gap-6 overflow-y-auto p-4">
    <div
      class="flex flex-wrap items-end justify-between gap-4 rounded-2xl border bg-card px-5 py-4 shadow-sm"
    >
      <div>
        <p class="text-xs uppercase tracking-[0.24em] text-muted-foreground">
          History
        </p>
        <h2 class="text-2xl font-semibold tracking-tight">
          Live telemetry history
        </h2>
        <p class="text-sm text-muted-foreground">
          Environment values share one chart; photosensor channels share
          another.
        </p>
      </div>

      <button
        type="button"
        class="rounded-lg border px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="historyStore.clearHistory"
      >
        Clear history
      </button>
    </div>

    <div
      v-if="streamError"
      class="rounded-xl border border-destructive/40 bg-destructive/10 px-4 py-3 text-sm text-destructive"
    >
      {{ streamError }}
    </div>

    <section class="space-y-3 rounded-2xl border bg-card p-4 shadow-sm">
      <div class="flex items-center justify-between gap-3">
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
      </div>

      <div class="grid gap-4 xl:grid-cols-2">
        <div class="rounded-2xl border bg-background/60 p-3">
          <VChart class="chart" :option="temperatureOption" autoresize />
        </div>
        <div class="rounded-2xl border bg-background/60 p-3">
          <VChart class="chart" :option="pressureOption" autoresize />
        </div>
        <div class="rounded-2xl border bg-background/60 p-3 xl:col-span-2">
          <VChart class="chart" :option="humidityOption" autoresize />
        </div>
      </div>
    </section>

    <section class="space-y-3 rounded-2xl border bg-card p-4 shadow-sm">
      <div class="flex items-center justify-between gap-3">
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
      </div>

      <VChart class="chart" :option="sensorOption" autoresize />
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, provide } from 'vue'
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
const streamError = computed(() => historyStore.streamError)

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
