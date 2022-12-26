<script>
// @ts-nocheck

import {createEventDispatcher} from 'svelte'
import {Dropdown, DropdownItem, DropdownMenu, DropdownToggle, } from 'sveltestrap';
import folderIcon from "../assets/folder.png";
import folderLinkIcon from "../assets/folder-link.png";
import fileIcon from "../assets/file.png";
import fileLinkIcon from "../assets/file-link.png";
import {CurrentPath} from '../js/store'
import {humanFileSize} from '../js/util'

export let file = {};
export let isLoading = false;

const dispatch = createEventDispatcher();

const fileClick = (file) => {
    $CurrentPath = file.path;
    dispatch('file-click', file);
}
const fileDelete = (file) => {
    dispatch('file-delete', file);
}
const fileRename = (file) => {
    dispatch('file-rename', file);
}

const filesize = () => {
    return file.is_dir ? "" : humanFileSize(file.size, true);
}
const filemodified = () => {
    return file.modified===0 ? "" : file.modified;
}

</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if Object.keys(file).length > 0}
<div class="file" class:selected={$CurrentPath===file.path} class:blur={isLoading}>
    <img class="icon" 
        src={file.is_dir && !file.is_link ? folderIcon 
            : file.is_dir && file.is_link ? folderLinkIcon
            : !file.is_dir && file.is_link ? fileLinkIcon 
            : fileIcon}
        alt="file icon" />
    <span class="filename" on:click={() => fileClick(file)}> 
        {file.is_link ? `${file.name} => ${file.link_path}` : file.name}</span>
    <span class="filesize">{filesize()}</span>
    <span class="filemodified">{filemodified()}</span> 
    <span>
        <Dropdown>
            <DropdownToggle class="btn btn-light">
                <i class="bi-three-dots"/></DropdownToggle>
          <DropdownMenu>
            <!-- <DropdownItem header>{file.name}</DropdownItem> -->
            <DropdownItem on:click={()=> fileDelete(file)}>
                <i class="bi-trash rp10"/>Delete</DropdownItem>
            <DropdownItem on:click={()=> fileRename(file)}>
                <i class="bi-pencil rp10"/>Rename</DropdownItem>
                
          </DropdownMenu>
        </Dropdown>
    </span>
</div>
{:else}
    <div class="empty"><p class="opaque"></p></div>
{/if}

<style>
.icon {
    width: 22px;
    height: 22px;
}
.file {
    cursor: pointer;
    display: flex;
    flex-wrap: nowrap;
    align-items: center;
    gap: 20px;
    padding: 10px;
    border-bottom: 1px solid gainsboro;  
}
.file:hover {
    background-color: whitesmoke;
}
.selected {
    font-weight: bold;
}
.filename {
    width: 100%;
}
.filesize {
    white-space: nowrap;
}
.filemodified {
    white-space: nowrap;
}
.empty {
    display: flex;
    flex-wrap: nowrap;
    padding: 10px;
    
}
.opaque {
    margin: 10px;
    width: 100%;
    background-color: #fafafa;
    height: 2em;
}
.blur {
  filter: blur(1px);
}
</style>