<script>
export let totalFiles = 0;
export let currentPath = "/";

const handleClick = () => {
    console.log('clicked')
}

</script>

<div class="search">
    <input class="input-path" placeholder="Path..." bind:value={currentPath} />    
    <button on:click={handleClick}>Filter</button>
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