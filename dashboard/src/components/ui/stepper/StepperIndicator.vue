<script setup lang="ts">
import type { StepperIndicatorProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  StepperIndicator as StepperIndicatorPrimitive,
  useForwardProps,
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperIndicatorProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperIndicatorPrimitive
    data-slot="stepper-indicator"
    v-bind="forwardedProps"
    :class="
      cn(
        'flex size-8 items-center justify-center rounded-full border text-sm font-medium transition-colors',
        props.class,
      )
    "
  >
    <slot />
  </StepperIndicatorPrimitive>
</template>
