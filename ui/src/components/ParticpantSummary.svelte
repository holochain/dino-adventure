<script lang="ts">
  import type { AdventureParticipantSummary } from "../types";

  const {
    participantSummary,
  }: { participantSummary: AdventureParticipantSummary } = $props();
</script>

<div class="m-2">
  <p>
    Created {participantSummary.batchCount} nest batch{participantSummary.batchCount ===
    1
      ? ""
      : "es"}.
  </p>
  {#if participantSummary.nestCounts}
    <p>Number of nests in batches {participantSummary.nestCounts.join(", ")}</p>
  {/if}
</div>
