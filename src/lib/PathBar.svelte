<script>
// @ts-nocheck

    import { audioDir } from "@tauri-apps/api/path";

// @ts-nocheck
import { invoke } from "@tauri-apps/api/tauri"
import AutoComplete from "simple-svelte-autocomplete"
import {createEventDispatcher} from 'svelte'
const dispatch = createEventDispatcher();

let items = [];
let hidden = true;
let lastFolder = "";
export let value;

const getFolders = async (path) => {
    try {
      const r = await invoke("get_folders", { path, hidden });
      return JSON.parse(r);      
    } catch (e) {
      console.log(e);
      return [];      
    }
}

async function getItems(keyword) {
    let index = keyword.lastIndexOf('/');
    if (index == 0) 
        index = 1;
    let folder = keyword.substring(0,index);
    if (folder !== lastFolder) {
        console.log('getting items for ', folder);
        items = await getFolders(folder);
        lastFolder = folder;
    }
    return items;
}

function onChange(path) {
    console.log('on change: ', path);
    dispatch('path-changed', path);
    return true;
}
</script>

<AutoComplete  
    cleanUserText={false}
    localFiltering={true}
    bind:selectedItem="{value}" 
    searchFunction="{getItems}"
    onChange={onChange}
    html5autocomplete={false}
    delay="500" />


<style>
.pathbar {
    border: 1px solid green;
}
.autocomplete {
    color: red;
  }

</style>
