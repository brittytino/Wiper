<template>
  <div class="min-h-screen bg-gray-50 flex flex-col">
    <!-- Header -->
    <header class="bg-white border-b border-gray-200 py-3 px-6 flex items-center">
      <div class="text-xl font-semibold text-gray-800 flex items-center gap-2">
        🛡️ SecureWipe - Heisenbug
      </div>
      <span class="ml-auto text-sm text-gray-500">NIST SP 800-88 Compliant</span>
    </header>

    <!-- Main wizard area -->
    <main class="flex-1 bg-white border-x border-gray-200 mx-auto w-full max-w-3xl flex flex-col">
      <!-- Introduction -->
      <section class="px-8 py-6 border-b border-gray-100">
        <p class="text-gray-700 leading-relaxed">
          Welcome to <span class="font-medium">SecureWipe</span>.  
          This wizard will guide you through securely erasing a storage device.  
          Please review each step carefully this process is <span class="font-medium text-red-600">irreversible</span>.
        </p>
      </section>

      <!-- Step: Device selection -->
      <section class="px-8 py-6 border-b border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-2">Step 1: Select Device</h2>
        <select id="device" v-model="selectedPath" :disabled="wiping"
          class="w-full p-2.5 rounded-md border border-gray-300 focus:ring-2 focus:ring-sky-400 focus:outline-none bg-white text-gray-900">
          <option value="" disabled>Select a device...</option>
          <option v-for="dev in devices" :key="dev.path" :value="dev.path">
            {{ devDisplay(dev) }}
          </option>
        </select>
        <p class="mt-2 text-xs text-gray-500">Ensure you select the correct disk. Data will be permanently lost.</p>
      </section>

      <!-- Step: Method selection -->
      <section class="px-8 py-6 border-b border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-2">Step 2: Choose Wipe Method</h2>
        <select id="method" v-model="method" :disabled="wiping || !selectedDev"
          class="w-full p-2.5 rounded-md border border-gray-300 focus:ring-2 focus:ring-sky-400 focus:outline-none bg-white text-gray-900">
          <option value="clear">Clear (Overwrite)</option>
          <option value="purge">Purge (Secure Erase)</option>
        </select>
        <p class="mt-2 text-xs text-gray-500">NIST SP 800-88 recommended algorithms for HDDs and SSDs.</p>
      </section>

      <!-- Step: Serial confirmation -->
      <section v-if="selectedDev && selectedDev.serial && !wiping" class="px-8 py-6 border-b border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-2">Step 3: Confirm Serial</h2>
        <p class="text-sm text-gray-600 mb-2">
          Please type the last 4 digits of the device’s serial number:
        </p>
        <input v-model="confirmSerial" placeholder="Last 4 of serial"
          class="w-full p-2.5 rounded-md border border-gray-300 focus:ring-2 focus:ring-sky-400 bg-gray-50 text-gray-900" />
      </section>

      <!-- Progress -->
      <section v-if="wiping || lastProgressPct > 0" class="px-8 py-6 border-b border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-3">Progress</h2>
        <div class="h-3 w-full bg-gray-200 rounded-md overflow-hidden">
          <div class="h-3 bg-sky-500 transition-all duration-300"
            :style="{ width: `${lastProgressPct}%` }"></div>
        </div>
        <div class="flex justify-between items-center mt-2 text-xs text-gray-600">
          <span class="tabular-nums">{{ lastProgressPct.toFixed(0) }}%</span>
          <span class="font-medium">{{ lastProgressMsg }}</span>
        </div>
      </section>

      <!-- Status -->
      <section v-if="status" class="px-8 py-6 border-b border-gray-100">
        <div :class="['w-full px-3 py-2 text-sm rounded-md text-center',
                      isError ? 'bg-red-50 text-red-700 border border-red-200'
                              : 'bg-green-50 text-green-700 border border-green-200']">
          {{ status }}
        </div>
      </section>

      <!-- Certificate -->
      <section v-if="certificate" class="px-8 py-6 border-b border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-2">Wipe Certificate</h2>
        <div class="bg-gray-50 border border-gray-200 rounded-md p-3 space-y-2">
          <div class="flex items-center justify-between">
            <span class="font-medium text-gray-800">Immutable Blockchain-Verified Report</span>
            <span class="text-[11px] px-2 py-0.5 rounded-full bg-sky-100 text-sky-700">Verified</span>
          </div>
          <div>
            <span class="font-medium">JSON:</span>
            <a href="#" @click.prevent="openPath(certificate.json)"
              class="text-sky-600 underline hover:text-sky-800 font-mono text-sm">
              {{ filename(certificate.json) }}
            </a>
          </div>
          <div v-if="certificate.pdf">
            <span class="font-medium">PDF:</span>
            <a href="#" @click.prevent="openPath(certificate.pdf)"
              class="text-sky-600 underline hover:text-sky-800 font-mono text-sm">
              {{ filename(certificate.pdf) }}
            </a>
          </div>
        </div>
      </section>
    </main>

    <!-- Footer actions -->
    <footer class="bg-white border-t border-gray-200 py-3 px-8 flex items-center justify-between">
      <div class="text-xs text-gray-400">SecureWipe © 2025</div>
      <div class="flex gap-3">
        <button @click="loadDevices" :disabled="wiping"
          class="px-4 py-2 rounded-md border border-gray-300 font-medium bg-white text-gray-700 hover:bg-gray-100 disabled:bg-gray-100 disabled:text-gray-400">
          Refresh
        </button>
        <button
          @click="onPrepareWipe"
          :disabled="!canStartWipe || wiping"
          class="px-5 py-2 rounded-md font-medium transition bg-sky-500 text-white hover:bg-sky-600 disabled:bg-gray-200 disabled:text-gray-500">
          {{ wiping ? "Wiping…" : "Start Wipe" }}
        </button>
      </div>
    </footer>
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
        this.isError = false;
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
        this.isError = false;
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
body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  background-color: #f9fafb;
}
</style>
