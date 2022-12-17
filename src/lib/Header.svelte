<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { getVersion } from '@tauri-apps/api/app';
    import {onMount} from 'svelte';
    import {UserStore} from '../js/store'
    import logo from '../assets/logo.png'

    let version = '';

    const appVersion = async () => {
      return await getVersion();
    }

    onMount(async () => {
      version = await appVersion();
    })

    const logout = async () => {
    try {
      const r = await invoke("disconnect");
      $UserStore.isConnected = false;
      $UserStore.needPassword = false;
      
    } catch (e) {
      console.log(e);
    }

  };

</script>

<div class="header">
  <img src={logo} alt="logo"/>
  <h1 class="title">Rusty</h1>
  <div class="w100"></div>
  <div>v{version}</div>
  {#if $UserStore.isConnected}
    <div>{$UserStore.user}@{$UserStore.server}</div>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <div><a href="#" on:click={logout}>Logout</a></div>
  {:else}
    <div></div>
  {/if}
</div>

<style>
    .title {
        white-space: nowrap;
    }
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 10px;
        padding-right: 10px;
    }
</style>