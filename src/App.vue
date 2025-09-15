<template>
  <div id="app" class="min-h-screen bg-gradient-to-br from-slate-900 via-indigo-900 to-purple-900 relative overflow-hidden">
    <!-- Animated Background Elements -->
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute -top-40 -right-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl animate-pulse"></div>
      <div class="absolute top-1/2 -left-40 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse delay-1000"></div>
      <div class="absolute bottom-20 right-1/3 w-64 h-64 bg-cyan-500/10 rounded-full blur-3xl animate-pulse delay-2000"></div>
    </div>

    <!-- Floating Particles -->
    <div class="absolute inset-0 pointer-events-none">
      <div v-for="n in 20" :key="n" class="absolute animate-float" :style="getParticleStyle(n)">
        <div class="w-1 h-1 bg-white/20 rounded-full"></div>
      </div>
    </div>

    <div class="container mx-auto px-4 py-8 relative z-10">
      <!-- Enhanced Header -->
      <header class="text-center mb-16">
        <div class="flex items-center justify-center mb-8">
          <div class="relative group">
            <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl blur opacity-75 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-pulse"></div>
            <div class="relative w-20 h-20 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl flex items-center justify-center mr-6 transform hover:scale-110 transition-all duration-300">
              <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
              </svg>
            </div>
          </div>
          <div class="text-left">
            <h1 class="text-6xl font-extrabold bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent mb-2">
              SecureWipe
            </h1>
            <p class="text-xl text-gray-300 font-light">Professional Data Erasure Solution</p>
            <div class="w-32 h-1 bg-gradient-to-r from-blue-500 to-purple-600 mt-3 rounded-full"></div>
          </div>
        </div>
        
        <div class="flex justify-center space-x-12 text-sm">
          <div class="group cursor-pointer">
            <div class="flex items-center p-4 rounded-xl bg-slate-800/40 backdrop-blur-sm border border-slate-700/50 hover:border-green-500/50 transition-all duration-300 hover:transform hover:scale-105">
              <div class="relative">
                <div class="w-4 h-4 bg-green-500 rounded-full mr-3 animate-pulse"></div>
                <div class="absolute inset-0 w-4 h-4 bg-green-500 rounded-full mr-3 animate-ping opacity-30"></div>
              </div>
              <span class="text-gray-300 group-hover:text-green-400 font-medium">NIST Compliant</span>
            </div>
          </div>
          <div class="group cursor-pointer">
            <div class="flex items-center p-4 rounded-xl bg-slate-800/40 backdrop-blur-sm border border-slate-700/50 hover:border-blue-500/50 transition-all duration-300 hover:transform hover:scale-105">
              <div class="relative">
                <div class="w-4 h-4 bg-blue-500 rounded-full mr-3 animate-pulse delay-300"></div>
                <div class="absolute inset-0 w-4 h-4 bg-blue-500 rounded-full mr-3 animate-ping opacity-30 delay-300"></div>
              </div>
              <span class="text-gray-300 group-hover:text-blue-400 font-medium">Tamper-Proof Certificates</span>
            </div>
          </div>
          <div class="group cursor-pointer">
            <div class="flex items-center p-4 rounded-xl bg-slate-800/40 backdrop-blur-sm border border-slate-700/50 hover:border-purple-500/50 transition-all duration-300 hover:transform hover:scale-105">
              <div class="relative">
                <div class="w-4 h-4 bg-purple-500 rounded-full mr-3 animate-pulse delay-700"></div>
                <div class="absolute inset-0 w-4 h-4 bg-purple-500 rounded-full mr-3 animate-ping opacity-30 delay-700"></div>
              </div>
              <span class="text-gray-300 group-hover:text-purple-400 font-medium">Cross-Platform</span>
            </div>
          </div>
        </div>
      </header>

      <!-- Enhanced Navigation -->
      <nav class="flex justify-center mb-12">
        <div class="relative">
          <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl blur opacity-30"></div>
          <div class="relative bg-slate-800/80 backdrop-blur-lg rounded-xl p-2 flex space-x-2 border border-slate-700/50">
            <button
              v-for="(step, index) in steps"
              :key="step.id"
              @click="currentStep = step.id"
              :class="[
                'relative px-6 py-3 rounded-lg text-sm font-semibold transition-all duration-300 flex items-center space-x-2',
                currentStep === step.id
                  ? 'bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg shadow-blue-500/25 transform scale-105'
                  : 'text-gray-300 hover:text-white hover:bg-slate-700/50 hover:transform hover:scale-102'
              ]"
            >
              <component :is="step.icon" class="w-5 h-5" />
              <span>{{ step.name }}</span>
              
              <!-- Step indicator -->
              <div v-if="currentStep === step.id" class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
              
              <!-- Progress line -->
              <div 
                v-if="index < steps.length - 1"
                :class="[
                  'absolute top-1/2 -right-6 w-12 h-0.5 -translate-y-1/2 transition-all duration-500',
                  steps.findIndex(s => s.id === currentStep) > index 
                    ? 'bg-gradient-to-r from-green-500 to-blue-500' 
                    : 'bg-gray-600'
                ]"
              ></div>
            </button>
          </div>
        </div>
      </nav>

      <!-- Enhanced Main Content -->
      <main class="max-w-7xl mx-auto">
        <div class="relative">
          <div class="absolute -inset-4 bg-gradient-to-r from-blue-600/10 to-purple-600/10 rounded-2xl blur-xl"></div>
          <div class="relative bg-slate-800/30 backdrop-blur-lg rounded-2xl border border-slate-700/30 p-8 shadow-2xl">
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
                class="min-h-96"
              />
            </transition>
          </div>
        </div>
      </main>

      <!-- Enhanced Progress Indicator -->
      <div class="mt-12 flex justify-center">
        <div class="flex space-x-4">
          <div 
            v-for="(step, index) in steps" 
            :key="step.id"
            :class="[
              'w-16 h-2 rounded-full transition-all duration-500',
              steps.findIndex(s => s.id === currentStep) >= index
                ? 'bg-gradient-to-r from-blue-500 to-purple-600'
                : 'bg-gray-600'
            ]"
          ></div>
        </div>
      </div>

      <!-- Enhanced System Tray / Status Bar -->
      <div class="fixed bottom-6 right-6">
        <div class="relative group">
          <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl blur opacity-30 group-hover:opacity-50 transition duration-300"></div>
          <div class="relative bg-slate-800/90 backdrop-blur-lg rounded-xl p-4 shadow-2xl border border-slate-700/50">
            <div class="flex items-center space-x-4 text-sm">
              <div class="flex items-center space-x-3">
                <div class="relative">
                  <div :class="[
                    'w-3 h-3 rounded-full',
                    systemStatus === 'ready' ? 'bg-green-500' : 
                    systemStatus === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                  ]"></div>
                  <div :class="[
                    'absolute inset-0 w-3 h-3 rounded-full animate-ping opacity-40',
                    systemStatus === 'ready' ? 'bg-green-500' : 
                    systemStatus === 'busy' ? 'bg-yellow-500' : 'bg-red-500'
                  ]"></div>
                </div>
                <span class="text-gray-300 font-medium">{{ systemStatusText }}</span>
              </div>
              
              <div class="h-6 w-px bg-gradient-to-b from-transparent via-gray-600 to-transparent"></div>
              
              <div class="flex items-center space-x-2">
                <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                <span class="text-gray-400 font-mono text-xs">v1.0.0</span>
              </div>
            </div>
            
            <!-- Mini activity indicator -->
            <div class="mt-3 flex space-x-1">
              <div v-for="n in 8" :key="n" :class="[
                'w-1 h-1 rounded-full transition-all duration-300',
                systemStatus === 'busy' && n <= (Date.now() / 200) % 8 
                  ? 'bg-blue-500' 
                  : 'bg-gray-600'
              ]"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Enhanced Floating Action Buttons -->
      <div class="fixed left-6 top-1/2 transform -translate-y-1/2 space-y-4">
        <div class="group">
          <button class="relative w-14 h-14 bg-slate-800/80 backdrop-blur-lg rounded-full border border-slate-700/50 text-gray-300 hover:text-white hover:border-blue-500/50 transition-all duration-300 hover:transform hover:scale-110 shadow-lg">
            <svg class="w-6 h-6 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
            </svg>
            <div class="absolute -right-2 -top-2 w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
          </button>
          <div class="absolute left-16 top-1/2 transform -translate-y-1/2 bg-slate-800 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap">
            Settings
          </div>
        </div>
        
        <div class="group">
          <button class="relative w-14 h-14 bg-slate-800/80 backdrop-blur-lg rounded-full border border-slate-700/50 text-gray-300 hover:text-white hover:border-green-500/50 transition-all duration-300 hover:transform hover:scale-110 shadow-lg">
            <svg class="w-6 h-6 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </button>
          <div class="absolute left-16 top-1/2 transform -translate-y-1/2 bg-slate-800 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap">
            Help
          </div>
        </div>
      </div>

      <!-- Enhanced Loading Overlay -->
      <transition name="fade">
        <div v-if="systemStatus === 'busy'" class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center">
          <div class="bg-slate-800/90 backdrop-blur-lg rounded-2xl p-8 border border-slate-700/50 text-center">
            <div class="relative w-16 h-16 mx-auto mb-4">
              <div class="absolute inset-0 border-4 border-gray-600 rounded-full"></div>
              <div class="absolute inset-0 border-4 border-blue-500 rounded-full border-t-transparent animate-spin"></div>
            </div>
            <p class="text-white font-medium mb-2">Processing...</p>
            <p class="text-gray-400 text-sm">Please wait while the operation completes</p>
          </div>
        </div>
      </transition>
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

