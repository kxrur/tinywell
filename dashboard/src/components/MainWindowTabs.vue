<template>
  <Tabs
    :model-value="modelValue"
    class="flex w-full items-center"
    @update:model-value="updateTab"
  >
    <TabsList
      class="grid h-auto w-full max-w-xl grid-cols-4 gap-2 rounded-xl border bg-card p-2 shadow-sm"
    >
      <TabsTrigger
        v-for="tab in tabs"
        :key="tab.value"
        :value="tab.value"
        class="h-auto rounded-lg px-4 py-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground data-[state=active]:bg-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-sm"
      >
        {{ tab.label }}
      </TabsTrigger>
    </TabsList>
  </Tabs>
</template>

<script setup lang="ts">
type TabValue = 'control' | 'info' | 'history' | 'export'

defineProps<{
  modelValue: TabValue
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', value: TabValue): void
}>()

const updateTab = (value: string | number) => {
  if (typeof value === 'string') {
    emit('update:modelValue', value as TabValue)
  }
}

const tabs: { label: string; value: TabValue }[] = [
  { label: 'Control', value: 'control' },
  { label: 'Info', value: 'info' },
  { label: 'History', value: 'history' },
  { label: 'Export', value: 'export' },
]
</script>
