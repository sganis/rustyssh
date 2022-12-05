<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {createEventDispatcher} from 'svelte'
    import folderIcon from "../assets/folder.png";
    import fileIcon from "../assets/file.png";
    import linkIcon from "../assets/link.png";

    export let file = {};
    
    let greetMsg = ""

    const dispatch = createEventDispatcher();

    async function greet(){
      greetMsg = await invoke("greet", { name })
    }

    const fileClick = (file) => {
        dispatch('clear-selection');
        file.selected = !file.selected;
        dispatch('file-click', file.path);
    }

</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="file" on:click={() => fileClick(file)}>
    <img class="icon" 
        src={file.filetype=="DIR"? folderIcon 
            : file.filetype=="LINK" ? linkIcon
            : fileIcon}
        alt="file icon" />
    <span class="filename"> {file.name}</span>
    <span class="filesize">{file.size}</span>
    <span class="filemodified">{file.modified}</span> 
</div>

<style>
.icon {
    width: 20px;
    height: 20px;
}
.file {
    cursor: pointer;
    display: flex;
    flex-wrap: nowrap;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-bottom: 1px solid gainsboro;  
}
.file:hover {
    background-color: whitesmoke;
}

.filename {
    width: 100%;
}
.filemodified {
    white-space: nowrap;
}
</style>