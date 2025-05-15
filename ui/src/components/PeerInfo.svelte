<script lang="ts">
  import { getPeerInfo, type PeerInfo } from "../api";

  let peerInfo = $state<PeerInfo | null>(null);

  setInterval(() => {
    getPeerInfo()
      .then((info) => {
        peerInfo = info;
      })
      .catch((e) => {
        console.error(e);
      });
  }, 1000);
</script>

<div class="peer-into-container">
  {#if peerInfo == null}
    <p>Checking peer connections...</p>
  {:else}
    <p
      class="tooltip tooltip-left"
      data-tip={`Direct connections: ${peerInfo.direct_connected_peers}, all: ${peerInfo.connected_peers}`}
    >
      {peerInfo.direct_connected_peers}
      / {peerInfo.connected_peers}
    </p>
  {/if}
</div>

<style>
  .peer-into-container {
    cursor: default;
  }
</style>
