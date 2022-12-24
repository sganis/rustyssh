<script>
// @ts-nocheck

import {createEventDispatcher} from 'svelte'
import {Dropdown, DropdownItem, DropdownMenu, DropdownToggle, FormGroup, Input } from 'sveltestrap';
import {CurrentPath, FileRequested, Progress} from '../js/store'

export let totalFiles = 0;
export let isDownloading = false;
export let isUploading = false;

const dispatch = createEventDispatcher();

let hidden = true;

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
    dispatch('new-folder', $CurrentPath);
}
const hiddenChanged = () => {
    dispatch('show-hidden', hidden);
}
</script>

<div class="search">
    <!-- <input  placeholder="Path..." bind:value={$CurrentPath} />     -->
    <!-- <button on:click={handleClick}>Filter</button> -->
    <button class="btn btn-light border text-nowrap" 
        on:click={goUp} disabled={inRootFolder}>
        <i class="bi-arrow-up-circle rp10"></i>Up</button>
    <button class="btn btn-light border" 
        on:click={downloadClick} disabled={!$FileRequested || isDownloading}>
        <i class="bi-download rp10"></i>Download</button>
    <button class="btn btn-light border" 
        on:click={uploadClick} disabled={$FileRequested || isUploading}>
        <i class="bi-upload rp10"></i>Upload</button>
    
    <Dropdown>
        <DropdownToggle class="btn btn-light border">
            <i class="bi-three-dots"/></DropdownToggle>
      <DropdownMenu>
        <DropdownItem on:click={newFolderClick}>
            <i class="bi-folder rp10"/>New Folder
        </DropdownItem>
        <!-- <DropdownItem>
           <i class="bi-folder-symlink rp10"/>Create Link
        </DropdownItem> -->
        <!-- <DropdownItem>
            
        </DropdownItem> -->
                        
     </DropdownMenu>
    </Dropdown>
    <span class="text-nowrap">
        <input type="checkbox" bind:checked={hidden} id="chk-hidden" on:change={hiddenChanged}/>
        <label class="form-check-label" for="chk-hidden">
            Hidden
          </label></span>
    <div class="w100"></div>
    {#if $Progress > 0 }
    <div>{prog}%</div>
    {/if}
    <div class="totalItems">[{totalFiles} items]</div>
</div>

<style>

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