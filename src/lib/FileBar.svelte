<script>
import {createEventDispatcher} from 'svelte'
import {
	Button,
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
  } from 'sveltestrap';

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
    <!-- <input  placeholder="Path..." bind:value={$CurrentPath} />     -->
    <!-- <button on:click={handleClick}>Filter</button> -->
    <button class="btn btn-light border text-nowrap" on:click={goUp} disabled={inRootFolder}>Go Up</button>
    <button class="btn btn-light border" on:click={downloadClick} disabled={!$FileRequested || isDownloading}>Download</button>
    <button class="btn btn-light border" on:click={uploadClick} disabled={isUploading}>Upload</button>
    
    <Dropdown>
        <DropdownToggle caret class="btn btn-light border">Create</DropdownToggle>
      <DropdownMenu>
        <!-- <DropdownItem divider /> -->
        <DropdownItem on:click={newFolderClick}>Folder</DropdownItem>
        <DropdownItem >Shortcut</DropdownItem>
      </DropdownMenu>
    </Dropdown>
    <!-- <button>
        <svg viewBox="0 0 20 20" fill="none" width="20" focusable="false">
            <path d="m20.836 9.911-6.105-.886L12 3.492 9.27 9.025l-6.106.886 4.43 4.319-.833 6.164L12 17.442l5.239 2.95-.834-6.162 4.43-4.319ZM12 15.72l-3.35 1.886.531-3.924-2.794-2.723 3.878-.564L12 6.882l1.735 3.514 3.878.563-2.795 2.724.532 3.924L12 15.72Z" fill="red"></path></svg>
    </button>
    <button>
        <svg viewBox="0 0 24 24" fill="none" width="16" height="16" focusable="false">
            <path d="m5.25 9.25 6.5 6.25 6.5-6.25" stroke="currentColor" stroke-width="1.5" stroke-miterlimit="10"></path></svg>
    </button> -->
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