import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { commands, type ConnectionStatus, type Result } from '@/bindings'

const DEFAULT_SERIAL_PORT = '/dev/ttyUSB0'

function unwrapResult<T>(
  result: Result<T, string>,
  fallbackMessage: string,
): T {
  if (result.status === 'error') {
    throw new Error(result.error ?? fallbackMessage)
  }

  return result.data
}

function statusIsConnected(status: ConnectionStatus): boolean {
  return typeof status === 'object' && status !== null && 'Connected' in status
}

function isConnecting(status: ConnectionStatus): boolean {
  return status === 'Connecting'
}

export const useSerialStore = defineStore('serial', () => {
  const status = ref<ConnectionStatus>('Disconnected')
  const configuredPort = ref(DEFAULT_SERIAL_PORT)
  const connectPromise = ref<Promise<void> | null>(null)

  const connectedPort = computed(() =>
    typeof status.value === 'object' &&
    status.value !== null &&
    'Connected' in status.value
      ? status.value.Connected.port
      : null,
  )
  const isConnected = computed(() => connectedPort.value !== null)

  async function refreshStatus(): Promise<ConnectionStatus> {
    status.value = unwrapResult(
      await commands.serialStatus(),
      'Failed to read serial connection status',
    )

    return status.value
  }

  async function setPort(port: string): Promise<void> {
    configuredPort.value = port
    unwrapResult(
      await commands.serialSetPort(port),
      `Failed to configure serial port ${port}`,
    )
  }

  async function waitForConnection(): Promise<void> {
    for (let attempt = 0; attempt < 10; attempt += 1) {
      const currentStatus = await refreshStatus()
      if (statusIsConnected(currentStatus)) {
        return
      }

      await new Promise(resolve => window.setTimeout(resolve, 150))
    }

    throw new Error('Timed out waiting for serial connection')
  }

  async function ensureConnected(port = configuredPort.value): Promise<void> {
    if (connectPromise.value) {
      return connectPromise.value
    }

    connectPromise.value = (async () => {
      const currentStatus = await refreshStatus()

      if (statusIsConnected(currentStatus)) {
        return
      }

      if (isConnecting(currentStatus)) {
        await waitForConnection()
        return
      }

      await setPort(port)
      unwrapResult(
        await commands.serialConnect(),
        'Failed to connect to the serial device',
      )
      await refreshStatus()
    })()

    try {
      await connectPromise.value
    } finally {
      connectPromise.value = null
    }
  }

  async function disconnect(): Promise<void> {
    unwrapResult(
      await commands.serialDisconnect(),
      'Failed to disconnect from the serial device',
    )
    await refreshStatus()
  }

  function unwrapCommandResult<T>(
    result: Result<T, string>,
    fallbackMessage: string,
  ): T {
    return unwrapResult(result, fallbackMessage)
  }

  return {
    status,
    configuredPort,
    connectedPort,
    isConnected,
    disconnect,
    ensureConnected,
    refreshStatus,
    setPort,
    unwrapCommandResult,
  }
})
