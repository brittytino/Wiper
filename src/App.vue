<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center px-2 py-6">
    <div class="w-full max-w-md mx-auto bg-white border border-gray-200 rounded-2xl shadow-2xl px-6 py-8 space-y-8">

      <h1 class="text-2xl font-bold text-gray-800 text-center mb-2 tracking-tight">SecureWipe</h1>
      <p class="text-gray-500 text-center text-sm mb-6">Cross-platform Disk Erasure & Cert Generator</p>

      <!-- Device selection -->
      <div>
        <label for="device" class="block text-base font-semibold text-gray-700 mb-2">Select Device</label>
        <select id="device" v-model="selectedPath" :disabled="wiping"
          class="w-full p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none bg-gray-50 shadow-sm transition">
          <option value="" disabled>Select a device...</option>
          <option v-for="dev in devices" :key="dev.path" :value="dev.path">
            {{ devDisplay(dev) }}
          </option>
        </select>
      </div>

      <!-- Wipe method -->
      <div>
        <label for="method" class="block text-base font-semibold text-gray-700 mb-2">Wipe Method</label>
        <select id="method" v-model="method" :disabled="wiping || !selectedDev"
          class="w-full p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none bg-gray-50 shadow-sm transition">
          <option value="clear">Clear (Overwrite)</option>
          <option value="purge">Purge (Secure Erase)</option>
        </select>
      </div>

      <!-- Serial confirmation -->
      <div v-if="selectedDev && selectedDev.serial && !wiping">
        <label class="block text-xs text-gray-500 mb-1">Type last 4 digits of serial to confirm</label>
        <input v-model="confirmSerial" placeholder="Last 4 of serial"
          class="w-full p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 bg-gray-100 shadow-sm transition" />
      </div>

      <!-- Actions -->
      <div class="flex gap-2 items-center justify-between mb-2">
        <button
          @click="onPrepareWipe"
          :disabled="!canStartWipe || wiping"
          class="flex-1 py-2 px-4 rounded-lg font-semibold transition
          bg-blue-600 text-white hover:bg-blue-700 focus:bg-blue-700 shadow hover:scale-105 disabled:bg-gray-300 disabled:text-gray-500 disabled:cursor-not-allowed">
          {{ wiping ? "Wiping…" : "One-Click Wipe" }}
        </button>
        <button @click="loadDevices" :disabled="wiping"
          class="px-4 py-2 rounded-lg border font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 disabled:bg-gray-300 disabled:text-gray-400 transition">
          Refresh
        </button>
      </div>

      <!-- Progress -->
      <div v-if="wiping || lastProgressPct > 0" class="w-full pt-2">
        <div class="h-4 w-full bg-gray-200 rounded-lg overflow-hidden relative">
          <div class="h-4 bg-blue-500 rounded-lg transition-all duration-300"
               :style="{ width: `${lastProgressPct}%` }"></div>
        </div>
        <div class="flex justify-between items-center mt-1 text-xs text-gray-600 px-1">
          <span>{{ lastProgressPct.toFixed(0) }}%</span>
          <span class="font-medium">{{ lastProgressMsg }}</span>
        </div>
      </div>

      <!-- Status -->
      <div v-if="status" :class="['w-full p-2 text-sm rounded-lg text-center mt-2', isError ? 'bg-red-100 text-red-700' : 'bg-green-50 text-green-700']">
        {{ status }}
      </div>

      <!-- Certificate -->
      <div v-if="certificate" class="bg-blue-50 border border-blue-200 rounded-xl p-3 mt-4 space-y-2">
        <h3 class="text-lg font-semibold text-blue-800 text-left">Wipe Certificate</h3>
        <div>
          <span class="font-medium">JSON:</span>
          <a href="#" @click.prevent="openPath(certificate.json)"
            class="text-blue-600 underline hover:text-blue-900 font-mono text-sm">
            {{ filename(certificate.json) }}
          </a>
        </div>
        <div v-if="certificate.pdf">
          <span class="font-medium">PDF:</span>
          <a href="#" @click.prevent="openPath(certificate.pdf)"
            class="text-blue-600 underline hover:text-blue-900 font-mono text-sm">
            {{ filename(certificate.pdf) }}
          </a>
        </div>
      </div>

      <div class="text-center pt-6 text-xs text-gray-400">
        SecureWipe © 2025. Cross-platform UI for easy, standards-compliant asset erasure & disposal.
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
  },
  methods: {
    filename(fullPath: string) {
      return fullPath.split(/[\\/]/).pop() || fullPath;
    },
    async loadDevices() {
      try {
        this.devices = (await invoke("get_devices")) as DeviceInfo[];
        this.status = this.devices.length ? "" : "No devices found";
      } catch (e: any) {
        this.isError = true;
        this.status = `Failed to load devices: ${e.message || e}`;
      }
    },
    async onPrepareWipe() {
      if (!this.selectedDev) return;
      const r = window.prompt(
        `Type 'WIPE' to confirm wiping ${this.selectedDev.name}`,
        ""
      );
      if (r?.toUpperCase() === "WIPE") this.startWipe();
    },
    async startWipe() {
      if (!this.selectedDev) return;
      this.wiping = true;
      this.status = "Wiping started…";
      this.lastProgressPct = 0;
      try {
        this.certificate = (await invoke("wipe_device", {
          device_path: this.selectedDev.path,
          method: this.method,
          operator: "operator@example.com",
        })) as { json: string; pdf?: string };
        this.status = "Wipe complete. Certificate ready.";
        this.lastProgressPct = 100;
      } catch (e: any) {
        this.isError = true;
        this.status = `Wipe failed: ${e.message || e}`;
      } finally {
        this.wiping = false;
      }
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
/* System animation style for smoother transitions */
</style>
