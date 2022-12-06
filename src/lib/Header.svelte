<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {UserStore} from '../js/store'
    import logo from '../assets/logo.png'
    export let version = 'v1.0'

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
  <h1 class="title">Rusty {version}</h1>
  <div class="w100"></div>
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