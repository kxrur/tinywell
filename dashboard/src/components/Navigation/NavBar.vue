<template>
  <div>
    <Sidebar>
      <SidebarHeader class="border-b border-border">
        <div class="flex items-center gap-2 px-4 py-3">
          <div
            class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center mr-6"
          >
            <Beaker class="w-5 h-5 text-primary-foreground" />
          </div>
          <h1 class="text-xl font-bold text-foreground">TinyWell</h1>
        </div>
      </SidebarHeader>

      <SidebarContent class="px-2 py-4">
        <div
          class="px-2 pb-2 text-xs font-medium uppercase tracking-wide text-muted-foreground"
        >
          Experiments
        </div>
        <div class="space-y-1">
          <div
            v-for="experiment in experimentStore.experiments"
            :key="experiment.id ?? experiment.name"
          >
            <SidebarMenuButton
              class="w-full justify-start py-2 hover:bg-accent hover:text-accent-foreground"
              :is-active="
                experiment.id === experimentStore.activeExperiment?.id
              "
              @click="experimentStore.select(experiment.id).catch(showError)"
            >
              <span class="truncate font-medium">{{ experiment.name }}</span>
            </SidebarMenuButton>
          </div>
          <p
            v-if="experimentStore.isLoading"
            class="px-2 py-2 text-sm text-muted-foreground"
          >
            Creating experiment…
          </p>
          <p v-if="errorMessage" class="px-2 py-2 text-sm text-destructive">
            {{ errorMessage }}
          </p>
        </div>
      </SidebarContent>

      <SidebarFooter class="p-4">
        <SerialDeviceConnect />
        <Separator class="my-4" />
        <Button
          class="w-full justify-center gap-2"
          variant="default"
          @click="openCreateDialog"
        >
          <Plus class="w-4 h-4" />
          <span>New Experiment</span>
        </Button>
      </SidebarFooter>
    </Sidebar>

    <Dialog v-model:open="isCreateDialogOpen">
      <DialogContent>
        <form class="grid gap-4" @submit.prevent="createExperiment">
          <DialogHeader>
            <DialogTitle>New Experiment</DialogTitle>
            <DialogDescription>
              Name the experiment that will receive recorded telemetry.
            </DialogDescription>
          </DialogHeader>

          <div class="grid gap-2">
            <Label for="experiment-name">Experiment name</Label>
            <Input
              id="experiment-name"
              v-model="newExperimentName"
              autofocus
              :disabled="isCreating"
              placeholder="e.g. Growth assay A"
            />
            <p v-if="dialogError" class="text-sm text-destructive">
              {{ dialogError }}
            </p>
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              :disabled="isCreating"
              @click="isCreateDialogOpen = false"
            >
              Cancel
            </Button>
            <Button type="submit" :disabled="isCreating">
              {{ isCreating ? 'Creating…' : 'Create experiment' }}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Plus, Beaker } from 'lucide-vue-next'
import { useExperimentStore } from '@/stores/experiments'

const experimentStore = useExperimentStore()
const errorMessage = ref('')
const isCreateDialogOpen = ref(false)
const isCreating = ref(false)
const newExperimentName = ref('')
const dialogError = ref('')

function showError(error: unknown) {
  errorMessage.value =
    error instanceof Error ? error.message : 'Experiment action failed'
}

function openCreateDialog() {
  dialogError.value = ''
  newExperimentName.value = ''
  isCreateDialogOpen.value = true
}

async function createExperiment() {
  const name = newExperimentName.value.trim()
  if (!name) {
    dialogError.value = 'Enter an experiment name'
    return
  }

  isCreating.value = true
  dialogError.value = ''
  try {
    await experimentStore.createExperiment(name)
    isCreateDialogOpen.value = false
  } catch (error) {
    dialogError.value =
      error instanceof Error ? error.message : 'Failed to create experiment'
  } finally {
    isCreating.value = false
  }
}

onMounted(() => {
  experimentStore.initialize().catch(showError)
})
</script>