// Enhanced Icons with better styling
const HardDriveIcon = { 
  template: `
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <rect x="2" y="3" width="20" height="14" rx="2" ry="2" stroke-width="2"/>
      <line x1="8" y1="21" x2="16" y2="21" stroke-width="2"/>
      <line x1="12" y1="17" x2="12" y2="21" stroke-width="2"/>
      <circle cx="6" cy="14" r="1" fill="currentColor"/>
      <circle cx="10" cy="14" r="1" fill="currentColor"/>
    </svg>
  ` 
}

const PlayIcon = { 
  template: `
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <polygon points="5,3 19,12 5,21" fill="currentColor" stroke-width="2"/>
    </svg>
  ` 
}

const DocumentIcon = { 
  template: `
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z" stroke-width="2"/>
      <circle cx="9" cy="13" r="1" fill="currentColor"/>
      <circle cx="9" cy="16" r="1" fill="currentColor"/>
      <line x1="11" y1="13" x2="17" y2="13" stroke-width="1"/>
      <line x1="11" y1="16" x2="15" y2="16" stroke-width="1"/>
    </svg>
  ` 
}

const ClipboardIcon = { 
  template: `
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" stroke-width="2"/>
      <rect x="8" y="2" width="8" height="4" rx="1" ry="1" stroke-width="2"/>
      <line x1="9" y1="12" x2="15" y2="12" stroke-width="1"/>
      <line x1="9" y1="16" x2="15" y2="16" stroke-width="1"/>
    </svg>
  ` 
}

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

