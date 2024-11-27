import type { SystemInfo } from "$lib/types/system_info";
import { invoke } from "@tauri-apps/api/core";
import { serverUrlStore } from "$stores/serverUrl.svelte";

let systemInfo = $state<SystemInfo>();

export const systemInfoStore = {
  get systemInfo() {
    return systemInfo;
  },
  async fetchSystemInfo() {
    try {
      systemInfo = await invoke("system_info", {
        serverUrl: serverUrlStore.serverUrl,
      });
      console.log("Successfully fetched system info.");
    } catch (e) {
      console.error(`Failed to fetch system info: ${e}.`);
    }
  },
};
