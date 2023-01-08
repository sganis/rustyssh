<script>
// @ts-nocheck
import { scale } from 'svelte/transition';
import Spinner from "$lib/Spinner.svelte";
//import renameIcon from "../assets/rename.png";
import {createEventDispatcher, onMount} from 'svelte'

const dispatch = createEventDispatcher();

export let file = {}
export let icon = {}
let isRenaming = false;
let newName = file.name;

onMount(async () => {
    console.log('mounted')
});

const cancelRename = () => {
    dispatch("cancel-rename");
}
const init = (e) => {
    e.focus()
}
const fileRename = (file) => {
    isRenaming = true;
    console.log('renaming: ', file.name, newName)
    dispatch("file-rename", [file, newName]);
}

</script>

<div class="rename">
    <div class="innerrename">
        {#if isRenaming}
            <div class="spinner">
                <Spinner /> 
            </div>
        {:else}
            <img class="icon" src={icon} alt="rename icon" />
        {/if}
        <form class="rename-form" on:submit={()=>fileRename(file)} >
            <input class="rename-input" 
                bind:value={newName}  
                on:focus="{e => e.target.select()}" 
                use:init 
                spellcheck="false"
                disabled={isRenaming}
                />
            <div class="rename-btn" in:scale="{{duration: 200}}">
                <button type="submit" class="btn btn-success" disabled={isRenaming}>Rename</button>
                <button type="cancel" class="btn btn-secondary" on:click={cancelRename} disabled={isRenaming}>Cancel</button>
            </div>
        </form>
    </div>
</div>

<style>
.icon {
    width: 22px;
    height: 22px;
}
.spinner {
    margin-left: 7px;
}
.rename {
    display: flex;
    flex-wrap: nowrap;
    align-items: center;
    padding: 0px;
    padding-right: 10px;
    margin: 0px;
    border-bottom: 1px solid gainsboro;  
}
.innerrename {
    width: 100%;
    height: 100%;
    padding: 9px;
    padding-right: 0;
    padding-left: 15px;
    margin: 0;
    display: flex;
    flex-wrap: nowrap;
    align-items: center;
    gap: 20px;
}
.rename-btn {
    display: flex;
    gap: 5px;
}
.rename-form {
    display:flex;
    gap: 5px;
    width: 100%;
}
.rename-input {
    width: 100%;
    padding: 5px;
}
.btn {
    width: 90px;
    padding: 5px;
}

</style>