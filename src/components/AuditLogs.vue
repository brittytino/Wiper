<template>
  <div class="audit-logs p-8 bg-slate-900 rounded-lg max-w-4xl mx-auto shadow-lg text-white">
    <h2 class="text-3xl font-bold mb-6">Audit Logs</h2>
    <div v-if="logs.length">
      <ul class="overflow-y-auto max-h-96 font-mono text-sm space-y-2">
        <li v-for="(log, index) in logs" :key="index" class="p-2 bg-gray-800 rounded">
          <span class="text-gray-400">{{ log.timestamp }}</span> — {{ log.message }}
        </li>
      </ul>
    </div>
    <div v-else>
      <p>No audit log entries found.</p>
    </div>
    <div class="mt-6 flex justify-end">
      <button @click="refreshLogs" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg">Refresh Logs</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const logs = ref([])

async function refreshLogs() {
  try {
    logs.value = await invoke('get_audit_logs') || []
  } catch (error) {
    console.error('Failed to fetch audit logs:', error)
  }
}

onMounted(refreshLogs)
</script>
