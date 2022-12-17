<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import { downloadDir, appDataDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';

import {FileStore, PageStore, 
  UserStore, CurrentPath, FileRequested, NewFolderName,
  Message, Error, Progress} from '../js/store'

import {sleep, getParent} from "../js/util.js";
import FileBar from "$lib/FileBar.svelte";
import FileList from "$lib/FileList.svelte";
import FilePage from "$lib/FilePage.svelte";
import FileDownload from "$lib/FileDownload.svelte";
import Login from "./Login.svelte";
import { appWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event'
import Modal from "./Modal.svelte";

let isDownloading = false;
let isUploading = false;
let showNewFolder = false;

$: totalFiles = $FileStore.length;
$: isTextfile = $PageStore !== "Binary file";

(async () => {
  const unlisten = await listen('PROGRESS', ({payload}) => {
  //console.log(payload);
  Progress.set(payload.percent);
})
})();

const fileClick = async (e) => {
    const file = e.detail;
    if (file.is_dir) {
      await getFiles(file.path)
      $FileRequested = false;
    } else {
      await getPage(file.path, 1, 100)
      $FileRequested = true;
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
        $Error = ex;
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
      $FileRequested = true;    
    } catch (e) {
      console.log(e)
    }
}
const goUp = async (e) => {
  const path = getParent(e.detail);
  console.log('going up to ', path)
  $FileRequested = false;
  await getFiles(path);
}
const download = async (e) => {
  isDownloading = true;
  const remotepath = e.detail;
  const filename = remotepath.substr(remotepath.lastIndexOf('/') + 1);
  const downloadFolder = await downloadDir();
  try {
    let path = `${downloadFolder}${filename}`;
    const new_filename = await invoke("get_new_filename", { path });
    const localpath = `${downloadFolder}\\${new_filename}`;
    await invoke("download", { remotepath, localpath, window: appWindow});
    //const js = JSON.parse(r);
  } catch (ex) {
    console.log(ex)
  }
  isDownloading = false;
}
const upload = async (e) => {
  isUploading = true;
  let remotepath = e.detail;
  const localpath = await open({
    defaultPath: await appDataDir()
  });
  console.log(localpath);
  try {
    //const new_filename = await invoke("get_new_filename", { path });
    let filename = localpath.substr(localpath.lastIndexOf('\\') + 1);
    remotepath = `${remotepath}/${filename}`
    await invoke("upload", { localpath, remotepath, window: appWindow});
    await getFiles($CurrentPath);
  } catch (ex) {
    console.log(ex)
  }
  isUploading = false;
}
const newFolderDialog = async (e) => {
  let remotepath = e.detail;
  showNewFolder = true;
}
const newFolder = async () => {
  let parent = $CurrentPath;
  let name = $NewFolderName;
  let remotepath = `${parent}/${name}`
  console.log(remotepath)
  showNewFolder = false;
  try {
    await invoke("mkdir", { remotepath });
    await getFiles($CurrentPath);
  } catch (ex) {
    console.log(ex)
  }
  isUploading = false;
}


</script>



{#if $UserStore.isConnected && !$UserStore.isConnecting}
  <FileBar {totalFiles} {isDownloading} {isUploading}
    on:go-up={goUp} on:download={download} 
    on:upload={upload} on:new-folder-dialog={newFolderDialog}/>
  <div class="progress">
  {#if $Progress > 0 && $Progress < 1.0 }
    <progress value={$Progress} />
  {/if}
  </div>
  {#if $FileRequested}
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

{#if showNewFolder}
	<Modal on:close="{newFolder}" >
    <p>Folder name:</p>
    <input bind:value={$NewFolderName}/>
    <br/>
    <br/>
  </Modal>

{/if}



<style>
.progress {
  height: 8px;
  /* display: block;
  width: auto; */
  margin: 15px;
  margin-top: 5px;
  margin-bottom: 5px;
  /* border: 1px solid white; */
}
progress {
  height: 4px;
  display: block;
  width: auto;
  /* border-radius: 1px; */
  border: 0;
  padding: 0;
}
progress::-webkit-progress-value {
  background: #01579b;
  border-top-left-radius: 1px;
  border-bottom-left-radius: 1px;
}
/* Background - webkit browsers */
progress::-webkit-progress-bar {
  background: #fff;
}

</style>

    