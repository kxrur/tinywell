<template>
    <div class="grid grid-cols-5 gap-4 p-6">
        <div v-for="(cell, index) in cells" :key="index" class="flex flex-col items-center space-y-1">
            <ContextMenu :open="openContextMenu === index"
                @update:open="(open) => handleContextMenuToggle(index, open)">
                <ContextMenuTrigger>
                    <Button variant="ghost" class="h-12 w-12 rounded-full flex items-center justify-center text-sm"
                        :style="{
                            backgroundColor: getCellColor(cell),
                            color: getTextColor(cell)
                        }" @click="() => toggleCell(index)" @wheel.prevent="adjustValue(index, $event)">
                        {{ cell.value }}
                    </Button>
                </ContextMenuTrigger>
                <ContextMenuContent class="w-64">
                    <ContextMenuLabel>{{ cell.label }} Settings</ContextMenuLabel>
                    <ContextMenuSeparator />

                    <div class="p-3 space-y-3">
                        <div class="flex items-center justify-between">
                            <Label class="text-sm font-medium">Active</Label>
                            <Switch :model-value="cells[index].active"
                                @update:model-value="(checked: boolean) => cells[index].active = checked" />
                        </div>

                        <ContextMenuSeparator />

                        <div class="space-y-2">
                            <div class="flex items-center justify-between">
                                <Label class="text-sm font-medium">Wavelength</Label>
                                <span class="text-sm text-muted-foreground">{{ cell.wavelength }} nm</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <div class="w-4 h-4 rounded border"
                                    :style="{ backgroundColor: wavelengthToColor(cell.wavelength) }"></div>
                                <Slider :model-value="[cell.wavelength]" @update:model-value="(value: number[] | undefined) => {
                                    if (value && value.length > 0) {
                                        cells[index].wavelength = value[0]
                                    }
                                }" :min="380" :max="700" :step="1" class="flex-1" :disabled="!cell.active" />
                            </div>
                        </div>

                        <ContextMenuSeparator />

                        <div class="space-y-2">
                            <div class="flex items-center justify-between">
                                <Label class="text-sm font-medium">Brightness</Label>
                                <span class="text-sm text-muted-foreground">{{ cell.value }}%</span>
                            </div>
                            <Slider :model-value="[cell.value]" @update:model-value="(value: number[] | undefined) => {
                                if (value && value.length > 0) {
                                    cells[index].value = value[0]
                                }
                            }" :max="100" :step="1" class="w-full" :disabled="!cell.active" />
                        </div>
                    </div>
                </ContextMenuContent>
            </ContextMenu>

            <span class="text-xs">
                {{ cell.label }}
            </span>
        </div>
    </div>
</template>

<script setup lang="ts">

interface Cell {
    label: string
    value: number
    active: boolean
    wavelength: number
}

const cells = ref<Cell[]>([
    // Row 5 (top)
    { label: "D5", value: 75, active: true, wavelength: 470 },
    { label: "E5", value: 0, active: false, wavelength: 470 },
    { label: "F5", value: 50, active: true, wavelength: 620 },
    { label: "G5", value: 25, active: true, wavelength: 530 },
    { label: "H5", value: 100, active: true, wavelength: 580 },

    // Row 4
    { label: "D4", value: 0, active: false, wavelength: 470 },
    { label: "E4", value: 80, active: true, wavelength: 450 },
    { label: "F4", value: 0, active: false, wavelength: 470 },
    { label: "G4", value: 0, active: false, wavelength: 470 },
    { label: "H4", value: 0, active: false, wavelength: 470 },

    // Row 3
    { label: "D3", value: 60, active: true, wavelength: 660 },
    { label: "E3", value: 0, active: false, wavelength: 470 },
    { label: "F3", value: 40, active: true, wavelength: 590 },
    { label: "G3", value: 90, active: true, wavelength: 490 },
    { label: "H3", value: 30, active: true, wavelength: 470 },

    // Row 2
    { label: "D2", value: 0, active: false, wavelength: 470 },
    { label: "E2", value: 0, active: false, wavelength: 470 },
    { label: "F2", value: 70, active: true, wavelength: 470 },
    { label: "G2", value: 85, active: true, wavelength: 470 },
    { label: "H2", value: 45, active: true, wavelength: 470 },

    // Row 1 (bottom)
    { label: "D1", value: 95, active: true, wavelength: 470 },
    { label: "E1", value: 0, active: false, wavelength: 470 },
    { label: "F1", value: 55, active: true, wavelength: 470 },
    { label: "G1", value: 20, active: true, wavelength: 470 },
    { label: "H1", value: 65, active: true, wavelength: 470 },
])

const openContextMenu = ref<number | null>(null)

const handleContextMenuToggle = (index: number, open: boolean) => {
    if (open) {
        openContextMenu.value = index
    } else {
        openContextMenu.value = null
    }
}

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

const getCellColor = (cell: Cell): string => {
    if (!cell.active) {
        return '#9ca3af'
    }

    if (cell.value === 0) {
        return '#ffffff'
    }

    const baseColor = wavelengthToColor(cell.wavelength)

    // Extract RGB values
    const rgbMatch = baseColor.match(/rgb\((\d+), (\d+), (\d+)\)/)
    if (!rgbMatch) return baseColor

    const r = parseInt(rgbMatch[1])
    const g = parseInt(rgbMatch[2])
    const b = parseInt(rgbMatch[3])

    // Apply brightness
    const intensity = cell.value / 100
    const newR = Math.round(255 - ((255 - r) * intensity))
    const newG = Math.round(255 - ((255 - g) * intensity))
    const newB = Math.round(255 - ((255 - b) * intensity))

    return `rgb(${newR}, ${newG}, ${newB})`
}

const getTextColor = (cell: Cell): string => {
    if (!cell.active)
        return '#ffffff'

    if (cell.value < 50)
        return '#000000'
    else
        return '#ffffff'
}

const toggleCell = (index: number) => {
    cells.value[index].active = !cells.value[index].active
}

const adjustValue = (index: number, event: WheelEvent) => {
    const cell = cells.value[index]
    if (!cell.active) return

    const delta = event.deltaY > 0 ? -5 : 5
    const newValue = Math.max(0, Math.min(100, cell.value + delta))

    cells.value[index].value = newValue
}
</script>