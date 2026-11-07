<script setup lang="ts">
import type { StepperDescriptionProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  StepperDescription as StepperDescriptionPrimitive,
  useForwardProps,
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperDescriptionProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperDescriptionPrimitive
    data-slot="stepper-description"
    v-bind="forwardedProps"
    :class="cn('mt-1 text-center text-xs text-muted-foreground', props.class)"
  >
    <slot />
  </StepperDescriptionPrimitive>
</template>
