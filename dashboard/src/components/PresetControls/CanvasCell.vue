<!-- CanvasCell.vue -->
<template>
    <div class="flex flex-col items-center space-y-1">
        <ContextMenu :open="isContextMenuOpen" @update:open="handleContextMenuToggle">
            <ContextMenuTrigger>
                <Button variant="ghost" class="h-12 w-12 rounded-full flex items-center justify-center text-sm" :style="{
                    backgroundColor: getCellColor(),
                    color: getTextColor()
                }" @click="handleToggle" @wheel.prevent="handleWheelAdjust">
                    {{ cell.value }}
                </Button>
            </ContextMenuTrigger>
            <ContextMenuContent class="w-64">
                <ContextMenuLabel>{{ cell.label }} Settings</ContextMenuLabel>
                <ContextMenuSeparator />

                <div class="p-3 space-y-3">
                    <div class="flex items-center justify-between">
                        <Label class="text-sm font-medium">Active</Label>
                        <Switch :model-value="cell.active" @update:model-value="handleActiveChange" />
                    </div>

                    <ContextMenuSeparator />

                    <WavelengthSlider :model-value="cell.wavelength" @update:model-value="handleWavelengthChange"
                        :disabled="!cell.active || disableIndividualControls" />

                    <ContextMenuSeparator />

                    <BrightnessSlider :model-value="cell.value" @update:model-value="handleValueChange"
                        :disabled="!cell.active || disableIndividualControls" />
                </div>
            </ContextMenuContent>
        </ContextMenu>

        <span class="text-xs">
            {{ cell.label }}
        </span>
    </div>
</template>

<script setup lang="ts">

interface Cell {
    label: string
    value: number
    active: boolean
    wavelength: number
}

interface Props {
    cell: Cell
    isContextMenuOpen: boolean
    disableIndividualControls?: boolean
}

interface Emits {
    (e: 'update:cell', cell: Cell): void
    (e: 'context-menu-toggle', open: boolean): void
    (e: 'toggle'): void
}

const props = withDefaults(defineProps<Props>(), {
    disableIndividualControls: false
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

const getCellColor = (): string => {
    if (!props.cell.active) {
        return '#9ca3af'
    }

    if (props.cell.value === 0) {
        return '#ffffff'
    }

    const baseColor = wavelengthToColor(props.cell.wavelength)

    // Extract RGB values
    const rgbMatch = baseColor.match(/rgb\((\d+), (\d+), (\d+)\)/)
    if (!rgbMatch) return baseColor

    const r = parseInt(rgbMatch[1])
    const g = parseInt(rgbMatch[2])
    const b = parseInt(rgbMatch[3])

    // Apply brightness
    const intensity = props.cell.value / 100
    const newR = Math.round(255 - ((255 - r) * intensity))
    const newG = Math.round(255 - ((255 - g) * intensity))
    const newB = Math.round(255 - ((255 - b) * intensity))

    return `rgb(${newR}, ${newG}, ${newB})`
}

const getTextColor = (): string => {
    if (!props.cell.active)
        return '#ffffff'

    if (props.cell.value < 50)
        return '#000000'
    else
        return '#ffffff'
}

const handleContextMenuToggle = (open: boolean) => {
    emit('context-menu-toggle', open)
}

const handleToggle = () => {
    emit('toggle')
}

const handleWheelAdjust = (event: WheelEvent) => {
    if (!props.cell.active) return

    const delta = event.deltaY > 0 ? -5 : 5
    const newValue = Math.max(0, Math.min(100, props.cell.value + delta))

    const updatedCell = { ...props.cell, value: newValue }
    emit('update:cell', updatedCell)
}

const handleActiveChange = (checked: boolean) => {
    const updatedCell = { ...props.cell, active: checked }
    emit('update:cell', updatedCell)
}

const handleWavelengthChange = (wavelength: number) => {
    const updatedCell = { ...props.cell, wavelength }
    emit('update:cell', updatedCell)
}

const handleValueChange = (value: number) => {
    const updatedCell = { ...props.cell, value }
    emit('update:cell', updatedCell)
}
</script>