<script>
// @ts-nocheck

import {PageStore, CurrentPath} from '../js/store'
import JsonEditor from './JsonEditor.svelte';

$: isJson =  $CurrentPath.split('.').pop() == 'json';

</script>

    {#if isJson}
        <JsonEditor data={JSON.parse($PageStore)} />
    {:else}
    <div class="top border">
        <textarea spellcheck="false">{$PageStore}</textarea>
    </div>
    {/if}

<style>
.top {
    height: 100vh; 
    margin: 15px;
    margin-top: 0px;
}
textarea {
    /* margin: 0; */
    padding: 10px;
    border: 0;
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    outline: none;
    resize: none;
}
</style>