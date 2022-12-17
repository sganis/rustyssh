import { writable } from "svelte/store";
// import { tweened } from "svelte/motion";
// import { cubicOut } from "svelte/easing";

export const CurrentPath = writable("");
export const FileStore = writable([]);
export const PageStore = writable([]);
export const FileRequested = writable(false);
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
