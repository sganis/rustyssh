<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import {FileStore, PageStore, UserStore, CurrentPath, Message, Error} from '../js/store'
import {sleep, getParent} from "../js/util.js";
import FileBar from "$lib/FileBar.svelte";
import FileList from "$lib/FileList.svelte";
import FilePage from "$lib/FilePage.svelte";
import FileDownload from "$lib/FileDownload.svelte";
import Login from "./Login.svelte";
import { appWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event'

let fileRequested = false;

$: totalFiles = $FileStore.length;
$: isTextfile = $PageStore !== "Binary file";

// await listen("PROGRESS", ({ event, payload }) => { 
//   console.log(payload)
// });

// (async () => {
//   const unlisten = await listen('PROGRESS', ({event, payload}) => {
//   console.log(payload)
// })
// })();

const fileClick = async (e) => {
    const file = e.detail;
    if (file.is_dir) {
      await getFiles(file.path)
      fileRequested = false;
    } else {
      await getPage(file.path, 1, 100)
      fileRequested = true;
    }
}

const login = async (e) => {
    let args = e.detail
    console.log(args)
    $Error = "";
    
    const settings = {
      server: args.server,
      user: args.user,
      password: args.password,
      port: 22,
      private_key: "",
      home_dir: "",
    };

    if (args.password.length==0) {
      try {
        $Message = "Connecting with keys...";
        await invoke("connect_with_key", { settings: settings }); 
        $UserStore.user = args.user;
        $UserStore.server = args.server;
        $UserStore.isConnected = true;
      } catch (ex) {
        console.log(ex);
        $UserStore.needPassword = true;
        $Error = `${ex}<br/>Need passowrd`;
      }
    } else {
      try {
        $Message = "Connecting...";
        await invoke("connect_with_password", { settings: settings }); 
        $UserStore.user = args.user;
        $UserStore.server = args.server;
        $UserStore.isConnected = true;
      } catch (ex) {
        console.log(ex);
        $UserStore.needPassword = true;
        $Error = ex;
      }
    }

    if ($UserStore.isConnected) {
      if ($UserStore.needPassword) {
        $Message = "Setting up SSH keys...";
        try {
          await invoke("setup_ssh", { settings: settings }); 
          $UserStore.needPassword = false;
        } catch (ex) {
          console.log(ex);
          $Error = ex;
        }
      }
      await getFiles("/");
    }
      
    $UserStore.isConnecting=false;
}

const getFiles = async (path) => {
    $Error = "";
    $CurrentPath = path;
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path });
      const js = JSON.parse(r);      
      $FileStore = js.length > 0 ? [...js] : [];  
    } catch (e) {
      console.log(e);
      $Error = e.toString();
      $FileStore = [];      
    }
}

const getPage = async (path, page, recordsPerPage) => {
  try {
      const r = await invoke("get_page", { path, page, recordsPerPage });
      const js = JSON.parse(r);
      console.log(js);
      $PageStore = js; 
      fileRequested = true;    
    } catch (e) {
      console.log(e)
    }
}
const goUp = async (e) => {
  const path = getParent(e.detail);
  console.log('going up to ', path)
  fileRequested = false;
  await getFiles(path);
}
const download = async (e) => {
  //const remotepath = e.detail;
  const remotepath = "/usr.tar";
  const localpath = "C:\\Users\\san\\usr.tar";
  try {
      const r = await invoke("download", { remotepath, localpath, window: appWindow });
      const js = JSON.parse(r);
 
    } catch (e) {
      console.log(e)
    }
}

</script>

{#if $UserStore.isConnected && !$UserStore.isConnecting}
  <FileBar {totalFiles} on:go-up={goUp} on:download={download}/>
  {#if fileRequested}
    {#if isTextfile}
      <FilePage />
    {:else}
      <FileDownload  />
    {/if}
  {:else}
  
    <FileList on:file-click={fileClick}  />
  {/if}
{:else} 
  <Login on:login={login} />
{/if}

<style>


</style>

    