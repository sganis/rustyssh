<script>
import {FileStore,CurrentPath, Error} from '../js/store'
//import {fade, scale} from 'svelte/transition'
import File from "./File.svelte";
import ErrorBox from "./ErrorBox.svelte";
import {getParent} from "../js/util"


$: parent = {
    path: getParent($CurrentPath),
    name: '..',
    filetype: 'DIR',
    size: 0,
    modified: 0,
    selected: false,
    is_dir: true,
    is_link: false,
};
$: hasParent = $CurrentPath !== "/";

</script>
<div class="scrollable border">   
    <!-- {#if hasParent}
      <File file={parent} on:file-click />
    {/if} -->
    {#each $FileStore as file(file.path)}
      <!-- <div in:fade="{{duration:500}}" > -->
      <File {file} on:file-click on:file-delete on:file-rename/>
    {/each}
    {#if $Error}
      <ErrorBox />
    {/if}
</div>
<style>

</style>