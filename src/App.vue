<template>
  <div id="app" class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
    <div class="container mx-auto px-4 py-8">
      <!-- Header -->
      <header class="text-center mb-12">
        <div class="flex items-center justify-center mb-6">
          <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg flex items-center justify-center mr-4">
            <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
            </svg>
          </div>
          <div>
            <h1 class="text-4xl font-bold text-white">SecureWipe</h1>
            <p class="text-gray-300">Professional Data Erasure Solution</p>
          </div>
        </div>
        
        <div class="flex justify-center space-x-8 text-sm text-gray-300">
          <div class="flex items-center">
            <div class="w-3 h-3 bg-green-500 rounded-full mr-2"></div>
            <span>NIST Compliant</span>
          </div>
          <div class="flex items-center">
            <div class="w-3 h-3 bg-blue-500 rounded-full mr-2"></div>
            <span>Tamper-Proof Certificates</span>
          </div>
          <div class="flex items-center">
            <div class="w-3 h-3 bg-purple-500 rounded-full mr-2"></div>
            <span>Cross-Platform</span>
          </div>
        </div>
      </header>

      <!-- Navigation -->
      <nav class="flex justify-center mb-8">
        <div class="bg-slate-800 rounded-lg p-1 flex space-x-1">
          <button
            v-for="step in steps"
            :key="step.id"
            @click="currentStep = step.id"
            :class="[
              'px-4 py-2 rounded-md text-sm font-medium transition-all duration-200',
              currentStep === step.id
                ? 'bg-blue-600 text-white shadow-lg'
                : 'text-gray-300 hover:text-white hover:bg-slate-700'
            ]"
          >
            <component :is="step.icon" class="w-4 h-4 inline mr-2" />
            {{ step.name }}
          </button>
        </div>
      </nav>

      <!-- Main Content -->
      <main class="max-w-6xl mx-auto">
        <transition name="slide-fade" mode="out-in">
          <component 
            :is="currentComponent" 
            @next="nextStep"
            @previous="previousStep"
            @wipe-complete="onWipeComplete"
            @device-selected="onDeviceSelected"
            :selected-device="selectedDevice"
            :wipe-result="wipeResult"
            :certificate="certificate"
          />
        </transition>
      </main>

      <!-- System Tray / Status Bar -->
      <div class="fixed bottom-4 right-4">
        <div class="bg-slate-800 rounded-lg p-3 shadow-lg">
          <div class="flex items-center space-x-3 text-sm text-gray-300">
            <div class="flex items-center">
              <div :class="[
                'w-2 h-2 rounded-full mr-2',
                systemStatus === 'ready' ? 'bg-green-500' : 
                systemStatus === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
              ]"></div>
              <span>{{ systemStatusText }}</span>
            </div>
            <div class="h-4 w-px bg-gray-600"></div>
            <span>v1.0.0</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import DeviceSelector from './components/DeviceSelector.vue'
import WipeProgress from './components/WipeProgress.vue'
import CertificateView from './components/CertificateView.vue'
import AuditLogs from './components/AuditLogs.vue'

// Icons (simplified SVG components)
const HardDriveIcon = { template: '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>' }
const PlayIcon = { template: '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><polygon points="5,3 19,12 5,21"/></svg>' }
const DocumentIcon = { template: '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/></svg>' }
const ClipboardIcon = { template: '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/></svg>' }

// Application state
const currentStep = ref('select')
const selectedDevice = ref(null)
const wipeResult = ref(null)
const certificate = ref(null)
const systemStatus = ref('ready')

const steps = [
  { id: 'select', name: 'Select Device', icon: HardDriveIcon },
  { id: 'wipe', name: 'Secure Wipe', icon: PlayIcon },
  { id: 'certificate', name: 'Certificate', icon: DocumentIcon },
  { id: 'logs', name: 'Audit Logs', icon: ClipboardIcon },
]

const currentComponent = computed(() => {
  switch (currentStep.value) {
    case 'select': return DeviceSelector
    case 'wipe': return WipeProgress
    case 'certificate': return CertificateView
    case 'logs': return AuditLogs
    default: return DeviceSelector
  }
})

const systemStatusText = computed(() => {
  switch (systemStatus.value) {
    case 'ready': return 'System Ready'
    case 'busy': return 'Operation in Progress'
    case 'error': return 'System Error'
    default: return 'Unknown Status'
  }
})

// Event handlers
const nextStep = () => {
  const currentIndex = steps.findIndex(step => step.id === currentStep.value)
  if (currentIndex < steps.length - 1) {
    currentStep.value = steps[currentIndex + 1].id
  }
}

const previousStep = () => {
  const currentIndex = steps.findIndex(step => step.id === currentStep.value)
  if (currentIndex > 0) {
    currentStep.value = steps[currentIndex - 1].id
  }
}

const onDeviceSelected = (device) => {
  selectedDevice.value = device
  currentStep.value = 'wipe'
}

const onWipeComplete = (result, cert) => {
  wipeResult.value = result
  certificate.value = cert
  currentStep.value = 'certificate'
}

// Initialize application
onMounted(async () => {
  try {
    systemStatus.value = 'ready'
    // Initialize any required services
    await invoke('initialize_application')
  } catch (error) {
    console.error('Failed to initialize application:', error)
    systemStatus.value = 'error'
  }
})
</script>

<style scoped>
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease;
}

.slide-fade-enter-from {
  transform: translateX(30px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateX(-30px);
  opacity: 0;
}
</style>
