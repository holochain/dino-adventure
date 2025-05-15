<script lang="ts">
  import { getIsConnected } from "../api";
</script>

<div
  class="tooltip tooltip-left flex flex-col justify-center"
  data-tip="Holochain connection"
>
  <div
    class={[
      "w-3",
      "h-3",
      "rounded-full",
      getIsConnected() ? "bg-green-300" : "bg-red-300",
    ]}
  ></div>
</div>
