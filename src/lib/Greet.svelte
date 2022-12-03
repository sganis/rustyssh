<script>
    import { invoke } from "@tauri-apps/api/tauri"

    let name = "";
    let greetMsg = ""

    async function greet(){
      greetMsg = await invoke("greet", { name })
    }
  </script>

  <div>
    <div class="row">
      <input id="greet-input" 
        placeholder="Path..." 
        bind:value={name} />
      <button on:click={greet}>
        Greet
      </button>
    </div>
    {#if greetMsg} 
    <p>{greetMsg}</p>
    {/if}
  </div>

<style>
  #greet-input {
  /* margin-left: 10px; */
  margin-right: 5px;
}
</style>