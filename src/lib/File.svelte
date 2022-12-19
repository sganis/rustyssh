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
<div class="file" class:selected={$CurrentPath===file.path} >
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
                <i class="bi-trash lp10"/>Delete</DropdownItem>
            <DropdownItem on:click={()=> fileRename(file)}>
                <i class="bi-pencil lp10"/>Rename</DropdownItem>
                
          </DropdownMenu>
        </Dropdown>
    </span>
</div>

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
</style>