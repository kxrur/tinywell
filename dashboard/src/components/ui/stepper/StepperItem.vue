<script setup lang="ts">
import type { StepperItemProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { StepperItem as StepperItemPrimitive, useForwardProps } from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperItemProps & { class?: HTMLAttributes['class'] }
>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <StepperItemPrimitive
    data-slot="stepper-item"
    v-bind="forwardedProps"
    :class="
      cn(
        'group relative flex min-w-0 flex-1 flex-col items-center',
        props.class,
      )
    "
    v-slot="slotProps"
  >
    <slot v-bind="slotProps" />
  </StepperItemPrimitive>
</template>
