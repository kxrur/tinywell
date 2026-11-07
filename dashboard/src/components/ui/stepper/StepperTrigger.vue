<script setup lang="ts">
import type { StepperTriggerProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  StepperTrigger as StepperTriggerPrimitive,
  useForwardProps,
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperTriggerProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperTriggerPrimitive
    data-slot="stepper-trigger"
    v-bind="forwardedProps"
    :class="
      cn(
        'z-10 rounded-full outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
        props.class,
      )
    "
  >
    <slot />
  </StepperTriggerPrimitive>
</template>
