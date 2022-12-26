<script>
import {FileStore, Error} from '../js/store'
import File from "./File.svelte";
import ErrorBox from "./ErrorBox.svelte";

export let isLoading = true;

</script>
<div class="scrollable border main">   
    <div class="files blur">
      {#each $FileStore as file(file.path)}
        <File {file} {isLoading} 
          on:file-click on:file-delete on:file-rename/>
      {/each}
    </div>
    {#if isLoading}
      <div class="overlay"></div>
    {/if}
    {#if $Error}
      <ErrorBox />
    {/if}
</div>

<style>
.main {
  position: relative;
}
.files {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
}
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  background: white;
  opacity: 0.8; 
  z-index: 100;
  height: 100%;
  
}

</style>