<template>
  <div class="wipe-progress p-8 bg-slate-900 rounded-lg shadow-xl text-white max-w-3xl mx-auto">
    <h2 class="text-3xl font-bold mb-6">Secure Wipe in Progress</h2>
    
    <div class="mb-6 flex items-center justify-between">
      <p class="text-lg font-semibold">Device: {{ selectedDevice.name }}</p>
      <button @click="cancelWipe" class="px-4 py-2 bg-red-600 rounded-lg hover:bg-red-700 transition">Cancel</button>
    </div>
    
    <div class="w-full bg-gray-700 rounded-full overflow-hidden h-8 mb-6">
      <div
        class="bg-blue-600 h-8 transition-all duration-500"
        :style="{ width: progressPercent + '%' }"
        role="progressbar"
        :aria-valuenow="progressPercent"
        aria-valuemin="0"
        aria-valuemax="100"
      ></div>
    </div>

    <p class="mb-2 text-lg font-semibold">{{ progressPercent.toFixed(1) }}% Completed</p>
    <p class="mb-6 text-gray-300">Pass {{ currentPass }} of {{ totalPasses }}</p>
    
    <div class="mb-8 grid grid-cols-2 gap-6">
      <div>
        <p class="text-sm text-gray-400">Bytes Written</p>
        <p class="font-mono">{{ formatBytes(bytesWritten) }} / {{ formatBytes(totalBytes) }}</p>
      </div>
      <div>
        <p class="text-sm text-gray-400">Speed</p>
        <p>{{ currentSpeed.toFixed(2) }} MB/s</p>
      </div>
      <div>
        <p class="text-sm text-gray-400">Estimated Time Remaining</p>
        <p>{{ formatDuration(timeRemaining) }}</p>
      </div>
      <div>
        <p class="text-sm text-gray-400">Pattern</p>
        <p>{{ currentPattern }}</p>
      </div>
    </div>

    <!-- Optional Logs Section -->
    <div v-if="showLogs" class="overflow-y-auto max-h-40 bg-gray-800 p-4 rounded-md font-mono text-xs">
      <div v-for="(log, index) in logs" :key="index" class="mb-1">{{ log }}</div>
    </div>

    <div class="mt-8 flex justify-end space-x-4">
      <button @click="toggleLogs" class="text-sm text-gray-400 hover:text-white">
        {{ showLogs ? 'Hide Logs' : 'Show Logs' }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  selectedDevice: Object,
})

const emit = defineEmits(['wipe-complete', 'cancelled'])

const progressPercent = ref(0)
const currentPass = ref(1)
const totalPasses = ref(1)
const bytesWritten = ref(0)
const totalBytes = ref(props.selectedDevice?.size_bytes ?? 0)
const currentSpeed = ref(0)
const timeRemaining = ref(0)
const currentPattern = ref('Initializing')

const logs = ref([])
const showLogs = ref(false)

let isCancelled = false

function toggleLogs() {
  showLogs.value = !showLogs.value
}

async function startWiping() {
  logs.value.push('Starting secure wipe...')
  try {
    // Call backend wipe function
    await invoke('start_wipe', { device_path: props.selectedDevice.path })

    // Mock progress simulation - replace with real IPC event binding
    let progress = 0
    while (progress < 100 && !isCancelled) {
      await new Promise(r => setTimeout(r, 500))
      progress += 5
      progressPercent.value = progress
      currentPass.value = 1
      totalPasses.value = 1
      bytesWritten.value = props.selectedDevice.size_bytes * (progress / 100)
      currentPattern.value = 'Zero Pass'
      logs.value.push(`Progress: ${progressPercent.value.toFixed(1)}%`)
      if (progress >= 100) break
    }
    if (!isCancelled) emit('wipe-complete', props.selectedDevice)
  } catch (error) {
    logs.value.push(`Error: ${error}`)
    console.error(error)
  }
}

function cancelWipe() {
  isCancelled = true
  invoke('cancel_wipe')
  emit('cancelled')
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const k = 1024,
    sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function formatDuration(sec) {
  if (!sec) return '0s'
  const h = Math.floor(sec / 3600)
  const m = Math.floor((sec % 3600) / 60)
  const s = Math.floor(sec % 60)
  return (h ? h + 'h ' : '') + (m ? m + 'm ' : '') + s + 's'
}

onMounted(() => {
  startWiping()
})
</script>

<style scoped>
.wipe-progress {
  max-width: 600px;
  margin: auto;
}
</style>
