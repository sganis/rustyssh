<script>
import {FileStore,CurrentPath} from '../js/store'
//import {fade, scale} from 'svelte/transition'
import File from "./File.svelte";
import Error from "./Error.svelte";
import {getParent} from "../js/util"

/** @type {string} */
export let error = "";

$: parent = {
    path: getParent($CurrentPath),
    name: '..',
    filetype: 'DIR',
    size: 0,
    modified: 0,
    selected: false,
};
$: hasParent = $CurrentPath !== "/";

</script>
<div class="scrollable border">   
    {#if hasParent}
    <File file={parent} on:file-click />
    {/if}
    {#each $FileStore as file(file.path)}
      <!-- <div in:fade="{{duration:500}}" > -->
      <File {file} on:file-click />
    {/each}
    {#if error}
      <Error {error} />
    {/if}
</div>
<style>
.scrollable {
  overflow-y: auto; 
  height: 100vh; 
  scrollbar-gutter: stable;
  margin: 15px;
  padding: 10px; 
}
</style>