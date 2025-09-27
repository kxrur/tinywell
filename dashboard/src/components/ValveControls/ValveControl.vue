<template>
    <div class="flex flex-col items-center space-y-0 p-1 bg-background">
        <Label class="text-lg font-semibold">Valve</Label>

        <div class="relative">
            <svg width="120" height="120" viewBox="0 0 120 120" class="text-blue-500">
                <rect x="10" y="55" width="100" height="10" rx="5" fill="currentColor" />

                <circle cx="60" cy="60" r="25" fill="currentColor" stroke="currentColor" stroke-width="2" />

                <circle cx="52" cy="55" r="2.5" fill="white" />
                <circle cx="68" cy="55" r="2.5" fill="white" />

                <path v-if="isOpen" d="M 52 66 Q 60 70 68 66" stroke="white" stroke-width="2" fill="none"
                    stroke-linecap="round" />
                <line v-else x1="55" y1="66" x2="65" y2="66" stroke="white" stroke-width="2" stroke-linecap="round" />

                <rect x="57" y="35" width="6" height="15" rx="3" fill="currentColor" />

                <g v-if="isOpen">
                    <path d="M20 60 Q30 58 40 60 Q50 62 60 60 Q70 58 80 60 Q90 62 100 60" stroke="white"
                        stroke-width="1" fill="none" opacity="0.5" />
                </g>

            </svg>
        </div>

        <div class="text-sm font-medium" :class="isOpen ? 'text-green-600' : 'text-gray-500'">
            {{ isOpen ? 'Open' : 'Closed' }}
        </div>

        <div class="flex items-center space-x-2">
            <Label class="text-sm">Off</Label>

            <Switch :model-value="isOpen" @update:model-value="() => isOpen = !isOpen" />
            <Label class="text-sm">On</Label>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'

const isOpen = ref(false)
</script>