<template>
  <div class="certificate-view p-8 bg-slate-900 text-white rounded-lg max-w-3xl mx-auto shadow-lg">
    <h2 class="text-3xl font-bold mb-6">Wipe Certificate</h2>
    <p class="mb-4">The device wipe has been completed successfully. Download the certificate below.</p>

    <div class="mb-6 bg-gray-800 rounded-lg p-6">
      <h3 class="text-xl font-semibold mb-2">Certificate ID:</h3>
      <p class="font-mono break-all">{{ certificate.certificate_id }}</p>

      <h3 class="text-xl font-semibold mt-4 mb-2">Device:</h3>
      <p>{{ certificate.device_info.name }}</p>
      <p v-if="certificate.device_info.model">Model: {{ certificate.device_info.model }}</p>
      <p v-if="certificate.device_info.serial_number">Serial: {{ certificate.device_info.serial_number }}</p>
      <p>Capacity: {{ certificate.device_info.capacity_human }}</p>
      <p>Type: {{ certificate.device_info.device_type }}</p>

      <h3 class="text-xl font-semibold mt-4 mb-2">Wipe Details:</h3>
      <p>Method: {{ certificate.wipe_details.method_used }}</p>
      <p>Passes: {{ certificate.wipe_details.total_passes }}</p>
      <p>Bytes Processed: {{ formatBytes(certificate.wipe_details.bytes_processed) }}</p>
      <p>Duration: {{ formatDuration(certificate.wipe_details.duration_seconds) }}</p>
      <p>Verification Passed: {{ certificate.wipe_details.verification_passed ? 'Yes' : 'No' }}</p>

      <h3 class="text-xl font-semibold mt-4 mb-2">Verification URL:</h3>
      <a 
        :href="certificate.verification_data.verification_url"
        target="_blank"
        class="text-blue-400 hover:underline break-all"
      >{{ certificate.verification_data.verification_url }}</a>

      <div class="mt-6 text-center">
        <img alt="QR Code" :src="'data:image/png;base64,' + certificate.qr_code_data" class="mx-auto w-40 h-40" />
      </div>
    </div>

    <div class="flex justify-center space-x-6">
      <button @click="downloadPDF" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded font-semibold">Download PDF</button>
      <button @click="downloadJSON" class="px-6 py-2 bg-gray-700 hover:bg-gray-800 rounded font-semibold">Download JSON</button>
      <button @click="$emit('previous')" class="px-6 py-2 bg-slate-700 hover:bg-slate-600 rounded font-semibold">Back</button>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  certificate: Object,
})

function downloadPDF() {
  const b64 = props.certificate.pdf_data
  if (!b64) {
    alert('No PDF data available')
    return
  }
  const blob = b64toBlob(b64, 'application/pdf')
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `SecureWipe-Certificate-${props.certificate.certificate_id}.pdf`
  a.click()
  URL.revokeObjectURL(url)
}

function downloadJSON() {
  const jsonStr = JSON.stringify(props.certificate, null, 2)
  const blob = new Blob([jsonStr], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `SecureWipe-Certificate-${props.certificate.certificate_id}.json`
  a.click()
  URL.revokeObjectURL(url)
}

function b64toBlob(b64Data, contentType='', sliceSize=512) {
  const byteCharacters = atob(b64Data)
  const byteArrays = []

  for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
    const slice = byteCharacters.slice(offset, offset + sliceSize)
    const byteNumbers = new Array(slice.length)
    for (let i = 0; i < slice.length; i++) {
      byteNumbers[i] = slice.charCodeAt(i)
    }
    const byteArray = new Uint8Array(byteNumbers)
    byteArrays.push(byteArray)
  }

  return new Blob(byteArrays, { type: contentType })
}

function formatBytes(bytes) {
  if (!bytes) return '0 B'
  const k = 1024,
    sizes = ['B', 'KB', 'MB', 'GB', 'TB'],
    i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function formatDuration(seconds) {
  if (!seconds) return '0s'
  const h = Math.floor(seconds / 3600),
    m = Math.floor((seconds % 3600) / 60),
    s = Math.floor(seconds % 60)
  return (h ? h + 'h ' : '') + (m ? m + 'm ' : '') + s + 's'
}
</script>
