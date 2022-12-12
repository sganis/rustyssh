<script>
// @ts-nocheck

import { invoke } from "@tauri-apps/api/tauri"
import {createEventDispatcher} from 'svelte'
import folderIcon from "../assets/folder.png";
import folderLinkIcon from "../assets/folder-link.png";
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
    dispatch('file-click', file);
}

const filesize = () => {
    return file.is_dir ? "" : humanFileSize(file.size, true);
}
const filemodified = () => {
    return file.modified===0 ? "" : file.modified;
}

</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="file" class:selected={$CurrentPath===file.path} 
    on:click={() => fileClick(file)}>
    <img class="icon" 
        src={file.is_dir && !file.is_link ? folderIcon 
            : file.is_dir && file.is_link ? folderLinkIcon
            : !file.is_dir && file.is_link ? fileLinkIcon 
            : fileIcon}
        alt="file icon" />
    <span class="filename"> {file.is_link ? `${file.name} => ${file.link_path}` 
                                        : file.name}</span>
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