import { invoke } from "@tauri-apps/api/core";
import { serverUrlStore } from "./serverUrl.svelte";
import { error } from "$utils/error";

interface PackageConfig {
  values: Record<string, string>;
}

export const packageConfigStore = {
  async getConfig(packageName: string): Promise<PackageConfig> {
    try {
      return await invoke("get_package_config", {
        name: packageName,
        serverUrl: serverUrlStore.serverUrl,
      });
    } catch (e) {
      error(`Failed to get package config: ${e}`);
      throw e;
    }
  },

  async updateConfig(
    packageName: string,
    config: PackageConfig,
  ): Promise<void> {
    try {
      await invoke("update_package_config", {
        name: packageName,
        config,
        serverUrl: serverUrlStore.serverUrl,
      });
    } catch (e) {
      error(`Failed to update package config: ${e}`);
      throw e;
    }
  },
};
