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

interface Cell {
  label: string
  value: number
  active: boolean
  wavelength: number
}

const cells = ref<Cell[]>(Array.from({ length: 14 }, (_, index) => ({
  label: `S${index + 1}`,
  value: 0,
  active: true,
  wavelength: 470,
})))

type SensorFrame = {
  values: number[]
  wavelength: number
}

const isMounted = ref(true)

const applyFrame = (frame: SensorFrame) => {
  const maxValue = Math.max(...frame.values, 1)

  cells.value = cells.value.map((cell, index) => {
    const raw = frame.values[index] ?? 0
    const normalized = Math.round((raw / maxValue) * 100)
    return {
      ...cell,
      value: normalized,
      wavelength: frame.wavelength,
      active: raw > 0,
    }
  })
}

onMounted(async () => {
  const channel = new Channel<SensorFrame>()
  channel.onmessage = (frame) => {
    console.log(frame)
    if (!isMounted.value) {
      return
    }
    applyFrame(frame)
  }
  await commands.serialSetPort('/dev/ttyUSB0')
  await commands.serialConnect();
  await commands.subscribeSensorFrames(channel)
})

onUnmounted(() => {
  isMounted.value = false
})
</script>
