<script>
import {createEventDispatcher} from 'svelte'
import {CurrentPath, FileRequested, Progress} from '../js/store'
export let totalFiles = 0;
export let isDownloading = false;
export let isUploading = false;

const dispatch = createEventDispatcher();

$: inRootFolder = $CurrentPath === "/";
$: prog = ($Progress * 100.0).toFixed();

const goUp = () => {
    dispatch('go-up', $CurrentPath);
}
const downloadClick = () => {
    dispatch('download', $CurrentPath);
}
const uploadClick = () => {
    dispatch('upload', $CurrentPath);
}
const newFolderClick = () => {
    dispatch('new-folder-dialog', $CurrentPath);
}
</script>

<div class="search">
    <input class="input-path" placeholder="Path..." bind:value={$CurrentPath} />    
    <!-- <button on:click={handleClick}>Filter</button> -->
    <button on:click={goUp} disabled={inRootFolder}>Go Up</button>
    <button on:click={downloadClick} disabled={!$FileRequested || isDownloading}>Download</button>
    <button on:click={uploadClick} disabled={isUploading}>Upload</button>
    <button on:click={newFolderClick}>New Folder</button>
    <div class="w100"></div>
    {#if $Progress > 0 }
    <div>{prog}%</div>
    {/if}
    <div class="totalItems">[{totalFiles} items]</div>
</div>

<style>
.input-path {
    box-sizing: border-box;
    width: 400px;
}
.search {
    display: flex;
    align-items: center;
    gap: 5px;
    margin: 15px;
    margin-bottom: 0px;
}
.totalItems {
    /* display:flex; */
    white-space:nowrap;
}
</style>