<!-- BrightnessSlider.vue -->
<template>
    <div class="space-y-2">
        <div class="flex items-center justify-between w-full">
            <Label class="text-sm font-medium">Brightness</Label>
            <span class="text-sm text-muted-foreground ml-4">{{ modelValue }}%</span>
        </div>
        <Slider :model-value="[modelValue]" @update:model-value="handleChange" :max="100" :step="1" :class="sliderClass"
            :disabled="disabled" />
    </div>
</template>

<script setup lang="ts">

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
    sliderClass: 'w-full'
})

const emit = defineEmits<Emits>()

const handleChange = (value: number[] | undefined) => {
    if (value && value.length > 0) {
        emit('update:modelValue', value[0])
    }
}
</script>