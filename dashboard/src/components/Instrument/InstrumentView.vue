<template>
  <div class="h-full min-h-0 overflow-y-auto pr-1">
    <div
      class="mb-4 flex flex-wrap items-center justify-between gap-3 rounded-xl border bg-card px-4 py-3"
    >
      <div>
        <h2 class="text-lg font-semibold">Instrument</h2>
        <p class="text-sm text-muted-foreground">
          Operate experiment phases and LED channels while monitoring live
          telemetry.
        </p>
      </div>
      <span
        class="rounded-full px-3 py-1 text-xs font-medium"
        :class="
          serialStore.isConnected
            ? 'bg-green-500/10 text-green-700 dark:text-green-400'
            : 'bg-muted text-muted-foreground'
        "
      >
        {{
          serialStore.isConnected
            ? `Connected: ${serialStore.connectedPort}`
            : 'Not connected'
        }}
      </span>
    </div>

    <div
      v-if="!serialStore.isConnected"
      class="mb-4 rounded-xl border border-dashed p-4 text-sm text-muted-foreground"
    >
      Connect a serial device from the sidebar to control the instrument and
      view live telemetry.
    </div>

    <div
      v-if="instrumentStore.lastCommandMessage"
      class="mb-4 rounded-xl border px-4 py-3 text-sm"
      :class="
        instrumentStore.lastCommandSucceeded
          ? 'border-green-500/40 bg-green-500/10 text-green-800 dark:text-green-300'
          : 'border-destructive/40 bg-destructive/10 text-destructive'
      "
      role="status"
      aria-live="polite"
    >
      {{ instrumentStore.lastCommandMessage }}
    </div>

    <div
      v-if="instrumentStore.streamError"
      class="mb-4 rounded-xl border border-amber-500/40 bg-amber-500/10 px-4 py-3 text-sm text-amber-800 dark:text-amber-300"
      role="status"
    >
      Telemetry warning: {{ instrumentStore.streamError }}
    </div>

    <div
      class="grid grid-cols-1 items-start gap-4 xl:grid-cols-[minmax(19rem,2fr)_minmax(0,3fr)]"
    >
      <section class="space-y-4">
        <div class="rounded-xl border bg-card p-4">
          <div class="mb-3 flex items-start justify-between gap-3">
            <div>
              <h3 class="font-semibold">Experiment controls</h3>
              <p class="text-xs text-muted-foreground">
                Pump and valve actions are sequenced by the payload’s experiment
                phases.
              </p>
            </div>
            <span class="rounded-full bg-muted px-2.5 py-1 text-xs font-medium">
              {{ stateLabel(systemStatus?.state) }}
            </span>
          </div>

          <div class="grid gap-3">
            <button
              class="rounded-lg bg-primary px-3 py-2 text-sm font-medium text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="
                !serialStore.isConnected ||
                isCommandPending ||
                systemStatus?.state !== 0 ||
                !experimentStatus ||
                (experimentStatus?.lastState ?? 0) >= 4
              "
              @click="
                runCommand(instrumentStore.nextExperimentPhase, 'Next phase')
              "
            >
              {{ commandPending === 'Next phase' ? 'Sending…' : 'Next phase' }}
            </button>

            <div class="grid grid-cols-[minmax(0,1fr)_auto] gap-2">
              <label class="sr-only" for="experiment-phase"
                >Experiment phase</label
              >
              <select
                id="experiment-phase"
                v-model.number="selectedPhase"
                class="min-w-0 rounded-lg border bg-background px-3 py-2 text-sm"
                :disabled="!serialStore.isConnected || isCommandPending"
              >
                <option :value="1">Preparation</option>
                <option :value="2">Activation</option>
                <option :value="3">Growth</option>
                <option :value="4">Induction</option>
              </select>
              <button
                class="rounded-lg border px-3 py-2 text-sm font-medium hover:bg-accent disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="
                  !serialStore.isConnected ||
                  isCommandPending ||
                  systemStatus?.state !== 0
                "
                @click="
                  runCommand(
                    () => instrumentStore.runExperimentPhase(selectedPhase),
                    'Run phase',
                  )
                "
              >
                Run phase
              </button>
            </div>

            <button
              class="rounded-lg border border-destructive/50 px-3 py-2 text-sm font-semibold text-destructive hover:bg-destructive/10 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="!serialStore.isConnected || isCommandPending"
              @click="
                runCommand(instrumentStore.setStateIdle, 'Return to idle')
              "
            >
              {{
                commandPending === 'Return to idle'
                  ? 'Stopping…'
                  : 'Return to idle'
              }}
            </button>
          </div>

          <p class="mt-3 text-xs text-muted-foreground">
            Phase status: {{ phaseLabel(experimentStatus?.lastState) }} ·
            {{ progressLabel(experimentStatus?.progress) }}
          </p>
        </div>

        <div class="rounded-xl border bg-card p-4">
          <div class="mb-3">
            <h3 class="font-semibold">LED channels</h3>
            <p class="text-xs text-muted-foreground">
              Each wavelength channel illuminates all wells. Brightness is
              0–255.
            </p>
          </div>
          <div class="space-y-4">
            <div
              v-for="channel in ledChannels"
              :key="channel.wavelength"
              class="rounded-lg border p-3"
            >
              <div class="mb-2 flex items-center justify-between gap-2">
                <span class="text-sm font-medium"
                  >{{ channel.nanometers }} nm</span
                >
                <div class="flex gap-2">
                  <button
                    class="rounded-md border px-2.5 py-1 text-xs hover:bg-accent disabled:opacity-50"
                    :disabled="!serialStore.isConnected || isCommandPending"
                    @click="
                      runCommand(
                        () =>
                          instrumentStore.setLedState(
                            channel.wavelength,
                            'On',
                            brightness[channel.wavelength],
                          ),
                        `${channel.nanometers} nm LED on`,
                      )
                    "
                  >
                    On
                  </button>
                  <button
                    class="rounded-md border px-2.5 py-1 text-xs hover:bg-accent disabled:opacity-50"
                    :disabled="!serialStore.isConnected || isCommandPending"
                    @click="
                      runCommand(
                        () =>
                          instrumentStore.setLedState(
                            channel.wavelength,
                            'Off',
                            brightness[channel.wavelength],
                          ),
                        `${channel.nanometers} nm LED off`,
                      )
                    "
                  >
                    Off
                  </button>
                  <button
                    class="rounded-md border px-2.5 py-1 text-xs hover:bg-accent disabled:opacity-50"
                    :disabled="!serialStore.isConnected || isCommandPending"
                    @click="
                      runCommand(
                        () =>
                          instrumentStore.setLedState(
                            channel.wavelength,
                            'Toggle',
                            brightness[channel.wavelength],
                          ),
                        `${channel.nanometers} nm LED toggle`,
                      )
                    "
                  >
                    Toggle
                  </button>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <input
                  v-model.number="brightness[channel.wavelength]"
                  type="range"
                  min="0"
                  max="255"
                  step="1"
                  class="min-w-0 flex-1 accent-primary"
                  :aria-label="`${channel.nanometers} nm LED brightness`"
                  :disabled="!serialStore.isConnected || isCommandPending"
                />
                <span class="w-8 text-right font-mono text-xs">{{
                  brightness[channel.wavelength]
                }}</span>
                <button
                  class="rounded-md bg-secondary px-2.5 py-1 text-xs font-medium hover:bg-secondary/80 disabled:opacity-50"
                  :disabled="!serialStore.isConnected || isCommandPending"
                  @click="
                    runCommand(
                      () =>
                        instrumentStore.setLedState(
                          channel.wavelength,
                          'SetBrightness',
                          brightness[channel.wavelength],
                        ),
                      `${channel.nanometers} nm brightness`,
                    )
                  "
                >
                  Apply
                </button>
              </div>
            </div>
          </div>
        </div>

        <details class="rounded-xl border bg-card p-4">
          <summary class="cursor-pointer font-semibold">
            Advanced commands
          </summary>
          <p class="mt-2 text-xs text-muted-foreground">
            Current mroc accepts health-test and reset as no-ops. The Nano
            simulator resets its simulated state for testing.
          </p>
          <div class="mt-3 grid gap-2 sm:grid-cols-2">
            <button
              class="rounded-lg border px-3 py-2 text-sm hover:bg-accent disabled:opacity-50"
              :disabled="!serialStore.isConnected || isCommandPending"
              @click="
                runCommand(
                  instrumentStore.testSystemHealth,
                  'Test system health',
                )
              "
            >
              Test system health
            </button>
            <button
              class="rounded-lg border border-destructive/50 px-3 py-2 text-sm text-destructive hover:bg-destructive/10 disabled:opacity-50"
              :disabled="!serialStore.isConnected || isCommandPending"
              @click="confirmReset"
            >
              Reset payload
            </button>
          </div>
        </details>
      </section>

      <section class="space-y-4">
        <div class="rounded-xl border bg-card p-4">
          <div class="mb-4 flex flex-wrap items-start justify-between gap-2">
            <div>
              <h3 class="font-semibold">Experiment progress</h3>
              <p class="text-xs text-muted-foreground">
                Preparation through induction
              </p>
            </div>
            <span
              class="rounded-full px-2.5 py-1 text-xs font-medium"
              :class="
                telemetryStale
                  ? 'bg-amber-500/10 text-amber-700 dark:text-amber-300'
                  : 'bg-green-500/10 text-green-700 dark:text-green-400'
              "
              >{{ telemetryStale ? 'Waiting / stale' : 'Live' }}</span
            >
            <TelemetryUpdatedAt
              :received-at="lastReceivedAt"
              source="System and experiment status"
            />
          </div>
          <ExperimentStepper
            :current-state="systemStatus?.state ?? 0"
            :last-state="experimentStatus?.lastState ?? 0"
            :progress="experimentStatus?.progress ?? 0"
          />

          <details class="mt-5 border-t pt-3">
            <summary class="cursor-pointer text-sm font-medium">
              System and experiment details
            </summary>
            <div class="mt-3 grid grid-cols-2 gap-3 sm:grid-cols-3">
              <MetricCard
                label="Firmware"
                :value="systemStatus ? `v${systemStatus.firmwareVersion}` : '—'"
              />
              <MetricCard
                label="Uptime"
                :value="
                  systemStatus
                    ? `${systemStatus.uptimeSeconds.toLocaleString()} s`
                    : '—'
                "
              />
              <MetricCard
                label="Last phase"
                :value="phaseLabel(experimentStatus?.lastState)"
              />
              <MetricCard
                label="Progress"
                :value="progressLabel(experimentStatus?.progress)"
              />
              <MetricCard label="Phase timing" :value="phaseTimingLabel" />
            </div>
          </details>
        </div>

        <SensorsMonitor />

        <div class="rounded-xl border bg-card p-3">
          <div class="px-1 pt-1">
            <div class="flex items-center gap-2">
              <h3 class="font-semibold">Photosensors</h3>
              <TelemetryUpdatedAt
                :received-at="historyStore.latestSensorReceivedAt"
                source="Photosensor"
              />
            </div>
            <p class="text-xs text-muted-foreground">
              Live relative visualization; expand the readings below it for raw
              lux values. Latest wavelength:
              {{ historyStore.sensorWavelengthNm ?? '—' }} nm.
            </p>
          </div>
          <SensorCanvas v-if="historyStore.latestSensorFrame" />
          <p v-else class="p-4 text-sm text-muted-foreground">
            Waiting for photosensor telemetry.
          </p>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { toast } from 'vue-sonner'
