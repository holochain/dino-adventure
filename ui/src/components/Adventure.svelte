<script lang="ts">
  import {
    getAgentPubKeyB64,
    getDinoState,
    getMyLatestAdventure,
    createNest,
    createNestBatch,
    getNestBatchesWithNests,
    getIncomingPings,
    getIncomingPongs,
    getSentPings,
    getWantedSentPings,
    rawrAtAgent,
  } from "../api";
  import {
    type AgentPubKeyB64,
    decodeHashFromBase64,
    encodeHashToBase64,
  } from "@holochain/client";
  import Dino from "./Dino.svelte";
  import { onDestroy, onMount } from "svelte";
  import type { NestBatchesWithNests } from "../types";

  const otherDinoNames = $derived.by(() => {
    const adventureState = getMyLatestAdventure();
    if (!adventureState) {
      return {};
    }

    const dinoState = getDinoState();

    return adventureState.adventure.participants.reduce(
      (acc, p) => {
        const participantPubKeyB64 = encodeHashToBase64(p);
        if (
          participantPubKeyB64 in dinoState &&
          participantPubKeyB64 != getAgentPubKeyB64()
        ) {
          acc[participantPubKeyB64] = dinoState[participantPubKeyB64].dino.name;
        }

        return acc;
      },
      {} as Record<AgentPubKeyB64, string>,
    );
  });

  let selectedRawr = $state("");
  let rawrCount = $state<number>(30);

  const startRawr = (event: SubmitEvent) => {
    event.preventDefault();

    const useSelectedRawr = selectedRawr;
    const useRawrCount = rawrCount;

    if (!useSelectedRawr) {
      console.error("No dino selected");
      return;
    }

    rawrAtAgent({
      to_agent: decodeHashFromBase64(useSelectedRawr),
      times: useRawrCount,
      interval_ms: 500,
    });
  };

  const dinoRawrCardState = $derived.by(() => {
    let adventureState = getMyLatestAdventure();
    let dinoState = getDinoState();
    let sentPings = getSentPings();
    let wantedSentPings = getWantedSentPings();
    let incomingPings = getIncomingPings();
    let incomingPongs = getIncomingPongs();

    if (!adventureState) {
      return [];
    }

    return adventureState.adventure.participants
      .map((p) => {
        const participantPubKeyB64 = encodeHashToBase64(p);
        if (participantPubKeyB64 === getAgentPubKeyB64()) {
          return;
        }

        const dino = dinoState[participantPubKeyB64];
        if (dino) {
          return {
            dino: dino,
            sentPings: sentPings[participantPubKeyB64] ?? 0,
            wantedSentPings: wantedSentPings[participantPubKeyB64] ?? 0,
            incomingPings: incomingPings[participantPubKeyB64] ?? 0,
            incomingPongs: incomingPongs[participantPubKeyB64] ?? [],
          };
        }
      })
      .filter((v) => v !== undefined);
  });

  let nestState = $state<Record<AgentPubKeyB64, NestBatchesWithNests>>({});

  interface NestSummary {
    createdAt: string;
    count: number;
    totalSize: number;
  }

  const dinoNestCardState = $derived.by(() => {
    const myLatestAdventures = getMyLatestAdventure();
    if (!myLatestAdventures) {
      return [];
    }

    let dinoState = getDinoState();

    return myLatestAdventures.adventure.participants
      .map((p) => {
        let participantPubKeyB64 = encodeHashToBase64(p);
        if (!(participantPubKeyB64 in dinoState)) {
          return;
        }

        let batches;
        if (participantPubKeyB64 in nestState) {
          batches = nestState[participantPubKeyB64].nest_batches.reduce(
            (acc, batch) => {
              acc.push({
                createdAt: new Date(
                  batch.created_at / 1000,
                ).toLocaleTimeString(),
                count:
                  nestState[participantPubKeyB64].nests[
                    encodeHashToBase64(batch.address)
                  ]?.length ?? 0,
                totalSize:
                  nestState[participantPubKeyB64].nests[
                    encodeHashToBase64(batch.address)
                  ]?.reduce((acc, nest) => {
                    return acc + nest.nest.payload.length;
                  }, 0) ?? 0,
              });

              return acc;
            },
            [] as NestSummary[],
          );
        }

        return {
          dino: dinoState[participantPubKeyB64],
          nestBatches: batches ?? [],
        };
      }, [])
      .filter((v) => v !== undefined);
  });

  let cancelPoll = false;
  const pollForNestBatchesWithNests = async () => {
    if (cancelPoll) {
      return;
    }

    const adventureState = getMyLatestAdventure();
    if (!adventureState) {
      setTimeout(pollForNestBatchesWithNests, 100);
      return;
    }

    Promise.all(
      adventureState.adventure.participants.map((p) =>
        getNestBatchesWithNests(p).then((v) => [encodeHashToBase64(p), v]),
      ),
    )
      .then((r) => {
        nestState = Object.fromEntries(r);

        setTimeout(pollForNestBatchesWithNests, 5000);
      })
      .catch((e) => {
        console.error("Failed to get nest batches with nests", e);
        setTimeout(pollForNestBatchesWithNests, 5000);
      });
  };

  let numNests = $state(10);
  let minNestSize = $state(500);
  let maxNestSize = $state(2000);
  let diggingNests = $state(false);

  const digNests = async (event: SubmitEvent) => {
    event.preventDefault();

    // Guard against accidental double-submit (e.g., Enter key or programmatic calls)
    if (diggingNests) return;

    // Snapshot values to prevent mid-flight changes from affecting this run
    const useNumNests = numNests;
    const useMinNestSize = minNestSize;
    const useMaxNestSize = maxNestSize;

    diggingNests = true;

    try {
      const nestBatch = await createNestBatch();

      const createOne = async (count: number) => {
        if (count <= 0) {
          diggingNests = false;
          return;
        }

        const size =
          Math.round(Math.random() * (useMaxNestSize - useMinNestSize)) +
          useMinNestSize;

        createNest({
          nest_batch_address: nestBatch.address,
          size,
        })
          .then(() => {
            setTimeout(() => createOne(count - 1), 500);
          })
          .catch((e) => {
            console.error("Failed to create nest", e);
            setTimeout(() => createOne(count - 1), 500);
          });
      };

      createOne(useNumNests).catch(console.error);
    } catch (error) {
      console.error("Failed to create nest batch", error);
      diggingNests = false;
    }
  };

  const bytesSize = function (bytes: number) {
    if (bytes === 0) {
      return "0.00 B";
    }

    let e = Math.floor(Math.log(bytes) / Math.log(1024));
    return (
      (bytes / Math.pow(1024, e)).toFixed(2) + " " + " KMGTP".charAt(e) + "B"
    );
  };

  onMount(() => {
    pollForNestBatchesWithNests();
  });

  onDestroy(() => {
    cancelPoll = true;
  });
