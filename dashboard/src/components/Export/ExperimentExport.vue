<template>
  <section class="mx-auto flex w-full max-w-2xl flex-col gap-6 rounded-2xl border bg-card p-6 shadow-sm">
    <div class="grid gap-2">
      <Label for="export-format">Export format</Label>
      <Select v-model="format">
        <SelectTrigger id="export-format">
          <SelectValue placeholder="Select an export format" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="excel">Excel workbook (.xlsx)</SelectItem>
          <SelectItem value="csv">CSV files (.csv)</SelectItem>
        </SelectContent>
      </Select>
      <p class="text-sm text-muted-foreground">
        {{ formatDescription }}
      </p>
    </div>

    <div class="grid gap-3 sm:grid-cols-2">
      <Button :disabled="isExporting" @click="exportAll">
        <FolderOutput />
        Export all experiments
      </Button>
      <Button variant="outline" :disabled="isExporting || !experimentStore.activeExperiment" @click="exportActive">
        <Download />
        Export active experiment
      </Button>
    </div>

    <p v-if="resultMessage"
      class="rounded-lg border border-green-500/30 bg-green-500/10 px-3 py-2 text-sm text-green-700 dark:text-green-400">
      {{ resultMessage }}
    </p>
    <p v-if="errorMessage"
      class="rounded-lg border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
      {{ errorMessage }}
    </p>
  </section>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { Download, FolderOutput } from 'lucide-vue-next'
import { computed, ref } from 'vue'
import { toast } from 'vue-sonner'
import { commands } from '@/bindings'
import { useExperimentStore } from '@/stores/experiments'

const experimentStore = useExperimentStore()
const isExporting = ref(false)
const resultMessage = ref('')
const errorMessage = ref('')
const format = ref<'excel' | 'csv'>('excel')
const formatDescription = computed(() =>
  format.value === 'excel'
    ? 'One workbook per experiment with Environment and Well Readings tables on one sheet.'
    : 'Two CSV files per experiment: Environment and Well Readings.',
)

async function chooseFolder(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Choose export folder',
  })

  return typeof selected === 'string' ? selected : null
}

async function exportExperiments(experimentIds: number[] | null) {
  resultMessage.value = ''
  errorMessage.value = ''

  const folder = await chooseFolder()
  if (!folder) {
    return
  }

  isExporting.value = true
  try {
    const result =
      format.value === 'excel'
        ? await commands.exportExcelExperiments(folder, experimentIds)
        : await commands.exportCsvExperiments(folder, experimentIds)
    if (result.status === 'error') {
      throw new Error(result.error)
    }
    const kind = format.value === 'excel' ? 'workbook' : 'CSV file'
    resultMessage.value = `Exported ${result.data.experimentsExported} experiment${result.data.experimentsExported === 1 ? '' : 's'} to ${result.data.filesWritten} ${kind}${result.data.filesWritten === 1 ? '' : 's'}.`
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Export failed'
    toast.error('Export failed', { description: errorMessage.value })
  } finally {
    isExporting.value = false
  }
}

function exportAll() {
  return exportExperiments(null)
}

function exportActive() {
  const id = experimentStore.activeExperiment?.id
  if (id !== null && id !== undefined) {
    return exportExperiments([id])
  }
}
</script>
