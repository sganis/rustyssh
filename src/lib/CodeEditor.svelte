<script>
// @ts-nocheck
import { AceEditor } from "svelte-ace";
import "brace/mode/toml";
import "brace/mode/yaml";
import "brace/mode/python";
import "brace/mode/xml";
import "brace/theme/eclipse";
//import {onDestroy, onMount} from 'svelte'

export let data;
export let lang;

let theme = 'eclipse';
let options = {
    fontSize: "12pt",
};

</script>
    
<div class="editor border" >
    <AceEditor
        on:selectionChange={(obj) => console.log(obj.detail)}
        on:paste={(obj) => console.log(obj.detail)}
        on:input={(obj) => console.log(obj.detail)}
        on:focus={() => console.log('focus')}
        on:documentChange={(obj) => console.log(`document change : ${obj.detail}`)}
        on:cut={() => console.log('cut')}
        on:cursorChange={() => console.log('cursor change')}
        on:copy={() => console.log('copy')}
        on:init={(editor) => console.log(editor.detail)}
        on:commandKey={(obj) => console.log(obj.detail)}
        on:changeMode={(obj) => console.log(`change mode : ${obj.detail}`)}
        on:blur={() => console.log('blur')}
        lang={lang}
        theme={theme}
        options={options}
        value={data} />
</div>
    
<style>
.editor {
    overflow-y: auto;
    height: 100vh;
    margin: 15px;
    margin-top: 0px;
    --jse-theme-color: darkblue;
    --jse-theme-color-highlight: #687177;
}
.unsaved {
    --jse-theme-color: green;
}
</style>