import type { LedAction } from '@/bindings'
import ExperimentStepper from '@/components/Instrument/ExperimentStepper.vue'
import MetricCard from '@/components/Instrument/MetricCard.vue'
import TelemetryUpdatedAt from '@/components/TelemetryUpdatedAt.vue'
import { useHistoryStore } from '@/stores/history'
import { useInstrumentStore } from '@/stores/instrument'
import { useSerialStore } from '@/stores/serial'

const serialStore = useSerialStore()
const historyStore = useHistoryStore()
const instrumentStore = useInstrumentStore()

const selectedPhase = ref(1)
const brightness = ref<Record<number, number>>({
  0: 255,
  1: 255,
  2: 255,
  3: 255,
})
const now = ref(Date.now())
let freshnessTimer: number | undefined

const ledChannels = [
  { wavelength: 0, nanometers: 470 },
  { wavelength: 1, nanometers: 570 },
  { wavelength: 2, nanometers: 630 },
  { wavelength: 3, nanometers: 850 },
]

const frame = computed(() => instrumentStore.latestFrame)
const systemStatus = computed(() => frame.value?.systemStatus ?? null)
const experimentStatus = computed(() => frame.value?.experimentStatus ?? null)
const lastReceivedAt = computed(() => instrumentStore.lastReceivedAt)
const commandPending = computed(() => instrumentStore.pendingCommand)
const isCommandPending = computed(() => commandPending.value.length > 0)
const telemetryStale = computed(
  () =>
    lastReceivedAt.value === null || now.value - lastReceivedAt.value > 5000,
)
const phaseTimingLabel = computed(() => {
  if (!experimentStatus.value) return '—'
  const value = experimentStatus.value.elapsedOrStartedAtUptime
  return systemStatus.value?.state && systemStatus.value.state !== 0
    ? `Started ${value.toLocaleString()} s`
    : `Elapsed ${value.toLocaleString()} s`
})

function stateLabel(state: number | undefined): string {
  if (state === undefined) return '—'
  return (
    ['Idle', 'Preparation', 'Activation', 'Growth', 'Induction'][state] ??
    `Unknown (${state})`
  )
}

function phaseLabel(phase: number | undefined): string {
  return stateLabel(phase)
}

function progressLabel(progress: number | undefined): string {
  if (progress === 0) return 'Incomplete'
  if (progress === 1) return 'Complete'
  return progress === undefined ? '—' : `Unknown (${progress})`
}

async function runCommand(action: () => Promise<void>, label: string) {
  try {
    await action()
    toast.success(instrumentStore.lastCommandMessage || `${label} accepted`)
  } catch (error) {
    const message = error instanceof Error ? error.message : `${label} failed`
    toast.error(label, { description: message })
  }
}

function confirmReset() {
  if (window.confirm('Send the reset command to the connected device?')) {
    void runCommand(instrumentStore.resetPayload, 'Reset payload')
  }
}

onMounted(() => {
  freshnessTimer = window.setInterval(() => {
    now.value = Date.now()
  }, 1000)
})

onUnmounted(() => {
  if (freshnessTimer !== undefined) {
    window.clearInterval(freshnessTimer)
  }
})
</script>
