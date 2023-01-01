import { writable } from "svelte/store";
// import { tweened } from "svelte/motion";
// import { cubicOut } from "svelte/easing";

export const CurrentPath = writable("");
export const FileStore = writable([]);
export const FileViewStore = writable([]);
export const FilePageStore = writable([]);
export const PageStore = writable([]);
export const FileRequested = writable(false);
export const JsonChanged = writable(false);
export const JsonData = writable("");
export const JsonNewData = writable("");
export const NewFolderName = writable("");
export const Message = writable("");
export const Error = writable("");
export const UserStore = writable({
  user: "",
  server: "",
  isConnected: false,
  isConnecting: false,
  needPassword: false,
});
export const Progress = writable(0);

// export const Progress = tweened(0, {
//   duration: 100,
//   easing: cubicOut
// });
