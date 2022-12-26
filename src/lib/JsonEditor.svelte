<script>
// @ts-nocheck
import {JSONEditor} from 'svelte-jsoneditor'
import {JsonChanged,JsonData,JsonNewData} from '../js/store'
import {onDestroy, onMount} from 'svelte'
export let data;
let mode = 'tree'
let mainMenuBar = true;
let navigationBar = true;

let content = {
  text: undefined,
  json: data,
  
}
onMount(async () => {
  $JsonChanged = false;
});
onDestroy(() => {
  // if ($JsonChanged) {
  //   if (confirm("Save changes?")===true) {
  //     console.log('saving...')
  //   } else {
  //     console.log('canceled')
  //   }
  // }
  $JsonChanged = false;
});

const onChange = (updatedContent, previousContent, { contentErrors, patchResult }) => {
  // content is an object { json: JSONValue } | { text: string }
  console.log(updatedContent);
  let json = "";
  if ('text' in updatedContent && updatedContent['text'] !== undefined)  {
    let text = updatedContent['text'];
    //console.log(text);
    try {
    json = JSON.parse(text);
    } catch(e) {
      console.log(`Invalid json: ${text}`)
    }
    //console.log("text json: "+ json)
  } else {
    json = updatedContent['json'];
  }
  var s1 =  JSON.stringify($JsonData);
  var s2 =  JSON.stringify(json);
  if (s1 !== s2) {
    $JsonChanged = true;
    $JsonNewData = json;
    console.log(json);
  } else {
    $JsonChanged = false;
  }
}
// const onRenderMenu = (mode, items) => {
//   //let mode1: 'tree' | 'text';
//   // if (mode === 'tree') {
//   //   console.log(items);
//   //   let tree = items.filter(e => e.text === 'tree'); 
//   //   tree.className = 'jse-group-button jse-last';
//   //   items = items.filter(e => e.text !== 'table');
//   //   return items;
//   // }
// }
</script>

<div class="editor" class:unsaved={$JsonChanged}>
  <JSONEditor bind:content {mode} {mainMenuBar} {navigationBar} {onChange}/>
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