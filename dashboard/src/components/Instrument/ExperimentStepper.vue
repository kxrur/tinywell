<template>
  <Stepper
    :model-value="activeStep"
    :linear="false"
    aria-label="Experiment phase progress"
    class="w-full"
  >
    <StepperItem
      v-for="(phase, index) in phases"
      :key="phase.state"
      v-slot="{ state }"
      :step="phase.state"
      :completed="isCompleted(phase.state)"
      disabled
      class="relative"
    >
      <StepperSeparator
        v-if="index < phases.length - 1"
        :class="isCompleted(phase.state) ? 'bg-primary' : 'bg-muted'"
      />
      <StepperTrigger class="cursor-default">
        <StepperIndicator
          :class="[
            state === 'completed' &&
              'border-primary bg-primary text-primary-foreground',
            state === 'active' &&
              'border-primary bg-background text-primary ring-2 ring-primary/20',
            state === 'inactive' &&
              'border-muted-foreground/30 bg-background text-muted-foreground',
          ]"
        >
          <Check v-if="state === 'completed'" class="size-4" />
          <span v-else>{{ phase.state }}</span>
        </StepperIndicator>
      </StepperTrigger>
      <StepperTitle
        :class="[
          state === 'active' && 'text-primary',
          state === 'inactive' && 'text-muted-foreground',
        ]"
      >
        {{ phase.title }}
      </StepperTitle>
    </StepperItem>
  </Stepper>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Check } from 'lucide-vue-next'
import {
  Stepper,
  StepperIndicator,
  StepperItem,
  StepperSeparator,
  StepperTitle,
  StepperTrigger,
} from '@/components/ui/stepper'

const props = defineProps<{
  currentState: number
  lastState: number
  progress: number
}>()

const phases = [
  { state: 1, title: 'Preparation' },
  { state: 2, title: 'Activation' },
  { state: 3, title: 'Growth' },
  { state: 4, title: 'Induction' },
]

const activeStep = computed(() =>
  props.currentState >= 1 && props.currentState <= phases.length
    ? props.currentState
    : 0,
)

function isCompleted(phase: number): boolean {
  if (props.currentState > 0) {
    return phase < props.currentState
  }
  return (
    phase < props.lastState ||
    (phase === props.lastState && props.progress === 1)
  )
}
</script>
