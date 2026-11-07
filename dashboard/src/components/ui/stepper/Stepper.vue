<script setup lang="ts">
import type { StepperRootEmits, StepperRootProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  StepperRoot as StepperRootPrimitive,
  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = defineProps<
  StepperRootProps & { class?: HTMLAttributes['class'] }
>()
const emits = defineEmits<StepperRootEmits>()
const delegatedProps = reactiveOmit(props, 'class')
const forwardedProps = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <StepperRootPrimitive
    data-slot="stepper"
    v-bind="forwardedProps"
    :class="cn('flex w-full items-start gap-2', props.class)"
  >
    <slot />
  </StepperRootPrimitive>
</template>
