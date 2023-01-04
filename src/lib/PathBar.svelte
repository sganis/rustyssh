<script>
// @ts-nocheck
import { invoke } from "@tauri-apps/api/tauri"
import AutoComplete from "simple-svelte-autocomplete"
import {createEventDispatcher} from 'svelte'
const dispatch = createEventDispatcher();

let selectedPath;
let items = [];
let hidden = true;

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
    console.log('keyword: ', keyword);
    let last = keyword[keyword.length-1];
    console.log('last: ', last);

    if (last === '/') {
        console.log('getting items for ', keyword);
        items = await getFolders(keyword);
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
    className="pathbar"
    cleanUserText={false}
    localFiltering={true}
    bind:selectedItem="{selectedPath}" 
    searchFunction="{getItems}"
    onChange={onChange}
    delay="500" />


<style>
.pathbar {
  margin-left: 15px;
  margin-top: 0px;
  margin-bottom: 5px;
  margin-right: 15px;
}
</style>
