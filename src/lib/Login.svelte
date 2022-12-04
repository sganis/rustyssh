<script>
    import {createEventDispatcher} from 'svelte'
    import {UserStore} from '../js/store'

    import Spinner from '$lib/Spinner.svelte'
    import { sleep } from '../js/util';
    const dispatch = createEventDispatcher();

    let server = 'localhost';
    let user = 'support';
    let password = '';
    let message = "";
    
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
          <label>Server</label>
          <input
            type="text"
            value={server}
            disabled="{$UserStore.isConnecting}"
            id="server"
            placeholder="Enter remote ssh host name or IP address"         
          />
          <label>User</label>
          <input
            type="user"
            value={user}
            disabled="{$UserStore.isConnecting}"
            id="user"
            placeholder="Enter username"            
          />
          <label>Password</label>
          <input
            type="password"
            value={password}
            disabled="{$UserStore.isConnecting}"
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
    <div class="message">
      {#if $UserStore.isConnecting}
        <Spinner/>
        {message}
      {/if}
      {$UserStore.error}
    </div>
</div>

  <style>
    .login-container {
        display: flex;
        justify-content: center;
        align-items: center;
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
    }
    input:disabled, button:disabled {
        pointer-events: none;
        color:#aaa;
        background: #f5f5f5;
        border-color: #aaa;
    }
    .message {
        display: flex;
        justify-content:flex-start;   
        align-items: center;
        width: 400px; 
        gap: 10px;
    }
  </style>