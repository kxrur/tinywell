<template>
  <div class="min-w-0 rounded-lg border bg-background px-3 py-2">
    <div class="truncate text-xs text-muted-foreground">{{ label }}</div>
    <div class="mt-1 truncate text-sm font-semibold">{{ value }}</div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  value: string
}>()
</script>
