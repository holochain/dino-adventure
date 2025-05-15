<script lang="ts">
import {getFetchQueueCount} from "../api";
</script>


<p class="tooltip tooltip-left cursor-default" data-tip="Outstanding ops to fetch">{ getFetchQueueCount() }</p>
