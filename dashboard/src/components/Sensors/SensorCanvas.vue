<template>
  <div class="space-y-4 p-4">
    <div class="flex justify-center">
      <div class="grid grid-cols-5 gap-4 p-6">
        <CanvasCell v-for="(cell, index) in cells" :key="index" :cell="cell" :is-context-menu-open="false" read-only />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { Channel } from '@tauri-apps/api/core'
import { commands } from '@/bindings'
import { useSerialStore } from '@/stores/serial'

interface Cell {
  label: string
  value: number
  active: boolean
  wavelength: number
}

const cells = ref<Cell[]>(Array.from({ length: 14 }, (_, index) => ({
  label: `S${index + 1}`,
  value: 100,
  active: true,
  wavelength: 470,
})))

type SensorFrame = {
  values: number[]
  wavelength: number
}

const isMounted = ref(true)
const serialStore = useSerialStore()

const wavelengthEnumToNm = (wavelength: number): number => {
  switch (wavelength) {
    case 0:
      return 470
    case 1:
      return 570
    case 2:
      return 630
    case 3:
      return 850
    default:
      return 470
  }
}

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

onMounted(async () => {
  const channel = new Channel<SensorFrame>()
  channel.onmessage = (frame) => {
    if (!isMounted.value) {
      return
    }
    applyFrame(frame)
  }
  await serialStore.ensureConnected()
  serialStore.unwrapCommandResult(
    await commands.subscribeSensorFrames(channel),
    'Failed to subscribe to photosensor frames',
  )
})

onUnmounted(() => {
  isMounted.value = false
})
</script>
