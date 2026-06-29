<template>
  <div class="flex justify-center rounded-xl border bg-card p-2 shadow-sm">
    <div class="grid w-full max-w-md grid-cols-3 gap-2">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        type="button"
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors"
        :class="
          modelValue === tab.value
            ? 'bg-primary text-primary-foreground shadow-sm'
            : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
        "
        @click="emit('update:modelValue', tab.value)"
      >
        {{ tab.label }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
type TabValue = 'control' | 'info' | 'history'

defineProps<{
  modelValue: TabValue
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', value: TabValue): void
}>()

const tabs: { label: string; value: TabValue }[] = [
  { label: 'Control', value: 'control' },
  { label: 'Info', value: 'info' },
  { label: 'History', value: 'history' },
]
</script>
