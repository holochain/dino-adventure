<script lang="ts">
  import { getPeerConnectionsState } from "../api";
</script>

<div class="cursor-default">
  {#if getPeerConnectionsState() == null}
    <p>Checking peer connections...</p>
  {:else}
    <p
      class="tooltip tooltip-left"
      data-tip={`Direct connections: ${getPeerConnectionsState().directConnectedPeers}, all: ${getPeerConnectionsState().connectedPeers}`}
    >
      {getPeerConnectionsState().directConnectedPeers}
      / {getPeerConnectionsState().connectedPeers}
    </p>
  {/if}
</div>
