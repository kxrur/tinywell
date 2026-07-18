<template>
  <div class="flex h-full w-full bg-background p-4">
    <div class="flex w-full flex-col gap-6">
      <div class="relative flex min-h-9 items-center">
        <MainWindowTabs v-model="activeTab" class="mx-auto" />
        <TooltipProvider v-if="isRecording" class="absolute right-0">
          <Tooltip>
            <TooltipTrigger as-child>
              <Badge variant="outline" class="gap-1.5 border-green-500/40 bg-green-500/10 text-green-700 dark:text-green-400">
                <Save class="size-3.5" />
                <span class="text-xs font-semibold tracking-wide">SAVING</span>
              </Badge>
            </TooltipTrigger>
            <TooltipContent>
              Recording to {{ experimentStore.activeExperiment?.name }}
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>

      <div v-if="activeTab === 'control'" class="min-h-0 flex-1">
        <div v-if="!serialStore.isConnected" class="flex h-full items-center justify-center rounded-2xl border border-dashed p-8 text-center text-sm text-muted-foreground">
          Connect a serial device from the sidebar to control the instrument.
        </div>
        <NodeCanvas v-else />
      </div>

      <div
        v-if="activeTab === 'info'"
        class="flex min-h-0 flex-1 flex-col items-center gap-6 xl:flex-row xl:items-start xl:justify-center"
      >
        <div v-if="!serialStore.isConnected" class="flex h-full w-full items-center justify-center rounded-2xl border border-dashed p-8 text-center text-sm text-muted-foreground">
          Connect a serial device from the sidebar to view live sensor data.
        </div>
        <template v-else>
          <SensorCanvas />
          <SensorsMonitor />
        </template>
      </div>

      <div v-if="activeTab === 'history'" class="min-h-0 flex-1">
        <Graphs></Graphs>
      </div>

      <div v-if="activeTab === 'export'" class="min-h-0 flex-1">
        <ExperimentExport />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import { Save } from 'lucide-vue-next'
import ExperimentExport from '@/components/Export/ExperimentExport.vue'
import { useExperimentStore } from '@/stores/experiments'
import { useSerialStore } from '@/stores/serial'

const activeTab = ref<'control' | 'info' | 'history' | 'export'>('control')
const experimentStore = useExperimentStore()
const serialStore = useSerialStore()
const isRecording = computed(
  () => serialStore.isConnected && experimentStore.activeExperiment !== null,
)

</script>
