<script>
// @ts-nocheck
import {createEventDispatcher} from 'svelte'
import InfiniteScroll from "svelte-infinite-scroll";
import {FileStore, FileViewStore, FilePageStore, Error} from '../js/store'
import File from "./File.svelte";
import ErrorBox from "./ErrorBox.svelte";

export let isLoading = true;

const dispatch = createEventDispatcher();

let size = 10;
//let files = [];
// let fileview =

$: {
  let files = $FileStore.slice().splice(size * $FilePageStore, size * ($FilePageStore + 1)); // clone
  $FileViewStore = [
  ...$FileViewStore,
  ...files
  ];
  console.log('page: ', $FilePageStore);
  console.log('fileview: ', $FileViewStore);
  console.log('total files: ', files);
  
}

</script>
<div class="scrollable border main">   
    <!-- <div class="files blur"> -->
      {#each $FileViewStore as file}
        <File {file} {isLoading} 
          on:file-click on:file-delete on:file-rename/>
      {/each}
      <InfiniteScroll 
        hasMore={$FileViewStore.length < $FileStore.length} 
        threshold={100} on:loadMore={() => {
          console.log('loading more...');
          $FilePageStore++}} />
    <!-- </div> -->
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