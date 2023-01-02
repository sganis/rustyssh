<script>
// @ts-nocheck

import {PageStore, CurrentPath} from '../js/store'
import JsonEditor from './JsonEditor.svelte';
import CodeEditor from './CodeEditor.svelte';


let languages = {yml: 'yaml', toml: 'toml', py: 'python', xml: 'xml'};
let images = ['jpg', 'png'];
let lang = 'yaml';
let isJson = false;
let isImage = false;
let isCode = false;

$: ext =  $CurrentPath.split('.').pop();
$: {
    isJson =  ext == 'json';
    isCode = ext in languages;
    isImage = images.includes(ext);
    console.log('image: ', isImage);

    if (isCode) {
        lang = languages[ext];
        console.log('language detected: ', lang);
    }
}

</script>
    {#if isJson}
        <JsonEditor data={JSON.parse($PageStore)} />
    {:else if isCode}
        <CodeEditor {lang} data={$PageStore} />   
    {:else if isImage}
        <!-- <img src={$CurrentPath} />    -->
        <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4
  //8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==" alt="Red dot" />
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