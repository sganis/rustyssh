<script>
import {PageStore} from '../js/store'

</script>

<div class="top border">
    <textarea>
    {$PageStore}
    </textarea>
</div>

<style>
.top {
    height: 100vh; 
    margin: 15px;
    margin-top: 0px;
}
textarea {
    margin: 0;
    padding: 10px;
    border: 0;
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    outline: none;
    resize: none;
}
</style>