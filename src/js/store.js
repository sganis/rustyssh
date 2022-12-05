import { writable } from "svelte/store";

export const CurrentPath = writable("");
export const FileStore = writable([]);
export const UserStore = writable({
  user: "",
  server: "",
  isConnected: false,
  isConnecting: false,
  error: "",
});
