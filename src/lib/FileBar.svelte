<script>
import {createEventDispatcher} from 'svelte'
import {CurrentPath, Progress} from '../js/store'
export let totalFiles = 0;

const dispatch = createEventDispatcher();

$: inRootFolder = $CurrentPath === "/";
$: prog = ($Progress * 100.0).toFixed();

const handleClick = () => {
    console.log('clicked')
}
const goUp = () => {
    dispatch('go-up', $CurrentPath);
}
const downloadClick = () => {
    dispatch('download', $CurrentPath);
}

</script>

<div class="search">
    <input class="input-path" placeholder="Path..." bind:value={$CurrentPath} />    
    <!-- <button on:click={handleClick}>Filter</button> -->
    <button on:click={goUp} disabled={inRootFolder}>Go Up</button>
    <button on:click={downloadClick} disabled={$Progress > 0}>Download</button>
    <div class="w100"></div>
    {#if $Progress > 0}
    <div>{prog}%</div>
    {/if}
    <div class="totalItems">Items: {totalFiles}</div>
</div>

<style>
.input-path {
    box-sizing: border-box;
    width: 400px;
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