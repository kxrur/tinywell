<template>
    <div>
        <div>
            <div class="flex items-center ml-12">
                <Label class="font-bold text-xl mr-2">Uniform Configuration</Label>
                <Switch />
            </div>
        </div>

        <div class="grid grid-cols-5 gap-4 p-6">
            <CanvasCell v-for="(cell, index) in cells" :key="index" :cell="cell"
                :is-context-menu-open="openContextMenu === index"
                @update:cell="(updatedCell) => handleCellUpdate(index, updatedCell)"
                @context-menu-toggle="(open) => handleContextMenuToggle(index, open)"
                @toggle="() => toggleCell(index)" />
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

const handleCellUpdate = (index: number, updatedCell: Cell) => {
    cells.value[index] = updatedCell
}

const toggleCell = (index: number) => {
    cells.value[index].active = !cells.value[index].active
}
</script>