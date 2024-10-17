import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

let shown = $state(false);

export const window = {
  get shown() {
    return shown;
  },
  async show() {
    if (!shown) {
      await getCurrentWebviewWindow().show();
      shown = true;
    }
  },
};
