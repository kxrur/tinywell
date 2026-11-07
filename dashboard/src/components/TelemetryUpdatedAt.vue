<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger as-child>
        <button
          type="button"
          class="text-muted-foreground transition-colors hover:text-foreground"
          :aria-label="tooltipText"
        >
          <Info class="size-4" />
        </button>
      </TooltipTrigger>
      <TooltipContent>{{ tooltipText }}</TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Info } from 'lucide-vue-next'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

const props = defineProps<{
  receivedAt: number | null
  source: string
}>()

const tooltipText = computed(() =>
  props.receivedAt === null
    ? `No ${props.source.toLowerCase()} telemetry received yet`
    : `${props.source} telemetry last received at ${new Date(props.receivedAt).toLocaleTimeString()}`,
)
</script>
