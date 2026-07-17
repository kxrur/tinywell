<template>
  <div class="space-y-4 p-4">
    <div class="flex justify-center">
      <div class="flex flex-col gap-4 p-6">
        <div
          v-for="(row, rowIndex) in layoutRows"
          :key="rowIndex"
          class="grid grid-cols-4 gap-4"
        >
          <template
            v-for="slot in row"
            :key="slot === null ? `empty-${rowIndex}` : slot"
          >
            <div v-if="slot === null" class="size-20" />
            <CanvasCell
              v-else
              :cell="cells[slot]"
              :is-context-menu-open="false"
              read-only
            />
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useHistoryStore, wavelengthEnumToNm } from '@/stores/history'

interface Cell {
  label: string
  value: number
  active: boolean
  wavelength: number
}

const cells = ref<Cell[]>(
  Array.from({ length: 14 }, (_, index) => ({
    label: `S${index + 1}`,
    value: 100,
    active: true,
    wavelength: 470,
  })),
)

type SensorFrame = {
  values: number[]
  wavelength: number
}

const historyStore = useHistoryStore()
const layoutRows = [
  [null, 0, 1, 2],
  [3, 4, 5, 6],
  [7, 8, 9, 10],
  [null, 11, 12, 13],
] as const

const applyFrame = (frame: SensorFrame) => {
  const displayWavelength = wavelengthEnumToNm(frame.wavelength)
  const maxValue = Math.max(...frame.values, 1)

  cells.value = cells.value.map((cell, index) => {
    const hasReading = index < frame.values.length
    const raw = frame.values[index] ?? 0
    const normalized = Math.round((raw / maxValue) * 100)
    return {
      ...cell,
      value: hasReading ? normalized : 0,
      wavelength: hasReading ? displayWavelength : cell.wavelength,
      active: hasReading,
    }
  })
}

watch(
  () => historyStore.latestSensorFrame,
  frame => {
    if (frame) {
      applyFrame(frame)
    }
  },
  { immediate: true },
)
</script>
