<script>
import {FileStore} from '../js/store'
import {fade, scale} from 'svelte/transition'
import File from "./File.svelte";
import {getParent} from "../js/util"
export let currentPath;

$: parent = {
    path: getParent(currentPath),
    name: '..',
    filetype: 'DIR',
    size: 0,
    modified: 0,
    selected: false,
};
$: hasParent = currentPath !== "/";

</script>
<div class="scrollable border">   
    {#if hasParent}
    <File file={parent} on:file-click />
    {/if}
    {#each $FileStore as file(file.path)}
    <div in:fade="{{duration:500}}" >
        <File {file} on:file-click on:clear-selection />
    </div>
    {/each}
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