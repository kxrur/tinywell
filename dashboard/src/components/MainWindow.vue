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
        <NodeCanvas />
      </div>

      <div
        v-if="activeTab === 'info'"
        class="flex min-h-0 flex-1 flex-col items-center gap-6"
      >
        <SensorCanvas />
        <SensorsMonitor />
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
import { computed, watch } from 'vue'
import { Save } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import ExperimentExport from '@/components/Export/ExperimentExport.vue'
import { useExperimentStore } from '@/stores/experiments'
import { useHistoryStore } from '@/stores/history'
import { useSerialStore } from '@/stores/serial'

const activeTab = ref<'control' | 'info' | 'history' | 'export'>('control')
const experimentStore = useExperimentStore()
const historyStore = useHistoryStore()
const serialStore = useSerialStore()
const isRecording = computed(
  () => serialStore.isConnected && experimentStore.activeExperiment !== null,
)

watch(activeTab, tab => {
  if (tab === 'info' || tab === 'history') {
    historyStore.ensureStreaming().catch(error => {
      const message =
        error instanceof Error ? error.message : 'Failed to start telemetry stream'
      toast.error('Telemetry failed to start', { description: message })
    })
  }
})
</script>
