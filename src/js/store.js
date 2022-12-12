import { writable } from "svelte/store";

export const CurrentPath = writable("");
export const FileStore = writable([]);
export const PageStore = writable([]);
export const Message = writable("");
export const Error = writable("");
export const UserStore = writable({
  user: "",
  server: "",
  isConnected: false,
  isConnecting: false,
  needPassword: false,
});
