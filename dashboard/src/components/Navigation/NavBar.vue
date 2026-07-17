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
            <ContextMenu>
              <ContextMenuTrigger>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger as-child>
                      <SidebarMenuButton
                        class="w-full justify-start py-2 hover:bg-accent hover:text-accent-foreground"
                        :is-active="
                          experiment.id === experimentStore.activeExperiment?.id
                        "
                        @click="
                          experimentStore.select(experiment.id).catch(showError)
                        "
                      >
                        <span class="truncate font-medium">{{
                          experiment.name
                        }}</span>
                      </SidebarMenuButton>
                    </TooltipTrigger>
                    <TooltipContent side="right">{{
                      experiment.name
                    }}</TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </ContextMenuTrigger>
              <ContextMenuContent>
                <ContextMenuItem
                  variant="destructive"
                  @select="openDeleteDialog(experiment)"
                >
                  <Trash2 />
                  Delete experiment
                </ContextMenuItem>
              </ContextMenuContent>
            </ContextMenu>
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

    <Dialog v-model:open="isDeleteDialogOpen">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete experiment?</DialogTitle>
          <DialogDescription>
            This permanently deletes {{ experimentToDelete?.name }} and all of
            its recorded telemetry. This action cannot be undone.
          </DialogDescription>
        </DialogHeader>
        <p v-if="deleteError" class="text-sm text-destructive">
          {{ deleteError }}
        </p>
        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            :disabled="isDeleting"
            @click="isDeleteDialogOpen = false"
          >
            Cancel
          </Button>
          <Button
            type="button"
            variant="destructive"
            :disabled="isDeleting"
            @click="deleteExperiment"
          >
            {{ isDeleting ? 'Deleting…' : 'Delete experiment' }}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Plus, Beaker, Trash2 } from 'lucide-vue-next'
import type { Experiment } from '@/bindings'
import { useExperimentStore } from '@/stores/experiments'

const experimentStore = useExperimentStore()
const errorMessage = ref('')
const isCreateDialogOpen = ref(false)
const isCreating = ref(false)
const newExperimentName = ref('')
const dialogError = ref('')
const isDeleteDialogOpen = ref(false)
const isDeleting = ref(false)
const experimentToDelete = ref<Experiment | null>(null)
const deleteError = ref('')

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

function openDeleteDialog(experiment: Experiment) {
  experimentToDelete.value = experiment
  deleteError.value = ''
  isDeleteDialogOpen.value = true
}

async function deleteExperiment() {
  if (!experimentToDelete.value) {
    return
  }

  isDeleting.value = true
  deleteError.value = ''
  try {
    await experimentStore.deleteExperiment(experimentToDelete.value)
    isDeleteDialogOpen.value = false
    experimentToDelete.value = null
  } catch (error) {
    deleteError.value =
      error instanceof Error ? error.message : 'Failed to delete experiment'
  } finally {
    isDeleting.value = false
  }
}

onMounted(() => {
  experimentStore.initialize().catch(showError)
})
</script>
