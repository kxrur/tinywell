<template>
    <div class="flex h-full w-full bg-background">
        <div class="flex w-full gap-6 p-4">
            <div class="flex-1">
                <NodeCanvas />
            </div>
            <div class="flex-1">
                <SensorCanvas />
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import NodeCanvas from './PresetControls/NodeCanvas.vue'
import SensorCanvas from './Sensors/SensorCanvas.vue'
</script>
