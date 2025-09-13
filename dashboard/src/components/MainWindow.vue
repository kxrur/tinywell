<template>
    <div class="flex h-screen w-full bg-background">
        <div class="flex flex-col w-3/12 h-full justify-center space-y-36 p-4">
            <PumpControls name="Pump 1" />
            <PumpControls name="Pump 2" />
        </div>

        <div class="flex flex-col w-3/12">
            <div class="flex-1">
                <NodeCanvas />
            </div>
        </div>

        <div class="flex flex-col w-3/12">
            <div class="p-4">
                <ValveControl />
            </div>
        </div>
        <div class="flex flex-col w-3/12 border-l-2">
            <div class="flex-1 p-4">
                <SensorsMonitor></SensorsMonitor>
            </div>
            <div class="p-4">
                <TestControls></TestControls>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">

</script>
