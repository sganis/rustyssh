<script>
  import { invoke } from "@tauri-apps/api/tauri"
  import {createEventDispatcher, onMount} from 'svelte'
  import {UserStore, Message, Error} from '../js/store'
  import Spinner from './Spinner.svelte'
  //import { sleep } from '../js/util';
  const dispatch = createEventDispatcher();

  let server = 'localhost';
  let user = 'support';
  let password = '';
    
  onMount(async () => {
    try {
      const s = await invoke("read_settings"); 
      server = s.server;
      user = s.user;
    } catch (ex) {
      console.log('Cannot read settings: '+ex);
    }
	});

    const handleSubmit = async () => {
      $Error = '';
      $Message = "Connecting...";
      $UserStore.isConnecting = true;
      //await sleep(1000);
      dispatch('login', {server,user,password});
    }

</script>

<div class="full login-container">
    <form class="login" on:submit|preventDefault={handleSubmit} >
          <label for="server">Server</label>
          <input
            type="text"
            bind:value={server}
            disabled="{$UserStore.isConnecting}"
            id="server"
            placeholder="Enter remote ssh host name or IP address"         
          />
          <label for="user">User</label>
          <input
            type="user"
            bind:value={user}
            disabled="{$UserStore.isConnecting}"
            id="user"
            placeholder="Enter username"            
          />
          <label for="password">Password</label>
          <input
            type="password"
            bind:value={password}
            disabled="{$UserStore.isConnecting || !$UserStore.needPassword}"
            id="password"
            placeholder="Password"            
          />
          <div class="login-button">
            <div class="w100"></div>          
            <button type="submit" class="btn btn-light border" 
              disabled={$UserStore.isConnecting}>
              <i class="bi-power rp10"></i>Connect
            </button>
          </div>
        
    </form>
    <br/>
    <div class="message ">
      {#if $UserStore.isConnecting}
      <div class="spinner">
        <Spinner/> {$Message}
      </div>
      {:else}
      <div class="error">
        {@html $Error}
      </div>
      {/if}
    </div>
</div>

  <style>
    .login-container {
        display: flex;
        justify-content: center;
        align-items: center;
        padding-top: 20px
    }
    .login{
        width: 400px;
        display: flex;
        /* justify-content: center; */
        /* align-items: center; */
        flex-direction: column;  
        gap: 5px;
    }
    .login-button {
        display: flex;
        justify-content: space-between;
        padding-top: 10px;
    }
    .spinner {
        display: flex;
        justify-content:flex-start;   
        align-items: center;        
        width: 400px; 
        gap: 10px;
    }
    .message {
      width: 400px; 
      height: 100%; 
    }
    .error {
      color: red;
    }
  </style>