<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import {FileStore, UserStore} from '../js/store'
import {getParent} from "../js/util.js";
import FileBar from "$lib/FileBar.svelte";
import FileList from "$lib/FileList.svelte";
import Login from "./Login.svelte";

let error = "";
let message = "";

$: totalFiles = $FileStore.length


const fileClick = async (e) => {
    const path = e.detail;
    console.log('here: ' + path)
    await getFiles(path)
}

const clearSelection = () =>{
    $FileStore.map((f) => {
        f.selected = false;
    });
    // = [...files];
}

const login = async (e) => {
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

    if (args.password.length==0) {
      try {
        console.log('connecting with key...');
        await invoke("connect_with_key", { settings: settings }); 
        $UserStore.user = args.user;
        $UserStore.server = args.server;
        $UserStore.isConnected = true;
      } catch (ex) {
        console.log(ex);
        $UserStore.needPassword = true;
        $UserStore.error = `${ex}<br/>Need passowrd`;
      }
    } else {
      try {
        console.log('connecting with password...');
        await invoke("connect_with_password", { settings: settings }); 
        $UserStore.user = args.user;
        $UserStore.server = args.server;
        $UserStore.isConnected = true;
      } catch (ex) {
        console.log(ex);
        $UserStore.needPassword = true;
        $UserStore.error = ex;
      }
    }


    if ($UserStore.isConnected) {
      if ($UserStore.needPassword) {
        // setup ssh
        message = "Setting up ssh authentication...";
        try {
          await invoke("setup_ssh", { settings: settings }); 
          $UserStore.needPassword = false;
        } catch (ex) {
          console.log(ex);
          $UserStore.error = ex;
        }
      }

      await getFiles("/");
    }
      
    $UserStore.isConnecting=false;
}

  const getFiles = async (path) => {
    error = "";
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path });
      const js = JSON.parse(r);
      console.log(js)
      $FileStore = js.length > 0 ? [...js] : [];      
    } catch (e) {
      console.log(e);
      error = e.toString();
      $FileStore = [];      
    }
  };

</script>


{#if $UserStore.isConnected}
  <FileBar {totalFiles} />
  <FileList {error} on:file-click={fileClick}  />
{:else} 
  <Login on:login={login} {message}/>
{/if}

<style>


</style>

    