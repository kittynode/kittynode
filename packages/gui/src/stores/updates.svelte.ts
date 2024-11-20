import { check } from "@tauri-apps/plugin-updater";
import type { Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const TWENTY_FOUR_HOURS = 24 * 60 * 60 * 1000;

let currentUpdate = $state<Update | null>(null);
let dismissedTime = $state<number | null>(null);
let lastChecked = $state(0);
let processingUpdate = $state(false);

export const updates = {
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

  get isProcessing() {
    return processingUpdate;
  },

  dismiss() {
    dismissedTime = Date.now();
  },

  async installUpdate() {
    if (!currentUpdate || processingUpdate) return;

    processingUpdate = true;
    try {
      let downloaded = 0;
      let contentLength = 0;

      await currentUpdate.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength as number;
            console.log(
              `Started downloading ${event.data.contentLength} bytes.`,
            );
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            console.log(`Downloaded ${downloaded} from ${contentLength}.`);
            break;
          case "Finished":
            console.log("Download finished.");
            break;
        }
      });

      console.log("Update installed.");
      await relaunch();
    } catch (error) {
      alert(`Failed to update Kittynode: ${error}.`);
      console.error(error);
    }
    processingUpdate = false;
  },
};
