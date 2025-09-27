<template>
    <div class="flex flex-col items-center space-y-2 p-2 bg-background">
        <!-- Pump Label -->
        <Label class="text-lg font-semibold">{{ props.name }}</Label>

        <div class="relative">
            <svg width="120" height="120" viewBox="0 0 120 120" class="text-blue-500">
                <rect x="30" y="45" width="60" height="45" rx="12" fill="currentColor" stroke="currentColor"
                    stroke-width="3" />

                <rect x="33" :y="87 - (39 * intensity / 100)" width="54" :height="39 * intensity / 100" rx="9"
                    fill="white" opacity="0.7" class="transition-all duration-700 ease-out" />

                <circle cx="48" cy="63" r="3" fill="white" />
                <circle cx="72" cy="63" r="3" fill="white" />

                <path v-if="intensity > 0" d="M 48 72 Q 60 78 72 72" stroke="white" stroke-width="3" fill="none"
                    stroke-linecap="round" />
                <line v-else x1="52" y1="72" x2="68" y2="72" stroke="white" stroke-width="3" stroke-linecap="round" />

                <rect x="12" y="63" width="18" height="6" fill="currentColor" />
                <circle cx="12" cy="66" r="3" fill="currentColor" />

                <rect x="90" y="63" width="18" height="6" fill="currentColor" />
                <circle cx="108" cy="66" r="3" fill="currentColor" />

                <rect x="48" y="30" width="24" height="15" rx="4" fill="currentColor" />
                <circle cx="60" cy="37" r="2" fill="white" />

                <rect x="36" y="90" width="6" height="9" rx="3" fill="currentColor" opacity="0.6" />
                <rect x="78" y="90" width="6" height="9" rx="3" fill="currentColor" opacity="0.6" />
            </svg>
        </div>

        <div class="text-center">
            <div class="text-sm font-medium text-gray-700">Intensity</div>
            <div class="text-xl font-bold" :class="getIntensityColor()">
                {{ intensity }}%
            </div>
        </div>

        <!-- Intensity Slider -->
        <div class="w-full max-w-sm space-y-2">
            <Label class="text-sm font-medium">Pump Speed</Label>
            <Slider :model-value="[intensity]" @update:model-value="updateIntensity" :max="100" :step="1"
                class="w-full" />
            <div class="flex justify-between text-xs text-gray-500">
                <span>Off</span>
                <span>Max</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Slider } from '@/components/ui/slider'
import { Label } from '@/components/ui/label'

interface Props {
    name?: string
}

const props = withDefaults(defineProps<Props>(), {
    name: 'Pump'
})

const intensity = ref(0)

const updateIntensity = (value: number[] | undefined) => {
    if (value && value.length > 0) {
        intensity.value = value[0]
    }
}

const getIntensityColor = () => {
    if (intensity.value === 0) return 'text-gray-400'
    if (intensity.value < 30) return 'text-blue-400'
    if (intensity.value < 70) return 'text-blue-500'
    return 'text-blue-600'
}
</script>

<style scoped>
@keyframes bubble {

    0%,
    100% {
        transform: scale(1);
        opacity: 0.8;
    }

    50% {
        transform: scale(1.3);
        opacity: 0.4;
    }
}
</style>
