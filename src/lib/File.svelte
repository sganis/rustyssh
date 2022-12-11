<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import {createEventDispatcher} from 'svelte'
import folderIcon from "../assets/folder.png";
import folderLinkIcon from "../assets/folder.png";
import fileIcon from "../assets/file.png";
import fileLinkIcon from "../assets/file-link.png";
import {FileStore, CurrentPath} from '../js/store'
import {humanFileSize} from '../js/util'

export let file = {};

const dispatch = createEventDispatcher();

const fileClick = (file) => {
    //dispatch('clear-selection');
    //file.selected = !file.selected;
    $CurrentPath = file.path;
    if (file.filetype==="DIR")
        dispatch('file-click', file.path);
}

const filesize = () => {
    return file.filetype==="DIR" ? "" 
    : humanFileSize(file.size, true);
}
const filemodified = () => {
    return file.modified===0 ? "" : file.modified;
}

</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="file" class:selected={$CurrentPath===file.path} 
    on:click={() => fileClick(file)}>
    <img class="icon" 
        src={file.filetype==="DIR"? folderIcon 
            : file.filetype==="LINK" ? fileLinkIcon
            : fileIcon}
        alt="file icon" />
    <span class="filename"> {file.filetype==="LINK"? `${file.name} => ${file.link}` : file.name}</span>
    <span class="filesize">{filesize()}</span>
    <span class="filemodified">{filemodified()}</span> 
</div>

<style>
.icon {
    width: 30px;
    height: 30px;
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