</script>

<div class="p-2">
  <span class="text-xl">Share the latest news</span>
  <div class="p-2">
    <p>
      <span>
        You're a very important Dino, doing interesting Dino things, and other
        Dinos need to know! RAAAWR your news at them!
      </span>&nbsp;
      <span class="text-gray-400">
        This sends signals to peers and records the round trip time
      </span>
    </p>

    <div class="flex flex-row justify-center" onsubmit={startRawr}>
      <form class="my-5 flex flex-col items-center gap-5 w-1/2">
        <label class="select">
          <span class="label">Who?</span>
          <select class="select" bind:value={selectedRawr} required>
            <option disabled value="">Choose a Dino</option>
            {#each Object.keys(otherDinoNames) as pubKey}
              <option value={pubKey}>{otherDinoNames[pubKey]}</option>
            {/each}
          </select>
        </label>

        <label class="input">
          <span class="label">How many RAWWRs?</span>
          <input
            type="number"
            placeholder="How many RAWWRs?"
            class="input"
            bind:value={rawrCount}
            required
          />
        </label>

        <button type="submit" class="btn btn-primary">RAWR!</button>
      </form>
    </div>
  </div>
</div>

<div class="grid grid-cols-3 justify-items-center w-full">
  {#each dinoRawrCardState as dinoCard}
    <div class="card w-64 shadow-sm">
      <Dino authoredDino={dinoCard.dino} />
      <p>RAAWRs: {dinoCard.sentPings}/{dinoCard.wantedSentPings}</p>
      <p>
        Echoed RAAWRs: {dinoCard.incomingPongs.length > 0
          ? Math.round(
              (dinoCard.incomingPongs.length / dinoCard.sentPings) * 100,
            )
          : 0}%
      </p>
      <p>
        Average RAAWR RTT: {dinoCard.incomingPongs.length > 0
          ? Math.round(
              (dinoCard.incomingPongs.reduce((acc, p) => acc + p, 0) /
                dinoCard.incomingPongs.length) *
                100,
            ) / 100
          : 0} ms
      </p>
      <p class="text-gray-400">Heard raws: {dinoCard.incomingPings}</p>
    </div>
  {/each}
</div>

<div class="p-2">
  <span class="text-xl">Dig nests</span>
  <div class="p-2">
    <p>
      <span>
        So many Dinos, so little time! Help out your fellow Dinos by digging
        nests for them to find.
      </span>&nbsp;
      <span class="text-gray-400">
        This creates a number of entries with random sizes.
      </span>
    </p>

    <div class="flex flex-row justify-center" onsubmit={digNests}>
      <form class="my-5 flex flex-col items-center gap-5 w-1/2">
        <label class="input">
          <span class="label">How many nests?</span>
          <input
            type="number"
            placeholder="How many nests?"
            class="input"
            bind:value={numNests}
            required
            disabled={diggingNests}
            aria-busy={diggingNests}
          />
        </label>

        <label class="input">
          <span class="label">Small nest size?</span>
          <input
            type="number"
            placeholder="Small nest size?"
            class="input"
            bind:value={minNestSize}
            required
            disabled={diggingNests}
            aria-busy={diggingNests}
          />
        </label>

        <label class="input">
          <span class="label">Large nest size?</span>
          <input
            type="number"
            placeholder="Large nest size?"
            class="input"
            bind:value={maxNestSize}
            required
            disabled={diggingNests}
            aria-busy={diggingNests}
          />
        </label>

        <button
          type="submit"
          class="btn btn-primary"
          disabled={diggingNests}
          aria-busy={diggingNests}
        >
          {#if diggingNests}
            <span class="loading loading-spinner loading-sm"></span>
            Digging...
          {:else}
            DIG!
          {/if}
        </button>
      </form>
    </div>
  </div>
</div>

<div class="grid grid-cols-3 justify-items-center w-full">
  {#each dinoNestCardState as dinoCard}
    <div class="card w-64 shadow-sm">
      <Dino authoredDino={dinoCard.dino} />
      {#each dinoCard.nestBatches as batch}
        <div>
          <p>
            Batch created by {dinoCard.dino.dino.name} at {batch.createdAt}. It
            contains {batch.count} nests with total size of {bytesSize(
              batch.totalSize,
            )}
          </p>
        </div>
      {/each}
    </div>
  {/each}
</div>

<div class="w-full h-16"></div>
