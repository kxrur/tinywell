<template>
    <div class="flex h-full w-full bg-background p-4">
        <div class="flex w-full flex-col gap-6">
            <MainWindowTabs v-model="activeTab" />

            <div v-if="activeTab === 'control'" class="min-h-0 flex-1">
                <NodeCanvas />
            </div>

            <div v-else class="flex min-h-0 flex-1 flex-col items-center gap-6">
                <SensorCanvas />
                <SensorsMonitor />
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">

const activeTab = ref<'control' | 'info'>('control')
</script>
