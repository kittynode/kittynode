import { check } from "@tauri-apps/plugin-updater";
import type { Update } from "@tauri-apps/plugin-updater";

const TWENTY_FOUR_HOURS = 24 * 60 * 60 * 1000;

// update logic
let needsUpdate: Update | null = null;
let lastChecked = 0;

// dismiss logic
let lastDismissed: number | null = $state(null);

export const needsUpdateStore = {
  get needsUpdate() {
    if (Date.now() > lastChecked + TWENTY_FOUR_HOURS) {
      this.check();
    }
    return needsUpdate;
  },

  async check() {
    needsUpdate = await check();
    lastChecked = Date.now();
  },

  get isDismissed() {
    return lastDismissed && Date.now() > lastDismissed + TWENTY_FOUR_HOURS;
  },

  dismiss() {
    lastDismissed = Date.now();
  },
};
