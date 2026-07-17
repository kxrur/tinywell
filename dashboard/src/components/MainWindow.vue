<template>
  <div class="flex h-full w-full bg-background p-4">
    <div class="flex w-full flex-col gap-6">
      <div class="relative flex min-h-9 items-center">
        <MainWindowTabs v-model="activeTab" class="mx-auto" />
        <Badge
          class="absolute right-0 max-w-64"
          :variant="experimentStore.activeExperiment ? 'default' : 'secondary'"
        >
          <span class="truncate">
            {{ experimentStore.activeExperiment?.name ?? 'Not recording' }}
          </span>
        </Badge>
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
    </div>
  </div>
</template>
<script setup lang="ts">
import { useExperimentStore } from '@/stores/experiments'

const activeTab = ref<'control' | 'info' | 'history'>('control')
const experimentStore = useExperimentStore()
</script>
