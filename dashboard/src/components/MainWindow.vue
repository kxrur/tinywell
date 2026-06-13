<template>
    <div class="flex h-full w-full bg-background">
        <div class="flex w-full gap-6 p-4">
            <div class="flex-1">
                <NodeCanvas />
            </div>
            <div class="flex flex-1 flex-col gap-6">
                <SensorCanvas />
                <SensorsMonitor />
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import NodeCanvas from './PresetControls/NodeCanvas.vue'
import SensorCanvas from './Sensors/SensorCanvas.vue'
import SensorsMonitor from './Sensors/SensorsMonitor.vue'
</script>
