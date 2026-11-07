<script setup lang="ts">
import type { StepperTitleProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { StepperTitle as StepperTitlePrimitive, useForwardProps } from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperTitleProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperTitlePrimitive
    data-slot="stepper-title"
    v-bind="forwardedProps"
    :class="
      cn(
        'mt-2 text-center text-xs font-medium leading-tight sm:text-sm',
        props.class,
      )
    "
  >
    <slot />
  </StepperTitlePrimitive>
</template>
