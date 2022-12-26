<script>
// @ts-nocheck

import {PageStore} from '../js/store'
import JsonEditor from './JsonEditor.svelte';

export let isJson = true;


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