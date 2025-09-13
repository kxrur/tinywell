<template>
    <div class="space-y-2">
        <div class="flex items-center justify-between">
            <Label class="text-sm font-medium">Wavelength</Label>
            <span class="text-sm text-muted-foreground">{{ modelValue }}nm</span>
        </div>
        <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded border" :style="{ backgroundColor: wavelengthToColor(modelValue) }"></div>
            <Slider :model-value="[modelValue]" @update:model-value="handleChange" :min="380" :max="700" :step="1"
                :class="sliderClass" :disabled="disabled" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { Slider } from '@/components/ui/slider'
import { Label } from '@/components/ui/label'

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

const wavelengthToColor = (wavelength: number): string => {
    const Gamma = 0.80
    const IntensityMax = 255
    let factor: number, red: number, green: number, blue: number

    if ((wavelength >= 380) && (wavelength < 440)) {
        red = -(wavelength - 440) / (440 - 380)
        green = 0.0
        blue = 1.0
    } else if ((wavelength >= 440) && (wavelength < 490)) {
        red = 0.0
        green = (wavelength - 440) / (490 - 440)
        blue = 1.0
    } else if ((wavelength >= 490) && (wavelength < 510)) {
        red = 0.0
        green = 1.0
        blue = -(wavelength - 510) / (510 - 490)
    } else if ((wavelength >= 510) && (wavelength < 580)) {
        red = (wavelength - 510) / (580 - 510)
        green = 1.0
        blue = 0.0
    } else if ((wavelength >= 580) && (wavelength < 645)) {
        red = 1.0
        green = -(wavelength - 645) / (645 - 580)
        blue = 0.0
    } else if ((wavelength >= 645) && (wavelength < 781)) {
        red = 1.0
        green = 0.0
        blue = 0.0
    } else {
        red = 0.0
        green = 0.0
        blue = 0.0
    }

    // Let the intensity fall off near the vision limits
    if ((wavelength >= 380) && (wavelength < 420)) {
        factor = 0.3 + 0.7 * (wavelength - 380) / (420 - 380)
    } else if ((wavelength >= 420) && (wavelength < 701)) {
        factor = 1.0
    } else if ((wavelength >= 701) && (wavelength < 781)) {
        factor = 0.3 + 0.7 * (780 - wavelength) / (780 - 700)
    } else {
        factor = 0.0
    }

    if (red !== 0) {
        red = Math.round(IntensityMax * Math.pow(red * factor, Gamma))
    }
    if (green !== 0) {
        green = Math.round(IntensityMax * Math.pow(green * factor, Gamma))
    }
    if (blue !== 0) {
        blue = Math.round(IntensityMax * Math.pow(blue * factor, Gamma))
    }

    return `rgb(${red}, ${green}, ${blue})`
}

const handleChange = (value: number[] | undefined) => {
    if (value && value.length > 0) {
        emit('update:modelValue', value[0])
    }
}
</script>