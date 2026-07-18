<template>
  <Card class="max-w-full self-center gap-4 bg-background py-4 xl:self-start">
    <CardHeader class="px-4">
      <div class="flex items-center justify-between">
      <div>
        <div class="flex items-center gap-2">
          <Label class="text-lg font-semibold">Environment Monitor</Label>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger as-child>
                <Button
                  variant="ghost"
                  size="icon-xs"
                  :aria-label="
                    lastUpdated
                      ? `Last updated at ${lastUpdated}`
                      : 'No telemetry received yet'
                  "
                >
                  <Info class="size-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                {{
                  lastUpdated
                    ? `Updated at ${lastUpdated}`
                    : 'No telemetry received yet'
                }}
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
        <p class="text-sm text-muted-foreground">
          Live `0x83` telemetry from the MSP serial protocol.
        </p>
      </div>
      </div>
    </CardHeader>

    <CardContent class="px-4">
      <div class="max-w-full overflow-hidden rounded-lg border">
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
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Info } from 'lucide-vue-next'
import { useHistoryStore } from '@/stores/history'

const historyStore = useHistoryStore()
const environment = computed(() => historyStore.latestEnvironmentFrame)
const lastUpdated = computed(() =>
  historyStore.latestEnvironmentReceivedAt === null
    ? ''
    : new Date(historyStore.latestEnvironmentReceivedAt).toLocaleTimeString(),
)

const ambientTempC = computed(() =>
  environment.value ? environment.value.ambientTempRaw / 100 : null,
)

const ambientPressureHpa = computed(() =>
  environment.value ? environment.value.ambientPressureRaw / 100 : null,
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
      rawValue: `${environment.value.ambientPressureRaw} Pa`,
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
