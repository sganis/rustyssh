<script>
import {PageStore} from '../js/store'
</script>

<div class="scrollable">
    {@html $PageStore.replaceAll("\n","<br/>")}
</div>

<style>
    .scrollable {
      overflow-y: auto; 
      height: 100vh; 
      scrollbar-gutter: stable;
      margin: 15px;
      padding: 10px; 
    }
</style>