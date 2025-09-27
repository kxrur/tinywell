<template>
    <div class="space-y-2">
        <div class="flex items-center justify-between w-full">
            <Label class="text-sm font-medium">Wavelength</Label>
            <span class="text-sm text-muted-foreground ml-4">{{ modelValue }}nm</span>
        </div>
        <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded border" :style="{ backgroundColor: convertWavelengthToColor(modelValue) }">
            </div>
            <Slider :model-value="[modelValue]" @update:model-value="handleChange" :min="380" :max="700" :step="1"
                :class="sliderClass" :disabled="disabled" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { convertWavelengthToColor } from '@/lib/wavelength'

interface Props {
    modelValue: number
    disabled?: boolean
    sliderClass?: string
}

interface Emits {
    (e: 'update:modelValue', value: number): void
}

const props = withDefaults(defineProps<Props>(), {
    disabled: false,
    sliderClass: 'flex-1'
})

const emit = defineEmits<Emits>()


const handleChange = (value: number[] | undefined) => {
    if (value && value.length > 0) {
        emit('update:modelValue', value[0])
    }
}
</script>