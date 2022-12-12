<script>
import {createEventDispatcher} from 'svelte'
import {CurrentPath} from '../js/store'
export let totalFiles = 0;

const dispatch = createEventDispatcher();

const handleClick = () => {
    console.log('clicked')
}
const goUp = () => {
    dispatch('go-up', $CurrentPath);
}

</script>

<div class="search">
    <input class="input-path" placeholder="Path..." bind:value={$CurrentPath} />    
    <button on:click={handleClick}>Filter</button>
    <!-- <button on:click={goUp}>Go Up</button> -->
    <div class="w100"></div>
    <div class="totalItems">Items: {totalFiles}</div>
</div>

<style>
.input-path {
    box-sizing: border-box;
    /* max-width: 100%; */
}
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