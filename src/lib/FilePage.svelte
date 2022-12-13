<script>
import {PageStore} from '../js/store'
</script>

<div class="scrollable monospace">
    {@html $PageStore.replaceAll("\n","<br/>")}
</div>

<style>
    .monospace {
        font-family: monospace;
    }
</style>