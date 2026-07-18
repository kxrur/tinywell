<template>
  <div class="space-y-3">
    <div class="space-y-2">
      <Label
        class="text-xs font-medium uppercase tracking-wide text-muted-foreground"
      >
        Serial Device
      </Label>

      <Select
        v-model="selectedPort"
        :disabled="isLoadingPorts || availablePorts.length === 0"
      >
        <SelectTrigger class="w-full">
          <SelectValue
            :placeholder="
              isLoadingPorts
                ? 'Loading ports...'
                : availablePorts.length === 0
                  ? 'No ports available'
                  : 'Select a port'
            "
          />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="port in availablePorts" :key="port" :value="port">
            {{ port }}
          </SelectItem>
        </SelectContent>
      </Select>
    </div>

    <Button
      class="w-full"
      :disabled="
        isLoadingPorts ||
        isConnecting ||
        (!serialStore.isConnected && availablePorts.length === 0)
      "
      @click="toggleConnection"
      variant="outline"
    >
      {{ isConnecting ? 'Connecting...' : serialStore.isConnected ? 'Disconnect' : 'Connect' }}
    </Button>

    <p v-if="loadError" class="text-xs text-destructive">
      {{ loadError }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { commands } from '@/bindings'
import { toast } from 'vue-sonner'
import { useSerialStore } from '@/stores/serial'

const serialStore = useSerialStore()
const availablePorts = ref<string[]>([])
const selectedPort = ref(serialStore.configuredPort)
const isLoadingPorts = ref(false)
const loadError = ref('')

const isConnecting = computed(() => serialStore.status === 'Connecting')

async function loadPorts() {
  isLoadingPorts.value = true
  loadError.value = ''

  try {
    await serialStore.refreshStatus()

    const result = await commands.serialListPorts()
    const ports = serialStore.unwrapCommandResult(
      result,
      'Failed to load available serial ports',
    )

    availablePorts.value = ports

    if (ports.length === 0) {
      return
    }

    const preferredPort =
      serialStore.connectedPort ?? serialStore.configuredPort

    if (preferredPort && ports.includes(preferredPort)) {
      selectedPort.value = preferredPort
    } else if (!ports.includes(selectedPort.value)) {
      selectedPort.value = ports[0]
    }

    await serialStore.setPort(selectedPort.value)
  } catch (error) {
    loadError.value =
      error instanceof Error ? error.message : 'Failed to load serial ports'
    toast.error('Failed to load serial ports', { description: loadError.value })
  } finally {
    isLoadingPorts.value = false
  }
}

async function connectToSelectedPort() {
  if (!selectedPort.value) {
    return
  }

  if (
    serialStore.connectedPort &&
    serialStore.connectedPort !== selectedPort.value
  ) {
    await serialStore.disconnect()
  }

  await serialStore.setPort(selectedPort.value)
  await serialStore.ensureConnected(selectedPort.value)
}

async function toggleConnection() {
  loadError.value = ''
  try {
    if (serialStore.isConnected) {
      await serialStore.disconnect()
      return
    }
    await connectToSelectedPort()
  } catch (error) {
    loadError.value =
      error instanceof Error ? error.message : 'Failed to update serial connection'
    toast.error('Serial connection failed', { description: loadError.value })
  }
}

watch(selectedPort, port => {
  if (!port) {
    return
  }

  serialStore.setPort(port).catch(error => {
    loadError.value =
      error instanceof Error ? error.message : 'Failed to set serial port'
    toast.error('Failed to set serial port', { description: loadError.value })
  })
})

onMounted(() => {
  void loadPorts()
})
</script>
