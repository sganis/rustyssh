<script>
// @ts-nocheck

import InfiniteScroll from "svelte-infinite-scroll";
import {FileStore, FileViewStore, FilePageStore, Error} from '../js/store'
import File from "./File.svelte";
import ErrorBox from "./ErrorBox.svelte";
//import {createEventDispatcher} from 'svelte'

// const dispatch = createEventDispatcher();

// const fileClick = (file) => {
//   console.log('file clicked', file.path);
//   dispatch('file-click', file);
// }

export let isLoading = true;

let size = 20;

$: {
  //console.log('page: ', $FilePageStore);
  let pagefiles = [...$FileStore].splice(size * $FilePageStore, size ); 
  //console.log('page files: ', pagefiles);
  
  $FileViewStore = [
  ...$FileViewStore,
  ...pagefiles
  ];
  
  //console.log('fileview: ', $FileViewStore);
  
}

</script>
<div class="scrollable border main">   
    {#each $FileViewStore as file}
      <File {file} {isLoading} 
        on:file-click on:file-delete on:file-rename on:file-duplicate/>
    {/each}
    <InfiniteScroll 
      hasMore={$FileViewStore.length < $FileStore.length} 
      threshold={100} on:loadMore={() => {$FilePageStore++}} />
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
/* .files {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
} */
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