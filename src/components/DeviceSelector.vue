<template>
  <div class="device-selector">
    <div class="bg-slate-800 rounded-xl p-8 shadow-2xl">
      <div class="text-center mb-8">
        <h2 class="text-3xl font-bold text-white mb-4">Select Storage Device</h2>
        <p class="text-gray-300">Choose the device you want to securely wipe</p>
        
        <div class="mt-6 flex justify-center">
          <button
            @click="refreshDevices"
            :disabled="isScanning"
            class="flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white rounded-lg transition-colors"
          >
            <RefreshIcon :class="['w-4 h-4 mr-2', isScanning && 'animate-spin']" />
            {{ isScanning ? 'Scanning...' : 'Refresh Devices' }}
          </button>
        </div>
      </div>

      <!-- Device List -->
      <div v-if="devices.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="device in devices"
          :key="device.id"
          @click="selectDevice(device)"
          :class="[
            'device-card p-6 rounded-lg border-2 cursor-pointer transition-all duration-200',
            selectedDevice?.id === device.id
              ? 'border-blue-500 bg-blue-900/30 shadow-lg scale-105'
              : 'border-slate-600 bg-slate-700 hover:border-slate-500 hover:bg-slate-600'
          ]"
        >
          <!-- Device Icon -->
          <div class="flex items-center justify-center mb-4">
            <div :class="[
              'w-16 h-16 rounded-lg flex items-center justify-center',
              getDeviceIconBackground(device.device_type)
            ]">
              <component :is="getDeviceIcon(device.device_type)" class="w-8 h-8 text-white" />
            </div>
          </div>

          <!-- Device Info -->
          <div class="text-center">
            <h3 class="text-lg font-semibold text-white mb-2">{{ device.name }}</h3>
            <p class="text-sm text-gray-300 mb-1">{{ device.size_human }}</p>
            <p class="text-xs text-gray-400 mb-3">{{ device.device_type }}</p>
            
            <!-- Device Details -->
            <div class="space-y-2 text-xs">
              <div v-if="device.model" class="flex justify-between">
                <span class="text-gray-400">Model:</span>
                <span class="text-gray-300">{{ device.model }}</span>
              </div>
              
              <div v-if="device.serial_number" class="flex justify-between">
                <span class="text-gray-400">Serial:</span>
                <span class="text-gray-300 font-mono">{{ device.serial_number.substring(0, 8) }}...</span>
              </div>
              
              <div class="flex justify-between">
                <span class="text-gray-400">Status:</span>
                <span :class="[
                  'font-medium',
                  device.is_mounted ? 'text-green-400' : 'text-yellow-400'
                ]">
                  {{ device.is_mounted ? 'Mounted' : 'Unmounted' }}
                </span>
              </div>
              
              <div class="flex justify-between">
                <span class="text-gray-400">Health:</span>
                <span :class="getHealthStatusColor(device.health_status)">
                  {{ device.health_status }}
                </span>
              </div>
            </div>
            
            <!-- Removable Badge -->
            <div v-if="device.is_removable" class="mt-3">
              <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-900 text-green-200">
                <UsbIcon class="w-3 h-3 mr-1" />
                Removable
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else-if="!isScanning" class="text-center py-16">
        <div class="w-24 h-24 bg-slate-700 rounded-full flex items-center justify-center mx-auto mb-6">
          <HardDriveIcon class="w-12 h-12 text-gray-400" />
        </div>
        <h3 class="text-xl font-medium text-gray-300 mb-2">No Storage Devices Found</h3>
        <p class="text-gray-400 mb-6">Connect a storage device and click refresh to scan for devices.</p>
        <button
          @click="refreshDevices"
          class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
        >
          Scan for Devices
        </button>
      </div>

      <!-- Loading State -->
      <div v-else class="text-center py-16">
        <div class="w-16 h-16 border-4 border-blue-600 border-t-transparent rounded-full animate-spin mx-auto mb-6"></div>
        <h3 class="text-xl font-medium text-white mb-2">Scanning for Devices</h3>
        <p class="text-gray-400">Please wait while we detect storage devices...</p>
      </div>

      <!-- Action Bar -->
      <div v-if="selectedDevice" class="mt-8 pt-6 border-t border-slate-600">
        <div class="bg-slate-700 rounded-lg p-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <div class="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center mr-4">
                <component :is="getDeviceIcon(selectedDevice.device_type)" class="w-5 h-5 text-white" />
              </div>
              <div>
                <h4 class="font-medium text-white">{{ selectedDevice.name }}</h4>
                <p class="text-sm text-gray-400">{{ selectedDevice.size_human }} • {{ selectedDevice.device_type }}</p>
              </div>
            </div>
            
            <div class="flex space-x-3">
              <button
                @click="selectedDevice = null"
                class="px-4 py-2 bg-slate-600 hover:bg-slate-500 text-white rounded-lg transition-colors"
              >
                Cancel
              </button>
              
              <button
                @click="proceedToWipe"
                class="px-6 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors font-medium"
              >
                Proceed to Secure Wipe
              </button>
            </div>
          </div>
          
          <!-- Warning -->
          <div class="mt-4 p-3 bg-red-900/30 border border-red-500/30 rounded-lg">
            <div class="flex items-center">
              <ExclamationTriangleIcon class="w-5 h-5 text-red-400 mr-2" />
              <p class="text-sm text-red-200">
                <strong>Warning:</strong> This operation will permanently delete all data on the selected device. This action cannot be undone.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// Icons (simplified)
const HardDriveIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>' }
const SsdIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><rect x="2" y="4" width="20" height="16" rx="2" ry="2"/><path d="m7 8 3 3-3 3"/><path d="M13 12h4"/></svg>' }
const UsbIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><circle cx="10" cy="7" r="1"/><circle cx="4" cy="20" r="1"/><path d="m4.7 19.3 15-15"/><path d="m21 3-3 1 2 2Z"/><path d="M9.26 7.68 5 12l2 5"/><path d="m10 14 5 2 2-5"/></svg>' }
const SdIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z"/><line x1="3" y1="6" x2="21" y2="6"/><path d="M16 10a4 4 0 0 1-8 0"/></svg>' }
const RefreshIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg>' }
const ExclamationTriangleIcon = { template: '<svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="m12 17.02.01 0"/></svg>' }

// Component props and emits
const emit = defineEmits(['device-selected'])

// Reactive state
const devices = ref([])
const selectedDevice = ref(null)
const isScanning = ref(false)

// Methods
const refreshDevices = async () => {
  try {
    isScanning.value = true
    devices.value = await invoke('scan_storage_devices')
  } catch (error) {
    console.error('Failed to scan devices:', error)
    // Show error notification
  } finally {
    isScanning.value = false
  }
}

const selectDevice = (device) => {
  selectedDevice.value = device
}

const proceedToWipe = () => {
  if (selectedDevice.value) {
    emit('device-selected', selectedDevice.value)
  }
}

const getDeviceIcon = (deviceType) => {
  switch (deviceType) {
    case 'SSD': return SsdIcon
    case 'USB': return UsbIcon
    case 'SD': return SdIcon
    default: return HardDriveIcon
  }
}

const getDeviceIconBackground = (deviceType) => {
  switch (deviceType) {
    case 'SSD': return 'bg-purple-600'
    case 'USB': return 'bg-green-600'
    case 'SD': return 'bg-orange-600'
    case 'HDD': return 'bg-blue-600'
    default: return 'bg-gray-600'
  }
}

const getHealthStatusColor = (status) => {
  switch (status) {
    case 'Healthy': return 'text-green-400'
    case 'Warning': return 'text-yellow-400'
    case 'Critical': return 'text-red-400'
    default: return 'text-gray-400'
  }
}

// Initialize component
onMounted(() => {
  refreshDevices()
})
</script>

<style scoped>
.device-card {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.device-card:hover {
  transform: translateY(-2px);
}
</style>
