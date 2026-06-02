<template>
    <div class="space-y-4 p-4">
        <!-- Uniform Configuration Panel -->
        <div class="flex flex-col items-center space-y-4">
            <div class="flex items-center space-x-4">
                <Label class="font-bold text-xl">Uniform Configuration</Label>
                <Switch :model-value="uniformConfigurationEnabled" @update:model-value="toggleUniformConfiguration" />
            </div>

            <!-- Animated slide-down panel -->
            <Transition enter-active-class="transition-all duration-300 ease-out"
                enter-from-class="opacity-0 transform -translate-y-4"
                enter-to-class="opacity-100 transform translate-y-0"
                leave-active-class="transition-all duration-200 ease-in"
                leave-from-class="opacity-100 transform translate-y-0"
                leave-to-class="opacity-0 transform -translate-y-4">
                <div v-if="uniformConfigurationEnabled"
                    class="flex flex-col items-center space-y-4 p-4 border rounded-lg bg-muted/20 max-w-sm">
                    <div class="flex flex-col space-y-4 w-full">
                        <BrightnessSlider :modelValue="uniformCell.value" @update:modelValue="updateUniformBrightness"
                            sliderClass="w-full" />
                        <WavelengthSlider :modelValue="uniformCell.wavelength"
                            @update:modelValue="updateUniformWavelength" sliderClass="w-full" />
                    </div>

                    <div class="text-sm text-muted-foreground text-center">
                        Changes apply immediately to all active cells
                    </div>
                </div>
            </Transition>
        </div>

        <!-- Node Canvas -->
        <div class="flex justify-center">
            <div class="grid grid-cols-5 gap-4 p-6">
                <CanvasCell v-for="(cell, index) in cells" :key="index" :cell="cell"
                    :is-context-menu-open="openContextMenu === index"
                    @update:cell="(updatedCell) => handleCellUpdate(index, updatedCell)"
                    @context-menu-toggle="(open) => handleContextMenuToggle(index, open)"
                    @toggle="() => toggleCell(index)" />
            </div>
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
    { label: "S1", value: 75, active: true, wavelength: 470 },
    { label: "S2", value: 0, active: false, wavelength: 470 },
    { label: "S3", value: 50, active: true, wavelength: 620 },
    { label: "S4", value: 25, active: true, wavelength: 530 },
    { label: "S5", value: 100, active: true, wavelength: 580 },
    { label: "S6", value: 0, active: false, wavelength: 470 },
    { label: "S7", value: 80, active: true, wavelength: 450 },
    { label: "S8", value: 0, active: false, wavelength: 470 },
    { label: "S9", value: 0, active: false, wavelength: 470 },
    { label: "S10", value: 0, active: false, wavelength: 470 },
    { label: "S11", value: 60, active: true, wavelength: 660 },
    { label: "S12", value: 0, active: false, wavelength: 470 },
    { label: "S13", value: 40, active: true, wavelength: 590 },
    { label: "S14", value: 90, active: true, wavelength: 490 },
])

const uniformConfigurationEnabled = ref<boolean>(false);

const uniformCell = ref<Cell>(
    { label: "H1", value: 65, active: true, wavelength: 470 }
)

const openContextMenu = ref<number | null>(null)

const handleContextMenuToggle = (index: number, open: boolean) => {
    if (open) {
        openContextMenu.value = index
    } else {
        openContextMenu.value = null
    }
}

const handleCellUpdate = (index: number, updatedCell: Cell) => {
    cells.value[index] = updatedCell
}

const toggleCell = (index: number) => {
    cells.value[index].active = !cells.value[index].active
}

const toggleUniformConfiguration = (enabled: boolean) => {
    uniformConfigurationEnabled.value = enabled
    if (enabled) {
        // Immediately apply current uniform settings when enabled
        applyUniformConfiguration()
    }
}

const updateUniformBrightness = (value: number) => {
    uniformCell.value.value = value
    // Apply immediately if uniform mode is enabled
    if (uniformConfigurationEnabled.value) {
        applyUniformConfiguration()
    }
}

const updateUniformWavelength = (wavelength: number) => {
    uniformCell.value.wavelength = wavelength
    // Apply immediately if uniform mode is enabled
    if (uniformConfigurationEnabled.value) {
        applyUniformConfiguration()
    }
}

const applyUniformConfiguration = () => {
    cells.value.forEach(cell => {
        cell.value = uniformCell.value.value
        cell.wavelength = uniformCell.value.wavelength
    })
}
</script>