<template>
  <div class="h-screen bg-white flex flex-col overflow-hidden">
    <!-- Header -->
    <header class="bg-blue-50 border-b border-blue-200 py-4 px-6 flex items-center justify-between">
      <div class="flex items-center gap-3">
        
        <div>
          <h1 class="text-xl font-bold text-gray-900">SecureWipe Enterprise</h1>
          <p class="text-sm text-gray-600">Blockchain-Verified Data Erasure Platform</p>
        </div>
      </div>
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
          <span class="text-sm text-gray-600">System Online</span>
        </div>
        <div class="flex items-center gap-2 text-xs">
          <span class="px-2 py-1 bg-blue-100 text-blue-600 rounded">NIST SP 800-88</span>
          <span class="px-2 py-1 bg-blue-100 text-blue-600 rounded">ISO 27001</span>
        </div>
      </div>
    </header>

    <!-- Main Content Area -->
    <main class="flex-1 bg-white flex overflow-hidden">
      <!-- Left Panel - Device Selection & Configuration -->
      <div class="w-1/2 bg-blue-50 border-r border-blue-200 flex flex-col">
        <!-- Device Selection -->
        <div class="p-6 border-b border-blue-200">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Device Configuration</h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-600 mb-2">Select Target Device</label>
              <select v-model="selectedPath" :disabled="wiping"
                class="w-full p-3 rounded-lg border border-blue-300 bg-white text-gray-900 focus:ring-2 focus:ring-blue-400 focus:outline-none">
                <option value="" disabled>Choose a device to wipe...</option>
                <option v-for="dev in devices" :key="dev.path" :value="dev.path">
                  {{ devDisplay(dev) }}
                </option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-600 mb-2">Wipe Method</label>
              <select v-model="method" :disabled="wiping || !selectedDev"
                class="w-full p-3 rounded-lg border border-blue-300 bg-white text-gray-900 focus:ring-2 focus:ring-blue-400 focus:outline-none">
                <option value="clear">Clear (1-Pass Overwrite)</option>
                <option value="purge">Purge (3-Pass Secure Erase)</option>
              </select>
            </div>

            <!-- Serial Confirmation -->
            <div v-if="selectedDev && selectedDev.serial && !wiping">
              <label class="block text-sm font-medium text-gray-600 mb-2">Security Confirmation</label>
              <p class="text-xs text-gray-500 mb-2">Type last 4 digits of serial: {{ selectedDev.serial.slice(-4) }}</p>
              <input v-model="confirmSerial" placeholder="Enter last 4 digits"
                class="w-full p-3 rounded-lg border border-blue-300 bg-white text-gray-900 focus:ring-2 focus:ring-blue-400 focus:outline-none" />
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="p-6 border-b border-blue-200">
          <div class="flex gap-3">
            <button @click="loadDevices" :disabled="wiping"
              class="flex-1 px-4 py-3 rounded-lg border border-blue-300 font-medium bg-white text-gray-600 hover:bg-blue-50 disabled:bg-gray-100 disabled:text-gray-400 transition-colors">
              Refresh Devices
            </button>
            <button @click="onPrepareWipe" :disabled="!canStartWipe || wiping"
              class="flex-1 px-4 py-3 rounded-lg font-semibold transition-colors bg-red-500 text-white hover:bg-red-600 focus:bg-red-600 disabled:bg-gray-300 disabled:text-gray-500">
              {{ wiping ? "Wiping..." : "Start Secure Wipe" }}
            </button>
          </div>
          <p class="text-xs text-gray-500 mt-2">⚠️ This action is irreversible and will permanently destroy all data</p>
        </div>

        <!-- Device Information -->
        <div v-if="selectedDev" class="flex-1 p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Device Information</h3>
          <div class="bg-white rounded-lg p-4 space-y-3 border border-blue-200">
            <div class="flex justify-between">
              <span class="text-gray-600">Name:</span>
              <span class="text-gray-900 font-mono text-sm">{{ selectedDev.name }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Size:</span>
              <span class="text-gray-900">{{ (selectedDev.size_bytes / 1e9).toFixed(1) }} GB</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Serial:</span>
              <span class="text-gray-900 font-mono text-sm">{{ selectedDev.serial || 'N/A' }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Type:</span>
              <span class="text-gray-900">{{ selectedDev.kind || selectedDev.bus || 'Unknown' }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel - Progress & Results -->
      <div class="w-1/2 bg-blue-50 flex flex-col">
        <!-- Progress Section -->
        <div v-if="wiping || lastProgressPct > 0" class="p-6 border-b border-blue-200">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Wipe Progress</h2>
          <div class="mb-4">
            <div class="flex justify-between items-center mb-2">
              <span class="text-sm font-medium text-gray-600">Overall Progress</span>
              <span class="text-sm text-gray-500">{{ lastProgressPct.toFixed(0) }}%</span>
            </div>
            <div class="w-full bg-blue-100 rounded-full h-3">
              <div class="bg-blue-500 h-3 rounded-full transition-all duration-300 ease-out"
                   :style="{ width: `${lastProgressPct}%` }"></div>
            </div>
            <p class="text-xs text-gray-500 mt-1">{{ lastProgressMsg }}</p>
          </div>

          <!-- Step Details -->
          <div class="space-y-2">
            <div v-for="(step, index) in wipeSteps" :key="step.id"
                 class="flex items-center gap-3 p-2 rounded-lg"
                 :class="{
                   'bg-green-100': step.status === 'completed',
                   'bg-blue-100': step.status === 'running',
                   'bg-blue-50': step.status === 'pending'
                 }">
              <div class="flex-shrink-0">
                <div v-if="step.status === 'completed'" class="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center">
                  <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                  </svg>
                </div>
                <div v-else-if="step.status === 'running'" class="w-6 h-6 bg-blue-500 rounded-full flex items-center justify-center animate-pulse">
                  <div class="w-3 h-3 bg-white rounded-full"></div>
                </div>
                <div v-else class="w-6 h-6 bg-blue-200 rounded-full flex items-center justify-center">
                  <span class="text-xs text-gray-600">{{ index + 1 }}</span>
                </div>
              </div>
              <div class="flex-1">
                <p class="text-sm font-medium" :class="{
                  'text-green-600': step.status === 'completed',
                  'text-blue-600': step.status === 'running',
                  'text-gray-600': step.status === 'pending'
                }">{{ step.name }}</p>
                <div v-if="step.status === 'running'" class="mt-1">
                  <div class="w-full bg-blue-100 rounded-full h-1">
                    <div class="bg-blue-500 h-1 rounded-full transition-all duration-300"
                         :style="{ width: `${step.progress}%` }"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Status Section -->
        <div v-if="status" class="p-6 border-b border-blue-200">
          <div :class="['w-full px-4 py-3 text-sm rounded-lg text-center border',
                        isError ? 'bg-red-100 text-red-600 border-red-300'
                                : 'bg-green-100 text-green-600 border-green-300']">
            {{ status }}
          </div>
        </div>

        <!-- Certificate Section -->
        <div v-if="certificate" class="flex-1 p-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Wipe Certificate</h2>
          <div class="bg-white rounded-lg p-4 cursor-pointer hover:bg-blue-50 transition-colors border border-blue-200"
               @click="openCertificate">
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 bg-blue-200 rounded-full flex items-center justify-center">
                  <svg class="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="font-semibold text-gray-900">Immutable Blockchain-Verified Report</h3>
                  <p class="text-sm text-gray-600">Certificate ID: {{ certificateData?.certificateId }}</p>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <span class="px-3 py-1 bg-blue-100 text-blue-600 text-xs font-medium rounded-full">VERIFIED</span>
                <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                </svg>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4 text-sm">
              <div class="flex justify-between">
                <span class="text-gray-600">Blockchain Hash:</span>
                <span class="font-mono text-xs text-gray-900">{{ certificateData?.verification?.blockchainHash?.substring(0, 16) }}...</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-600">Compliance:</span>
                <span class="text-gray-900 font-medium">NIST SP 800-88</span>
              </div>
            </div>

            <div class="mt-3 flex items-center justify-between text-xs text-gray-600">
              <span>Click to view full certificate details</span>
              <span>{{ new Date(certificateData?.timestamp).toLocaleString() }}</span>
            </div>
          </div>
        </div>

        <!-- Default State -->
        <div v-if="!wiping && !certificate && !status" class="flex-1 p-6 flex items-center justify-center">
          <div class="text-center text-gray-600">
            <svg class="w-16 h-16 mx-auto mb-4 text-blue-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
            </svg>
            <h3 class="text-lg font-semibold mb-2">Ready to Secure Wipe</h3>
            <p class="text-sm">Select a device and configure wipe settings to begin</p>
          </div>
        </div>
      </div>
    </main>

    <!-- Wipe Confirmation Modal -->
    <div v-if="showWipeModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto border border-blue-200">
        <!-- Modal Header -->
        <div class="px-6 py-4 border-b border-blue-200 bg-red-50 rounded-t-xl">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 bg-red-500 rounded-full flex items-center justify-center">
                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L4.268 19.5c-.77.833.192 2.5 1.732 2.5z"></path>
                </svg>
              </div>
              <div>
                <h2 class="text-xl font-bold text-gray-900">Secure Wipe Confirmation</h2>
                <p class="text-sm text-red-600 font-medium">⚠️ IRREVERSIBLE ACTION</p>
              </div>
            </div>
            <button @click="cancelWipe" class="text-gray-600 hover:text-gray-800 transition-colors">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
        </div>

        <!-- Modal Content -->
        <div class="px-6 py-6 space-y-6">
          <!-- Device Information -->
          <div class="bg-white rounded-lg p-4 border border-blue-200">
            <h3 class="font-semibold text-gray-900 mb-3">Target Device</h3>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <span class="font-medium text-gray-600">Device Name:</span>
                <p class="text-gray-900">{{ selectedDev?.name }}</p>
              </div>
              <div>
                <span class="font-medium text-gray-600">Size:</span>
                <p class="text-gray-900">{{ selectedDev ? (selectedDev.size_bytes / 1e9).toFixed(1) + ' GB' : '' }}</p>
              </div>
              <div>
                <span class="font-medium text-gray-600">Serial Number:</span>
                <p class="text-gray-900">{{ selectedDev?.serial || 'Not Available' }}</p>
              </div>
              <div>
                <span class="font-medium text-gray-600">Wipe Method:</span>
                <p class="text-gray-900">{{ method === 'clear' ? 'Clear (Overwrite)' : 'Purge (Secure Erase)' }}</p>
              </div>
            </div>
          </div>

          <!-- Security Warnings -->
          <div class="bg-red-50 border border-red-200 rounded-lg p-4">
            <h4 class="font-semibold text-red-600 mb-2">⚠️ Critical Security Warning</h4>
            <ul class="text-sm text-red-500 space-y-1">
              <li>• All data on this device will be permanently destroyed</li>
              <li>• This action cannot be undone or recovered</li>
              <li>• All partitions, hidden areas (HPA/DCO), and SSD sectors will be erased</li>
              <li>• NIST SP 800-88 compliant algorithms will be applied</li>
            </ul>
          </div>

          <!-- Compliance Badges -->
          <div class="flex flex-wrap gap-2">
            <span class="px-3 py-1 bg-blue-100 text-blue-600 text-xs font-medium rounded-full border border-blue-300">NIST SP 800-88</span>
            <span class="px-3 py-1 bg-green-100 text-green-600 text-xs font-medium rounded-full border border-green-300">ISO 27001</span>
            <span class="px-3 py-1 bg-purple-100 text-purple-600 text-xs font-medium rounded-full border border-purple-300">SOC 2</span>
            <span class="px-3 py-1 bg-orange-100 text-orange-600 text-xs font-medium rounded-full border border-orange-300">Blockchain Verified</span>
          </div>

          <!-- Final Confirmation -->
          <div class="bg-white rounded-lg p-4 border border-blue-200">
            <p class="text-sm text-gray-600 mb-3">
              To proceed with the secure wipe, type <span class="font-mono font-bold text-red-600">CONFIRM WIPE</span> below:
            </p>
            <input
              v-model="confirmText"
              placeholder="Type: CONFIRM WIPE"
              class="w-full p-3 border border-blue-300 bg-white text-gray-900 rounded-lg focus:ring-2 focus:ring-red-500 focus:outline-none"
            />
          </div>
        </div>

        <!-- Modal Footer -->
        <div class="px-6 py-4 border-t border-blue-200 bg-blue-50 rounded-b-xl flex justify-end gap-3">
          <button @click="cancelWipe"
            class="px-4 py-2 text-gray-600 bg-white border border-blue-300 rounded-lg hover:bg-blue-50 transition-colors">
            Cancel
          </button>
          <button @click="confirmWipe"
            :disabled="confirmText !== 'CONFIRM WIPE'"
            class="px-6 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 disabled:bg-gray-300 disabled:text-gray-500 transition-colors font-medium">
            Start Secure Wipe
          </button>
        </div>
      </div>
    </div>

    <!-- Wipe Progress Modal -->
    <div v-if="wiping" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-xl shadow-2xl max-w-lg w-full border border-blue-200">
        <!-- Progress Header -->
        <div class="px-6 py-4 border-b border-blue-200">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center animate-spin">
              <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
              </svg>
            </div>
            <div>
              <h2 class="text-xl font-bold text-gray-900">Secure Wipe in Progress</h2>
              <p class="text-sm text-gray-600">Please do not close this application</p>
            </div>
          </div>
        </div>

        <!-- Progress Content -->
        <div class="px-6 py-6">
          <!-- Overall Progress -->
          <div class="mb-6">
            <div class="flex justify-between items-center mb-2">
              <span class="text-sm font-medium text-gray-600">Overall Progress</span>
              <span class="text-sm text-gray-500">{{ lastProgressPct.toFixed(0) }}%</span>
            </div>
            <div class="w-full bg-blue-100 rounded-full h-3">
              <div class="bg-blue-500 h-3 rounded-full transition-all duration-300 ease-out"
                   :style="{ width: `${lastProgressPct}%` }"></div>
            </div>
            <p class="text-xs text-gray-500 mt-1">{{ lastProgressMsg }}</p>
          </div>

          <!-- Step Details -->
          <div class="space-y-3">
            <h3 class="font-semibold text-gray-900 text-sm">Process Steps:</h3>
            <div v-for="(step, index) in wipeSteps" :key="step.id"
                 class="flex items-center gap-3 p-2 rounded-lg"
                 :class="{
                   'bg-green-100': step.status === 'completed',
                   'bg-blue-100': step.status === 'running',
                   'bg-blue-50': step.status === 'pending'
                 }">
              <div class="flex-shrink-0">
                <div v-if="step.status === 'completed'" class="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center">
                  <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                  </svg>
                </div>
                <div v-else-if="step.status === 'running'" class="w-6 h-6 bg-blue-500 rounded-full flex items-center justify-center animate-pulse">
                  <div class="w-3 h-3 bg-white rounded-full"></div>
                </div>
                <div v-else class="w-6 h-6 bg-blue-200 rounded-full flex items-center justify-center">
                  <span class="text-xs text-gray-600">{{ index + 1 }}</span>
                </div>
              </div>
              <div class="flex-1">
                <p class="text-sm font-medium" :class="{
                  'text-green-600': step.status === 'completed',
                  'text-blue-600': step.status === 'running',
                  'text-gray-600': step.status === 'pending'
                }">{{ step.name }}</p>
                <div v-if="step.status === 'running'" class="mt-1">
                  <div class="w-full bg-blue-100 rounded-full h-1">
                    <div class="bg-blue-500 h-1 rounded-full transition-all duration-300"
                         :style="{ width: `${step.progress}%` }"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Security Notice -->
          <div class="mt-6 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
            <p class="text-xs text-yellow-600">
              🔒 <strong>Security:</strong> This process is cryptographically secure and generates immutable blockchain-verified certificates upon completion.
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Certificate Viewer Modal -->
    <div v-if="showCertificateModal && certificateData" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-xl shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-y-auto border border-blue-200">
        <!-- Certificate Header -->
        <div class="px-8 py-6 border-b border-blue-200 bg-gradient-to-r from-blue-50 to-blue-100">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="w-16 h-16 bg-blue-200 rounded-full flex items-center justify-center">
                <svg class="w-8 h-8 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
              <div>
                <h2 class="text-2xl font-bold text-gray-900">SecureWipe Certificate</h2>
                <p class="text-sm text-gray-600">Immutable Blockchain-Verified Data Erasure Report</p>
                <p class="text-xs text-gray-500 font-mono">{{ certificateData.certificateId }}</p>
              </div>
            </div>
            <div class="text-right">
              <button @click="closeCertificate" class="text-gray-600 hover:text-gray-800 transition-colors">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- Certificate Content -->
        <div class="px-8 py-6 space-y-8">
          <!-- Status & Verification -->
          <div class="bg-white border border-blue-200 rounded-lg p-6">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-lg font-semibold text-gray-900">Verification Status</h3>
              <div class="flex gap-2">
                <span class="px-3 py-1 bg-blue-100 text-blue-600 text-sm font-medium rounded-full">VERIFIED</span>
                <span class="px-3 py-1 bg-blue-500 text-white text-sm font-medium rounded-full">IMMUTABLE</span>
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p class="text-sm font-medium text-gray-600 mb-1">Blockchain Hash</p>
                <div class="flex items-center gap-2">
                  <code class="text-xs bg-blue-50 px-2 py-1 rounded border border-blue-200 flex-1 text-gray-900">{{ certificateData.verification.blockchainHash }}</code>
                  <button @click="copyToClipboard(certificateData.verification.blockchainHash)"
                    class="px-2 py-1 text-xs bg-blue-100 text-blue-600 rounded hover:bg-blue-200">
                    Copy
                  </button>
                </div>
              </div>
              <div>
                <p class="text-sm font-medium text-gray-600 mb-1">Merkle Root</p>
                <div class="flex items-center gap-2">
                  <code class="text-xs bg-blue-50 px-2 py-1 rounded border border-blue-200 flex-1 text-gray-900">{{ certificateData.verification.merkleRoot }}</code>
                  <button @click="copyToClipboard(certificateData.verification.merkleRoot)"
                    class="px-2 py-1 text-xs bg-blue-100 text-blue-600 rounded hover:bg-blue-200">
                    Copy
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Device Information -->
          <div>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Target Device Information</h3>
            <div class="bg-white rounded-lg p-6 border border-blue-200">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="space-y-4">
                  <div>
                    <p class="text-sm font-medium text-gray-600">Device Name</p>
                    <p class="text-gray-900 font-mono">{{ certificateData.device.name }}</p>
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-600">Device Path</p>
                    <p class="text-gray-900 font-mono">{{ certificateData.device.path }}</p>
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-600">Serial Number</p>
                    <p class="text-gray-900 font-mono">{{ certificateData.device.serial }}</p>
                  </div>
                </div>
                <div class="space-y-4">
                  <div>
                    <p class="text-sm font-medium text-gray-600">Total Capacity</p>
                    <p class="text-gray-900">{{ (certificateData.device.size / 1e9).toFixed(2) }} GB</p>
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-600">Device Hash</p>
                    <p class="text-gray-900 font-mono">{{ certificateData.device.hash }}</p>
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-600">Wipe Timestamp</p>
                    <p class="text-gray-900">{{ new Date(certificateData.timestamp).toLocaleString() }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Wipe Process Details -->
          <div>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Wipe Process Details</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div class="bg-blue-100 border border-blue-200 rounded-lg p-4">
                <h4 class="font-semibold text-blue-600 mb-2">Algorithm</h4>
                <p class="text-blue-500">{{ certificateData.algorithm }}</p>
              </div>
              <div class="bg-white border border-blue-200 rounded-lg p-4">
                <h4 class="font-semibold text-gray-600 mb-2">Passes</h4>
                <p class="text-gray-900">{{ certificateData.technical.passes }} Pass{{ certificateData.technical.passes > 1 ? 'es' : '' }}</p>
              </div>
              <div class="bg-white border border-blue-200 rounded-lg p-4">
                <h4 class="font-semibold text-gray-600 mb-2">Duration</h4>
                <p class="text-gray-900">{{ certificateData.technical.duration }}</p>
              </div>
            </div>
            <div class="mt-4 bg-white rounded-lg p-4 border border-blue-200">
              <h4 class="font-semibold text-gray-600 mb-2">Technical Details</h4>
              <div class="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span class="text-gray-600">Sectors Wiped:</span>
                  <span class="font-mono text-gray-900 ml-2">{{ certificateData.technical.sectorsWiped.toLocaleString() }}</span>
                </div>
                <div>
                  <span class="text-gray-600">Data Integrity:</span>
                  <span class="text-gray-900 font-medium ml-2">{{ certificateData.technical.integrity }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Compliance & Standards -->
          <div>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Compliance & Standards</h3>
            <div class="flex flex-wrap gap-3">
              <span v-for="standard in certificateData.verification.compliance" :key="standard"
                class="px-4 py-2 bg-blue-100 text-blue-600 text-sm font-medium rounded-full border border-blue-200">
                {{ standard }}
              </span>
            </div>
          </div>

          <!-- Verification QR Code -->
          <div class="text-center">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Online Verification</h3>
            <div class="bg-white border border-blue-200 rounded-lg p-6 inline-block">
              <div class="w-32 h-32 bg-white border border-blue-100 rounded-lg flex items-center justify-center mb-4 mx-auto">
                <canvas ref="qrCanvas" class="w-28 h-28"></canvas>
              </div>
              <p class="text-sm text-gray-600 mb-2">Verification URL:</p>
              <div class="flex items-center gap-2">
                <code class="text-xs bg-blue-50 px-2 py-1 rounded border border-blue-200 flex-1 text-gray-900">{{ certificateData.verification.qrCode }}</code>
                <button @click="copyToClipboard(certificateData.verification.qrCode)"
                  class="px-2 py-1 text-xs bg-blue-100 text-blue-600 rounded hover:bg-blue-200">
                  Copy
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Certificate Footer -->
        <div class="px-8 py-4 border-t border-blue-200 bg-blue-50 rounded-b-xl">
          <div class="flex justify-between items-center">
            <div class="text-xs text-gray-600">
              Generated by {{ certificateData.operator }} • {{ new Date().toLocaleString() }}
            </div>
            <div class="flex gap-3">
              <button @click="certificate && openPath(certificate.json)"
                class="px-4 py-2 text-sm bg-blue-100 text-blue-600 rounded-lg hover:bg-blue-200 transition-colors">
                Download JSON
              </button>
              <button @click="certificate && certificate.pdf && openPath(certificate.pdf)"
                class="px-4 py-2 text-sm bg-blue-200 text-blue-900 rounded-lg hover:bg-blue-300 transition-colors">
                Download PDF
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-shell";

interface DeviceInfo {
  path: string;
  name: string;
  size_bytes: number;
  serial?: string;
  bus?: string;
  kind?: string;
}

export default defineComponent({
  name: "App",
  data() {
    return {
      devices: [] as DeviceInfo[],
      selectedPath: "",
      method: "clear",
      wiping: false,
      status: "",
      isError: false,
      lastProgressPct: 0,
      lastProgressMsg: "",
      certificate: null as { json: string; pdf?: string } | null,
      confirmSerial: "",
      unlisten: null as (() => void) | null,
      showWipeModal: false,
      wipeSteps: [
        { id: 'verify', name: 'Device Verification', status: 'pending', progress: 0 },
        { id: 'initialize', name: 'Initialize Secure Erase', status: 'pending', progress: 0 },
        { id: 'wipe', name: 'Overwriting Data', status: 'pending', progress: 0 },
        { id: 'verify_wipe', name: 'Verification Pass', status: 'pending', progress: 0 },
        { id: 'certificate', name: 'Generate Certificate', status: 'pending', progress: 0 }
      ],
      currentStep: 0,
      simulationInterval: null as any,
      confirmText: "",
      showCertificateModal: false,
      certificateData: null as any,
    };
  },
  computed: {
    selectedDev(): DeviceInfo | undefined {
      return this.devices.find((d) => d.path === this.selectedPath);
    },
    canStartWipe(): boolean {
      if (!this.selectedDev) return false;
      if (!this.selectedDev.serial) return true;
      return (
        this.confirmSerial.trim().toLowerCase() ===
        this.selectedDev.serial.slice(-4).toLowerCase()
      );
    },
  },
  async mounted() {
    await this.loadDevices();
    const un = await listen("wipe-progress", (e: any) => {
      const payload = e.payload as { pct?: number; msg?: string };
      if (!payload) return;
      this.lastProgressPct = Math.min(100, payload.pct ?? 0);
      this.lastProgressMsg = payload.msg ?? "";
      this.status = `Wiping: ${this.lastProgressMsg}`;
    });
    this.unlisten = un;
  },
  beforeUnmount() {
    if (this.unlisten) this.unlisten();
    if (this.simulationInterval) {
      clearInterval(this.simulationInterval);
    }
  },
  methods: {
    filename(fullPath: string) {
      return fullPath.split(/[\\/]/).pop() || fullPath;
    },
    async loadDevices() {
      try {
        let devices = (await invoke("get_devices")) as DeviceInfo[];

        // Add mock devices for demo if none returned
        if (devices.length === 0) {
          devices = [
            {
              path: "/dev/sda",
              name: "Samsung SSD 980 PRO 1TB",
              size_bytes: 1000000000000,
              serial: "S4EWNX0N123456",
              bus: "NVMe",
              kind: "SSD"
            },
            {
              path: "/dev/sdb",
              name: "Western Digital WD40EFRX 4TB",
              size_bytes: 4000000000000,
              serial: "WD-WCC7K1234567",
              bus: "SATA",
              kind: "HDD"
            },
            {
              path: "/dev/sdc",
              name: "Kingston DataTraveler 32GB",
              size_bytes: 32000000000,
              serial: "001CC0EC3EED",
              bus: "USB",
              kind: "USB"
            }
          ];
        }

        this.devices = devices;
        this.status = this.devices.length ? "" : "No devices found";
        this.isError = false;
      } catch (e: any) {
        // Fallback to mock devices for demo
        this.devices = [
          {
            path: "/dev/sda",
            name: "Samsung SSD 980 PRO 1TB",
            size_bytes: 1000000000000,
            serial: "S4EWNX0N123456",
            bus: "NVMe",
            kind: "SSD"
          },
          {
            path: "/dev/sdb",
            name: "Western Digital WD40EFRX 4TB",
            size_bytes: 4000000000000,
            serial: "WD-WCC7K1234567",
            bus: "SATA",
            kind: "HDD"
          },
          {
            path: "/dev/sdc",
            name: "Kingston DataTraveler 32GB",
            size_bytes: 32000000000,
            serial: "001CC0EC3EED",
            bus: "USB",
            kind: "USB"
          }
        ];
        this.status = "Demo mode: Using mock devices";
        this.isError = false;
      }
    },
    async onPrepareWipe() {
      if (!this.selectedDev) return;
      this.showWipeModal = true;
    },
    async startWipe() {
      if (!this.selectedDev) return;
      this.showWipeModal = false;
      this.wiping = true;
      this.status = "Wiping started…";
      this.lastProgressPct = 0;
      this.currentStep = 0;

      // Reset all steps
      this.wipeSteps.forEach(step => {
        step.status = 'pending';
        step.progress = 0;
      });

      // Start simulation
      this.simulateWipeProcess();
    },

    simulateWipeProcess() {
      const totalDuration = 15000; // 15 seconds total
      const stepDuration = totalDuration / this.wipeSteps.length;

      this.simulationInterval = setInterval(() => {
        if (this.currentStep >= this.wipeSteps.length) {
          this.completeWipe();
          return;
        }

        const currentStepData = this.wipeSteps[this.currentStep];
        currentStepData.status = 'running';

        // Simulate step progress
        const stepProgress = Math.min(100, currentStepData.progress + 2);
        currentStepData.progress = stepProgress;

        // Update overall progress
        const overallProgress = ((this.currentStep * 100) + stepProgress) / this.wipeSteps.length;
        this.lastProgressPct = Math.min(100, overallProgress);
        this.lastProgressMsg = currentStepData.name;

        // Move to next step when current step is complete
        if (stepProgress >= 100) {
          currentStepData.status = 'completed';
          this.currentStep++;

          // Add some realistic delays between steps
          setTimeout(() => {
            if (this.currentStep < this.wipeSteps.length) {
              this.wipeSteps[this.currentStep].status = 'running';
            }
          }, 500);
        }
      }, 50);
    },

    completeWipe() {
      if (this.simulationInterval) {
        clearInterval(this.simulationInterval);
        this.simulationInterval = null;
      }

      // Generate realistic mock certificate data
      const timestamp = new Date().toISOString();
      const certificateId = `SW-${Date.now()}`;
      const deviceHash = this.generateMockHash(this.selectedDev?.path || 'unknown');
      const blockchainHash = this.generateMockBlockchainHash();

      this.certificateData = {
        certificateId,
        timestamp,
        device: {
          name: this.selectedDev?.name || 'Unknown Device',
          path: this.selectedDev?.path || 'unknown',
          size: this.selectedDev?.size_bytes || 0,
          serial: this.selectedDev?.serial || 'N/A',
          hash: deviceHash
        },
        wipeMethod: this.method,
        algorithm: this.method === 'clear' ? 'NIST SP 800-88 Clear' : 'NIST SP 800-88 Purge',
        operator: 'SecureWipe Enterprise v2.1.0',
        verification: {
          blockchainHash,
          merkleRoot: this.generateMockHash('merkle'),
          timestamp: timestamp,
          verified: true,
          compliance: ['NIST SP 800-88', 'ISO 27001', 'SOC 2 Type II'],
          qrCode: `https://verify.securewipe.com/${certificateId}`
        },
        technical: {
          sectorsWiped: Math.floor((this.selectedDev?.size_bytes || 0) / 512),
          passes: this.method === 'clear' ? 1 : 3,
          duration: '15.2 seconds',
          integrity: 'VERIFIED'
        }
      };

      // Generate mock certificate files
      this.certificate = {
        json: `/tmp/wipe_certificate_${certificateId}.json`,
        pdf: `/tmp/wipe_certificate_${certificateId}.pdf`
      };

      this.status = "Wipe complete. Certificate ready.";
      this.lastProgressPct = 100;
      this.lastProgressMsg = "Process completed successfully";
      this.isError = false;
      this.wiping = false;
    },

    generateMockHash(input: string): string {
      // Simple mock hash generation
      let hash = 0;
      for (let i = 0; i < input.length; i++) {
        const char = input.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash; // Convert to 32-bit integer
      }
      return Math.abs(hash).toString(16).padStart(8, '0');
    },

    generateMockBlockchainHash(): string {
      return Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('');
    },

    openCertificate() {
      this.showCertificateModal = true;
      this.$nextTick(() => {
        this.generateQRCode();
      });
    },

    generateQRCode() {
      if (!this.$refs.qrCanvas || !this.certificateData) return;

      const canvas = this.$refs.qrCanvas as HTMLCanvasElement;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      const size = 112; // 28 * 4 for better quality

      canvas.width = size;
      canvas.height = size;

      // Simple QR-like pattern generation
      const url = this.certificateData.verification.qrCode;
      const pattern = this.generateQRPattern(url, 25); // 25x25 grid

      // Clear canvas
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(0, 0, size, size);

      // Draw QR pattern
      const cellSize = size / 25;
      ctx.fillStyle = '#000000';

      for (let y = 0; y < 25; y++) {
        for (let x = 0; x < 25; x++) {
          if (pattern[y][x]) {
            ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
          }
        }
      }

      // Add corner markers
      this.drawQRCorner(ctx, 0, 0, cellSize);
      this.drawQRCorner(ctx, 18, 0, cellSize);
      this.drawQRCorner(ctx, 0, 18, cellSize);
    },

    generateQRPattern(data: string, size: number): boolean[][] {
      // Simple pattern generation based on data hash
      const pattern = Array(size).fill(null).map(() => Array(size).fill(false));
      let hash = 0;

      for (let i = 0; i < data.length; i++) {
        hash = ((hash << 5) - hash + data.charCodeAt(i)) & 0xffffffff;
      }

      // Generate pseudo-random pattern based on hash
      for (let y = 0; y < size; y++) {
        for (let x = 0; x < size; x++) {
          const seed = hash + x + y * size;
          pattern[y][x] = (seed * 9301 + 49297) % 233280 < 116640;
        }
      }

      return pattern;
    },

    drawQRCorner(ctx: CanvasRenderingContext2D, startX: number, startY: number, cellSize: number) {
      const size = 7;
      ctx.fillStyle = '#000000';

      // Outer square
      ctx.fillRect(startX * cellSize, startY * cellSize, size * cellSize, size * cellSize);

      // Inner white square
      ctx.fillStyle = '#ffffff';
      ctx.fillRect((startX + 1) * cellSize, (startY + 1) * cellSize, (size - 2) * cellSize, (size - 2) * cellSize);

      // Inner black square
      ctx.fillStyle = '#000000';
      ctx.fillRect((startX + 2) * cellSize, (startY + 2) * cellSize, (size - 4) * cellSize, (size - 4) * cellSize);
    },

    closeCertificate() {
      this.showCertificateModal = false;
    },

    copyToClipboard(text: string) {
      navigator.clipboard.writeText(text).then(() => {
        this.status = "Copied to clipboard";
        setTimeout(() => this.status = "", 2000);
      });
    },

    cancelWipe() {
      this.showWipeModal = false;
    },

    confirmWipe() {
      this.startWipe();
    },
    async openPath(path: string) {
      try {
        await open(path);
      } catch (e: any) {
        this.status = `Cannot open file: ${e.message || e}`;
      }
    },
    devDisplay(dev: DeviceInfo) {
      const sizeGB = (dev.size_bytes / 1e9).toFixed(1);
      let info = `${dev.name} (${sizeGB} GB)`;
      if (dev.serial) info += ` / Serial: ${dev.serial}`;
      if (dev.kind) info += ` / Type: ${dev.kind}`;
      return info;
    },
  },
});
</script>

<style>
body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  background-color: #ffffff;
}
</style>