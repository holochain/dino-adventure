<script lang="ts">
  import {
    getAllAdventures,
    getDinoState,
    getNestBatchesForAdventure,
  } from "../api";
  import {
    type ActionHash,
    type ActionHashB64,
    type AgentPubKeyB64,
    encodeHashToBase64,
  } from "@holochain/client";
  import Dino from "./Dino.svelte";
  import type { AdventureParticipantSummary } from "../types";
  import ParticipantSummary from "./ParticpantSummary.svelte";

  const adventureHistory = $derived.by(() => {
    let all = Object.values(getAllAdventures());

    return all.sort((a, b) => {
      return a.created_at - b.created_at;
    });
  });

  const nestInfo: Record<
    ActionHashB64,
    Record<AgentPubKeyB64, AdventureParticipantSummary>
  > = $state({});

  const refreshNestsForAdventure = async (address: ActionHash) => {
    try {
      const batches = await getNestBatchesForAdventure(address);

      console.log("batches", batches);

      if (!(encodeHashToBase64(address) in nestInfo)) {
        nestInfo[encodeHashToBase64(address)] = {};
      }

      for (const batch of batches.nest_batches) {
        nestInfo[encodeHashToBase64(address)][
          encodeHashToBase64(batch.author)
        ] = {
          batchCount: batches.nest_batches.filter(
            (b) =>
              encodeHashToBase64(b.author) === encodeHashToBase64(batch.author),
          ).length,
          nestCounts: batches.nest_batches
            .filter(
              (b) =>
                encodeHashToBase64(b.author) ===
                encodeHashToBase64(batch.author),
            )
            .map((batch) => {
              return (
                batches.nests[encodeHashToBase64(batch.address)]?.length || 0
              );
            }),
        };
      }
    } catch (error) {
      console.error("Failed to refresh nests for adventure", error);
    }
  };
</script>

<div class="flex flex-col items-center">
  <p class="text-xl">All adventures</p>
  {#each adventureHistory as adventure}
    <div class="card bg-base-100 w-11/12 shadow-lg my-5">
      <div class="card-body">
        <div class="card-title">
          Adventure from {new Date(
            adventure.created_at / 1000,
          ).toLocaleString()}
        </div>

        <p class="text-lg">Chief adventurer</p>
        <div class="flex flex-row justify-center">
          {#if getDinoState()[encodeHashToBase64(adventure.author)]}
            <Dino
              authoredDino={getDinoState()[
                encodeHashToBase64(adventure.author)
              ]}
            />
          {:else}
            <p>{encodeHashToBase64(adventure.author)}</p>
          {/if}

          {#if nestInfo[encodeHashToBase64(adventure.address)] && nestInfo[encodeHashToBase64(adventure.address)][encodeHashToBase64(adventure.author)]}
            <ParticipantSummary
              participantSummary={nestInfo[
                encodeHashToBase64(adventure.address)
              ][encodeHashToBase64(adventure.author)]}
            />
          {/if}
        </div>

        <p class="text-lg">Adventure participants</p>
        <div class="grid grid-cols-3">
          {#each adventure.adventure.participants.filter((p) => encodeHashToBase64(p) !== encodeHashToBase64(adventure.author)) as participant}
            {#if getDinoState()[encodeHashToBase64(participant)]}
              <Dino
                authoredDino={getDinoState()[encodeHashToBase64(participant)]}
              />
            {:else}
              <p>{encodeHashToBase64(participant)}</p>
            {/if}
          {/each}
        </div>

        <div class="card-actions justify-end">
          <button
            class="btn btn-primary"
            aria-label="Refresh nests"
            onclick={async () =>
              await refreshNestsForAdventure(adventure.address)}
            >Refresh nests</button
          >
        </div>
      </div>
    </div>
  {/each}
</div>
