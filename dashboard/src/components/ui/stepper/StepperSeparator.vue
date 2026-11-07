<script setup lang="ts">
import type { StepperSeparatorProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  StepperSeparator as StepperSeparatorPrimitive,
  useForwardProps,
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperSeparatorProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperSeparatorPrimitive
    data-slot="stepper-separator"
    v-bind="forwardedProps"
    :class="
      cn(
        'absolute left-[calc(50%+1rem)] right-[calc(-50%+1rem)] top-4 h-0.5 shrink-0 rounded-full bg-muted group-data-[state=completed]:bg-primary',
        props.class,
      )
    "
  >
    <slot />
  </StepperSeparatorPrimitive>
</template>
