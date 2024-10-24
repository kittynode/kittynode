import { invoke } from "@tauri-apps/api/core";

let initialized = $state(false);

export const initializedStore = {
  get initialized() {
    return initialized;
  },
  async initialize() {
    if (!initialized) {
      await invoke("init_kittynode");
      initialized = true;
    }
  },
  async uninitialize() {
    if (initialized) {
      initialized = false;
    }
  },
};
