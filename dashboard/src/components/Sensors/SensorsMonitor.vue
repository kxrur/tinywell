<template>
  <div
    class="inline-flex max-w-full self-center flex-col gap-4 rounded-lg border bg-background p-4"
  >
    <div class="flex items-center justify-between">
      <div>
        <div class="flex items-center gap-2">
          <Label class="text-lg font-semibold">Environment Monitor</Label>
          <TelemetryUpdatedAt
            :received-at="historyStore.latestEnvironmentReceivedAt"
            source="Environment"
          />
        </div>
        <p class="text-sm text-muted-foreground">
          Live `0x83` telemetry from the MSP serial protocol.
        </p>
      </div>
    </div>

    <div
      class="inline-block max-w-full self-center overflow-hidden rounded-lg border"
    >
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead class="w-[220px]">Field</TableHead>
            <TableHead>Display Value</TableHead>
            <TableHead>Raw Value</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="row in rows" :key="row.key">
            <TableCell class="font-medium">{{ row.label }}</TableCell>
            <TableCell>
              <Badge variant="outline">{{ row.displayValue }}</Badge>
            </TableCell>
            <TableCell class="font-mono text-sm">{{ row.rawValue }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import TelemetryUpdatedAt from '@/components/TelemetryUpdatedAt.vue'
import { useHistoryStore } from '@/stores/history'

const historyStore = useHistoryStore()
const environment = computed(() => historyStore.latestEnvironmentFrame)
const ambientTempC = computed(() =>
  environment.value ? environment.value.ambientTempRaw / 100 : null,
)

const ambientPressureHpa = computed(() =>
  environment.value ? environment.value.ambientPressureRaw / 25600 : null,
)

const ambientHumidityPercent = computed(() =>
  environment.value ? environment.value.ambientHumidityRaw / 1024 : null,
)

const rows = computed(() => {
  if (!environment.value) {
    return [
      {
        key: 'waiting',
        label: 'Environment telemetry',
        displayValue: 'No data yet',
        rawValue: 'N/A',
      },
    ]
  }

  return [
    {
      key: 'well-temp',
      label: 'Well Temp',
      displayValue: `${environment.value.wellTempC} °C`,
      rawValue: `${environment.value.wellTempC}`,
    },
    {
      key: 'ambient-temp',
      label: 'Ambient Temp',
      displayValue: `${ambientTempC.value?.toFixed(2)} °C`,
      rawValue: `${environment.value.ambientTempRaw} cC`,
    },
    {
      key: 'ambient-pressure',
      label: 'Ambient Pressure',
      displayValue: `${ambientPressureHpa.value?.toFixed(2)} hPa`,
      rawValue: `${environment.value.ambientPressureRaw} Q24.8 Pa`,
    },
    {
      key: 'ambient-humidity',
      label: 'Ambient Humidity',
      displayValue: `${ambientHumidityPercent.value?.toFixed(2)} %RH`,
      rawValue: `${environment.value.ambientHumidityRaw}`,
    },
  ]
})
</script>
