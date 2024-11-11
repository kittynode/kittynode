let serverUrl = $state("");

export const serverUrlStore = {
  get serverUrl() {
    return serverUrl;
  },
  async setServerUrl(url: string) {
    serverUrl = url;
  },
};
