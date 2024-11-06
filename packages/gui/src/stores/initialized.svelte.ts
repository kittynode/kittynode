import { invoke } from "@tauri-apps/api/core";

let initialized = $state(false);

export const initializedStore = {
  get initialized() {
    return initialized;
  },
  async initialize() {
    await invoke("init_kittynode");
    initialized = true;
  },
  async cheatInitialize() {
    initialized = !initialized;
  },
  async uninitialize() {
    initialized = false;
  },
};
