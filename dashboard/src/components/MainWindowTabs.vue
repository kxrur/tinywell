<template>
  <div class="flex justify-center rounded-xl border bg-card p-2 shadow-sm">
    <div class="grid w-full max-w-md grid-cols-2 gap-2">
      <button
        type="button"
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors"
        :class="modelValue === 'control'
          ? 'bg-primary text-primary-foreground shadow-sm'
          : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'"
        @click="emit('update:modelValue', 'control')"
      >
        Control
      </button>
      <button
        type="button"
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors"
        :class="modelValue === 'info'
          ? 'bg-primary text-primary-foreground shadow-sm'
          : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'"
        @click="emit('update:modelValue', 'info')"
      >
        Info
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: 'control' | 'info'
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', value: 'control' | 'info'): void
}>()
</script>
