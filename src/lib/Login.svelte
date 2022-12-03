<script>
    import {createEventDispatcher} from 'svelte'
    import Spinner from '$lib/Spinner.svelte'
    import { sleep } from '../js/util';
    const dispatch = createEventDispatcher();

    let server = 'localhost';
    let user = 'support';
    let password = 'support';
    let isConnecting = false;
    let message = "";

    const handleSubmit = async () => {
        isConnecting = true;
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
            disabled="{isConnecting}"
            id="server"
            placeholder="Enter remote ssh host name or IP address"         
          />
          <label>User</label>
          <input
            type="user"
            value={user}
            disabled="{isConnecting}"
            id="user"
            placeholder="Enter username"            
          />
          <label>Password</label>
          <input
            type="password"
            value={password}
            disabled="{isConnecting}"
            id="password"
            placeholder="Password"            
          />
          <div class="login-button">
          <div class="w100"></div>
          <button type="submit" 
            disabled={isConnecting}>
            Connect
          </button>
          </div>
        
    </form>
    <div class="message">
    {#if isConnecting}
        <Spinner/>
        {message}
    {:else}
        &nbsp;
    {/if}
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