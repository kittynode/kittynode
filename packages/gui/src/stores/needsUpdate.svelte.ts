import { check } from "@tauri-apps/plugin-updater";
import type { Update } from "@tauri-apps/plugin-updater";

const TWENTY_FOUR_HOURS = 24 * 60 * 60 * 1000;

let currentUpdate = $state<Update | null>(null);
let dismissedTime = $state<number | null>(null);
let lastChecked = $state(0);

export const needsUpdateStore = {
  async getUpdate() {
    const now = Date.now();
    if (now > lastChecked + TWENTY_FOUR_HOURS) {
      currentUpdate = await check();
      lastChecked = now;
    }
    return currentUpdate;
  },

  get hasUpdate() {
    return currentUpdate !== null;
  },

  get isDismissed() {
    if (!dismissedTime) return false;
    return Date.now() < dismissedTime + TWENTY_FOUR_HOURS;
  },

  dismiss() {
    dismissedTime = Date.now();
  },
};
