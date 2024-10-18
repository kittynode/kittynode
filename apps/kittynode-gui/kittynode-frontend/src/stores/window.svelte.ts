import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

let shown = $state(false);

export const window = {
  get shown() {
    return shown;
  },
  async show() {
    if (!shown) {
      // Workaround for https://github.com/tauri-apps/tauri/issues/6027
      await new Promise((resolve) => setTimeout(resolve, 42));
      await getCurrentWebviewWindow().show();
      shown = true;
    }
  },
};
