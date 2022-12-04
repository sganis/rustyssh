<script>
import { invoke } from "@tauri-apps/api/tauri"
import {FileStore, UserStore} from '../js/store'
import {getParent} from "../js/util.js";
import FileBar from "$lib/FileBar.svelte";
import FileList from "$lib/FileList.svelte";
import Login from "./Login.svelte";

let error = "";
let message = "";
let currentPath = "";

$: totalFiles = $FileStore.length


const fileClick = async (e) => {
    const path = e.detail;
    console.log('here: ' + path.toString())
    await getFiles(path)
}

const clearSelection = () =>{
    $FileStore.map((f) => {
        f.selected = false;
    });
    // = [...files];
}

const handleLogin = async (e) => {
    let args = e.detail
    console.log(args)
    error = "";
    const settings = {
      server: args.server,
      user: args.user,
      password: args.password,
      port: 22,
      private_key: "",
      home_dir: "",
    };
    console.log(settings);
    try {
      await invoke("connect", { settings: settings });
      $UserStore.user = args.user;
      $UserStore.server = args.server;
      $UserStore.isConnected = true;
      
      await getFiles("/");
    } catch (ex) {
      console.log(ex);
      $UserStore.error = ex.toString();
    }
    $UserStore.isConnecting=false;
}

  const getFiles = async (path) => {
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path });
      //console.log(r);
      const js = JSON.parse(r);
      console.log(js)
      $FileStore = Object.keys(js).length !== 0 ? [...js] : [];
      currentPath = path;
      console.log('currentPath',currentPath);
    } catch (e) {
      console.log(e);
    }
  };

  const goUp = async (path) => {
    let parent = getParent(path);
    await getFiles(parent);
  };

</script>

{#if $UserStore.isConnected}
<FileBar {totalFiles} {currentPath} />
<FileList {currentPath}
    on:file-click={fileClick}
    on:clear-selection={clearSelection} />
{:else} 
<Login on:login={handleLogin} />
{/if}

<style>


</style>

    