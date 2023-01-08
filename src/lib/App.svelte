<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import { downloadDir, appDataDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';

import {FileStore, PageStore, FileViewStore, FilePageStore,
  UserStore, CurrentPath, FileRequested,JsonChanged,JsonData,JsonNewData,
  Message, Error, Progress} from '../js/store'

import {getParent} from "../js/util.js";
import PathBar from "./PathBar.svelte";
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
let isGettingFiles = false;
let showNewFolderModal = false;
let hidden = false;
let newFolderName = "";

$: totalFiles = $FileStore.length;
$: currentFiles = $FileViewStore.length;
$: isTextfile = $PageStore !== "Binary file";
$: ext =  $CurrentPath.split('.').pop();
$: isImage = ['jpg', 'png'].includes(ext);
$: prog = parseInt($Progress * 100);

(async () => {
  const unlisten = await listen('PROGRESS', ({payload}) => {
  //console.log(payload);
  Progress.set(payload.percent);
})
})();

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
    $FileViewStore = [];
    $FilePageStore = 0;
    $FileStore = [];
    $CurrentPath = path;
    isGettingFiles = true;
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path, hidden });
      const js = JSON.parse(r);      
      $FileStore = js.length > 0 ? [...js] : [];  
      console.log('items: ', $FileStore.length);
      console.log($FileStore);
      
    } catch (e) {
      console.log(e);
      $Error = e.toString();
      $FileStore = [];      
    }
    isGettingFiles = false;    
}
const getPage = async (path, page, recordsPerPage) => {
  try {
      const r = await invoke("get_page", { path, page, recordsPerPage });
      const js = JSON.parse(r);
      //console.log(js);
      $PageStore = js; 
      $FileRequested = true;    
    } catch (e) {
      console.log(e)
    }
}
const goUp = async (e) => {
  const path = getParent(e.detail);
  $FileRequested = false;
  await getFiles(path);
}
const pathChanged = async (e) => {
  const path = e.detail;
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
const fileClick = async (e) => {
    const file = e.detail;
    $CurrentPath = file.path;
    if (file.is_dir) {
      await getFiles(file.path)
      $FileRequested = false;
    } else {
      await getPage(file.path, 1, 100)
      $FileRequested = true;
    }
}
const newFolder = async (action) => {
  if (!action) {
    showNewFolderModal = false;
    return;
  }

  let parent = $CurrentPath;
  let name = newFolderName;
  let remotepath = `${parent}/${name}`
  console.log(remotepath)
  showNewFolderModal = false;
  try {
    await invoke("mkdir", { remotepath });
    await getFiles($CurrentPath);
  } catch (ex) {
    console.log(ex)
  }
  isUploading = false;
}
const hiddenToggled = async (e) => {
  hidden = e.detail;
  await getFiles($CurrentPath);
}
const fileDelete = async (e) => {
    const file = e.detail;
    console.log('deleting '+file.path)
    try {
      await invoke("delete", { remotepath:file.path });
      await getFiles($CurrentPath);
    } catch (ex) {
      console.log(ex)
    }
}
const fileRename = async (e) => {
    const [file, newName] = e.detail;
    try {
      let src = file.path;
      let dst = `${$CurrentPath}/${newName}`;
      console.log('renaming '+src + ' to '+dst)
    
      await invoke("rename", { src, dst });
      await getFiles($CurrentPath);
    } catch (ex) {
      console.log(ex)
    }
}
const fileDuplicate = async (e) => {
    const file = e.detail;
    try {
      await invoke("duplicate", { path : file.path });
      await getFiles($CurrentPath);
    } catch (ex) {
      console.log(ex)
    }
}
const saveFile = async () => {
  console.log($JsonData);
  let path = $CurrentPath;
  let data = JSON.stringify($JsonNewData);
  try {
    const r = await invoke("save_file", { path, data });
  } catch (e) {
    console.log(e)
  }
  $JsonChanged = false;
}
</script>



{#if $UserStore.isConnected && !$UserStore.isConnecting}
  <FileBar {currentFiles} {totalFiles} {isDownloading} {isUploading} {hidden}
    on:go-up={goUp} 
    on:download={download} 
    on:upload={upload} 
    on:show-hidden={hiddenToggled}
    on:new-folder={()=> showNewFolderModal=true}
    on:file-duplicate={fileDuplicate}/>
  <div class="progress progress-wrap">
  {#if $Progress > 0 && $Progress < 1.0 }
      <div class="progress-bar" style:width="{prog}%"/>
  {/if}
  </div>
  <div class="path">
    {#if $JsonChanged}
      <button class="btn btn-sm save" on:click={saveFile}>Save</button>
    {/if}    
    <PathBar value={$CurrentPath} on:path-changed={pathChanged}/>
  </div>
  
  {#if $FileRequested}
    {#if isTextfile || isImage}
      <FilePage />
    {:else}
      <FileDownload  />
    {/if}
  {:else}
    <FileList isLoading={isGettingFiles}
      on:file-click={fileClick}  
      on:file-delete={fileDelete} 
      on:file-rename={fileRename}/>
  {/if}
{:else} 
  <Login on:login={login} />
{/if}

<Modal open={showNewFolderModal} onClosed={(action) => newFolder(action)}
  title="New folder">
  <input bind:value={newFolderName} />
</Modal>


<style>
.path {
  margin-left: 15px;
  margin-top: 0px;
  margin-bottom: 5px;
  margin-right: 5px;
  display: flex;
  padding-right: 10px;
  /* border: 1px solid red; */
}
.save {
  padding: 0;
  padding-left: 20px;
  padding-right: 20px;
  color: white;
  background-color: green;
  border: 1px solid darkgreen;
}
.progress-wrap {
  height: 8px;
  margin: 15px;
  margin-top: 5px;
  margin-bottom: 5px;
}
.progress-bar {
  background: darkblue;
}

</style>

    