import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { commands, type Experiment } from '@/bindings'
import { useSerialStore } from '@/stores/serial'

function defaultExperimentName() {
  return `Exp. ${new Date().toLocaleString()}`
}

export const useExperimentStore = defineStore('experiments', () => {
  const serialStore = useSerialStore()
  const experiments = ref<Experiment[]>([])
  const activeExperimentId = ref<number | null>(null)
  const isLoading = ref(false)
  const error = ref('')

  const activeExperiment = computed(
    () =>
      experiments.value.find(experiment => experiment.id === activeExperimentId.value) ??
      null,
  )

  async function refresh() {
    const result = await commands.experimentList()
    if (result.status === 'error') {
      throw new Error(result.error)
    }
    experiments.value = result.data
  }

  async function select(experimentId: number | null) {
    if (experimentId === null) {
      throw new Error('Experiment has no ID')
    }
    if (
      experimentId !== activeExperimentId.value &&
      serialStore.isConnected
    ) {
      await serialStore.disconnect()
    }

    const result = await commands.experimentSetActive(experimentId)
    if (result.status === 'error') {
      throw new Error(result.error)
    }
    activeExperimentId.value = experimentId
  }

  async function createExperiment(name: string) {
    const result = await commands.experimentCreate(name)
    if (result.status === 'error') {
      throw new Error(result.error)
    }
    await refresh()
    await select(result.data.id)
  }

  async function initialize() {
    if (isLoading.value) {
      return
    }

    isLoading.value = true
    error.value = ''
    try {
      // Each app launch starts a distinct recording session.
      await createExperiment(defaultExperimentName())
    } catch (cause) {
      error.value =
        cause instanceof Error ? cause.message : 'Failed to create experiment'
    } finally {
      isLoading.value = false
    }
  }

  return {
    activeExperiment,
    createExperiment,
    error,
    experiments,
    initialize,
    isLoading,
    select,
  }
})
