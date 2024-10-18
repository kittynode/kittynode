import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { tick } from "svelte";

let shown = $state(false);

export const window = {
  get shown() {
    return shown;
  },
  async show() {
    if (!shown) {
      await tick();
      await getCurrentWebviewWindow().show();
      shown = true;
    }
  },
};
