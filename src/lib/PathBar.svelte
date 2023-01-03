<script>
// @ts-nocheck

import AutoComplete from "simple-svelte-autocomplete"

let selectedPath;
let items = [];

async function getItems(keyword) {
    console.log('keyword: ', keyword);
    let last = keyword[keyword.length-1];
    console.log('last: ', last);

    if (last === '/') {
        console.log('getting items for ', keyword);
        items = ['/red/ssd','/red/share','/red/bin'];
    } 
    return items;
}

</script>

<AutoComplete  
    className="pathbar"
    cleanUserText={false}
    localFiltering={true}
    bind:selectedItem="{selectedPath}" 
    searchFunction="{getItems}"
    delay="500" />


<style>
.pathbar {
  margin-left: 15px;
  margin-top: 0px;
  margin-bottom: 5px;
  margin-right: 15px;
}
</style>
