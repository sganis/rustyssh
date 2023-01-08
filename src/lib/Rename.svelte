<script>
// @ts-nocheck
import { scale } from 'svelte/transition';
import renameIcon from "../assets/rename.png";
import {createEventDispatcher} from 'svelte'
const dispatch = createEventDispatcher();


export let file = {}


const cancelRename = () => {
    dispatch("cancel-rename");
}

</script>

<div class="rename">
    <div class="innerrename">
        <img class="icon" src={renameIcon} alt="rename icon" />
        <input class="rename-input" value={file.name} />
        <div class="rename-btn" in:scale="{{duration: 200}}">
            <button class="btn btn-success" on:click={(e)=>fileRename(e,file)}>Rename</button>
            <button class="btn btn-secondary" on:click={cancelRename}>Cancel</button>
        </div>
    </div>
</div>

<style>
.icon {
    width: 22px;
    height: 22px;
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
.rename-input {
    width: 100%;
    padding: 5px;
}
.btn {
    width: 90px;
    padding: 5px;
}

</style>