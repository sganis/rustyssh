<script>
    import {createEventDispatcher} from 'svelte'
    import {UserStore} from '../js/store'

    import Spinner from '$lib/Spinner.svelte'
    import { sleep } from '../js/util';
    const dispatch = createEventDispatcher();

    let server = 'localhost';
    let user = 'support';
    let password = '';
    export let message = "";
    
    const handleSubmit = async () => {
      $UserStore.error = '';
      $UserStore.isConnecting = true;
        message = "Connecting...";
        await sleep(1000);
        dispatch('login', {
            server,user,password
        })
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
            <button type="submit" 
              disabled={$UserStore.isConnecting}>
              Connect
            </button>
          </div>
        
    </form>
    <br/>
    <div class="message ">
      {#if $UserStore.isConnecting}
      <div class="spinner">
        <Spinner/> {message}
      </div>
      {:else}
      <div class="error">
        {$UserStore.error}
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
    input:disabled, button:disabled {
        pointer-events: none;
        color:#aaa;
        background: #f5f5f5;
        border-color: #aaa;
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