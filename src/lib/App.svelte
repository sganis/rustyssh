<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import {FileStore, UserStore, Message, Error} from '../js/store'
import {sleep} from "../js/util.js";
import FileBar from "$lib/FileBar.svelte";
import FileList from "$lib/FileList.svelte";
import Login from "./Login.svelte";


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
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path });
      const js = JSON.parse(r);
      //console.log(js)
      $FileStore = js.length > 0 ? [...js] : [];      
    } catch (e) {
      console.log(e);
      $Error = e.toString();
      $FileStore = [];      
    }
};

</script>


{#if $UserStore.isConnected && !$UserStore.isConnecting}
  <FileBar {totalFiles} />
  <FileList on:file-click={fileClick}  />
{:else} 
  <Login on:login={login} />
{/if}

<style>


</style>

    