// Enhanced particle system
const getParticleStyle = (n) => {
  const delay = (n * 0.5) + 's'
  const duration = (3 + (n % 4)) + 's'
  const x = Math.random() * 100 + '%'
  const y = Math.random() * 100 + '%'
  
  return {
    left: x,
    top: y,
    animationDelay: delay,
    animationDuration: duration
  }
}

// Event handlers (unchanged logic)
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

// Initialize application (unchanged logic)
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
/* Enhanced animations and transitions */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-fade-enter-from {
  transform: translateX(40px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateX(-40px);
  opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Custom animations */
@keyframes float {
  0%, 100% {
    transform: translateY(0px) rotate(0deg);
    opacity: 0.3;
  }
  25% {
    transform: translateY(-10px) rotate(90deg);
    opacity: 0.6;
  }
  50% {
    transform: translateY(-20px) rotate(180deg);
    opacity: 1;
  }
  75% {
    transform: translateY(-10px) rotate(270deg);
    opacity: 0.6;
  }
}

.animate-float {
  animation: float linear infinite;
}

/* Glassmorphism effects */
.backdrop-blur-lg {
  backdrop-filter: blur(16px);
}

.backdrop-blur-sm {
  backdrop-filter: blur(4px);
}

/* Custom scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(51, 65, 85, 0.3);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(59, 130, 246, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(59, 130, 246, 0.7);
}

/* Enhanced button hover effects */
button {
  position: relative;
  overflow: hidden;
}

button::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  background: radial-gradient(circle, rgba(255,255,255,0.3) 0%, transparent 70%);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
  border-radius: 50%;
  pointer-events: none;
}

button:hover::before {
  width: 300px;
  height: 300px;
}

/* Text glow effect */
.text-glow {
  text-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
}

/* Pulsing border animation */
@keyframes pulse-border {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.7);
  }
  50% {
    box-shadow: 0 0 0 6px rgba(59, 130, 246, 0);
  }
}

.animate-pulse-border {
  animation: pulse-border 2s infinite;
}

/* Smooth focus states */
*:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3);
}

/* Enhanced loading states */
.loading-skeleton {
  background: linear-gradient(
    90deg,
    rgba(71, 85, 105, 0.3) 25%,
    rgba(100, 116, 139, 0.3) 50%,
    rgba(71, 85, 105, 0.3) 75%
  );
  background-size: 200% 100%;
  animation: loading 1.5s infinite;
}

@keyframes loading {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}
</style>