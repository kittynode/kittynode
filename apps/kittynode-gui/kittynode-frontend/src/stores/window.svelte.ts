import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

let shown = $state(false);

export const win = {
  get shown() {
    return shown;
  },
  async show() {
    if (!shown) {
      // Workaround for https://github.com/tauri-apps/tauri/issues/6027
      await new Promise((resolve) => setTimeout(resolve, 50)); // 50 ms
      await getCurrentWebviewWindow().show();
      shown = true;
    }
  },
};
