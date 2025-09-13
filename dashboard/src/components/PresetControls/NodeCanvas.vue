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
}

const cells = ref<Cell[]>([
    // Row 5 (top)
    { label: "D5", value: 75, active: true },
    { label: "E5", value: 0, active: false },
    { label: "F5", value: 50, active: true },
    { label: "G5", value: 25, active: true },
    { label: "H5", value: 100, active: true },

    // Row 4
    { label: "D4", value: 0, active: false },
    { label: "E4", value: 80, active: true },
    { label: "F4", value: 0, active: false },
    { label: "G4", value: 0, active: false },
    { label: "H4", value: 0, active: false },

    // Row 3
    { label: "D3", value: 60, active: true },
    { label: "E3", value: 0, active: false },
    { label: "F3", value: 40, active: true },
    { label: "G3", value: 90, active: true },
    { label: "H3", value: 30, active: true },

    // Row 2
    { label: "D2", value: 0, active: false },
    { label: "E2", value: 0, active: false },
    { label: "F2", value: 70, active: true },
    { label: "G2", value: 85, active: true },
    { label: "H2", value: 45, active: true },

    // Row 1 (bottom)
    { label: "D1", value: 95, active: true },
    { label: "E1", value: 0, active: false },
    { label: "F1", value: 55, active: true },
    { label: "G1", value: 20, active: true },
    { label: "H1", value: 65, active: true },
])

const openContextMenu = ref<number | null>(null)

const handleContextMenuToggle = (index: number, open: boolean) => {
    if (open) {
        openContextMenu.value = index
    } else {
        openContextMenu.value = null
    }
}

const getCellColor = (cell: Cell): string => {
    if (!cell.active) {
        return '#9ca3af'
    }

    if (cell.value === 0) {
        return '#ffffff'
    }

    const intensity = cell.value / 100
    const red = Math.round(220 - (intensity * 80))
    const green = Math.round(230 - (intensity * 80))
    const blue = 255

    return `rgb(${red}, ${green}, ${blue})`
}

const getTextColor = (cell: Cell): string => {
    if (!cell.active)
        return '#ffffff'

    if (cell.value < 100)
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