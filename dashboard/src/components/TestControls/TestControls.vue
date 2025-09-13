<template>
    <div class="flex flex-col space-y-4 p-4 bg-background border rounded-lg">
        <Label class="text-lg font-semibold">Test Controls</Label>

        <div class="space-y-2">
            <Label class="text-sm font-medium">Port</Label>
            <Select v-model="selectedPort">
                <SelectTrigger class="w-full">
                    <SelectValue placeholder="Select a port" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="COM1">COM1</SelectItem>
                    <SelectItem value="COM2">COM2</SelectItem>
                    <SelectItem value="COM3">COM3</SelectItem>
                    <SelectItem value="USB1">/dev/ttyUSB0</SelectItem>
                    <SelectItem value="USB2">/dev/ttyUSB1</SelectItem>
                    <SelectItem value="ACM0">/dev/ttyACM0</SelectItem>
                </SelectContent>
            </Select>
        </div>

        <div class="space-y-2">
            <Label class="text-sm font-medium">Duration (seconds)</Label>
            <Input type="number" v-model.number="duration" placeholder="Enter duration in seconds" min="1" max="3600"
                class="w-full" />
        </div>

        <div class="flex items-center justify-between text-sm">
            <span class="text-muted-foreground">Status:</span>
            <Badge :variant="isRunning ? 'default' : 'secondary'">
                {{ isRunning ? 'Running' : 'Idle' }}
            </Badge>
        </div>

        <div v-if="isRunning" class="space-y-2">
            <div class="flex justify-between text-xs text-muted-foreground">
                <span>Progress</span>
                <span>{{ Math.round((elapsed / duration) * 100) }}%</span>
            </div>
            <!-- FIXME: progress not working -->
            <Progress :value="progress" class="h-2" />
            <div class="text-xs text-muted-foreground text-center">
                {{ elapsed }}s / {{ duration }}s ({{ duration - elapsed }}s remaining)
            </div>
        </div>

        <div class="flex space-x-2">
            <Button @click="startTest" :disabled="!selectedPort || !duration || isRunning" class="flex-1">
                <Play class="w-4 h-4 mr-2" />
                {{ isRunning ? 'Running...' : 'Start Test' }}
            </Button>

            <Button @click="stopTest" variant="destructive" :disabled="!isRunning" class="flex-1">
                <Square class="w-4 h-4 mr-2" />
                Stop
            </Button>
        </div>

    </div>
</template>

<script setup lang="ts">
import { Play, Square } from 'lucide-vue-next'

// State
const selectedPort = ref('')
const duration = ref(10)
const isRunning = ref(false)
const elapsed = ref(0)
const testInterval = ref<NodeJS.Timeout | null>(null)

interface TestResult {
    port: string
    duration: number
    success: boolean
    timestamp: string
}

const lastTestResult = ref<TestResult | null>(null)
const progress = ref<number>()

const startTest = () => {
    if (!selectedPort.value || !duration.value) return

    isRunning.value = true
    elapsed.value = 0
    progress.value = (elapsed.value / duration.value) * 100

    testInterval.value = setInterval(() => {
        elapsed.value++

        if (elapsed.value >= duration.value) {
            completeTest(true)
        }
    }, 1000)
}

const stopTest = () => {
    if (testInterval.value) {
        clearInterval(testInterval.value)
        testInterval.value = null
    }

    if (isRunning.value) {
        completeTest(false)
    }
}

const completeTest = (success: boolean) => {
    isRunning.value = false

    if (testInterval.value) {
        clearInterval(testInterval.value)
        testInterval.value = null
    }

    lastTestResult.value = {
        port: selectedPort.value,
        duration: duration.value,
        success,
        timestamp: new Date().toLocaleTimeString()
    }

    elapsed.value = 0
}

// Cleanup on unmount
onUnmounted(() => {
    if (testInterval.value) {
        clearInterval(testInterval.value)
    }
})
</script>
