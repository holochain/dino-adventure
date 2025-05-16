<script lang="ts">
  import {
    getAgentPubKeyB64,
    getDinoState,
    getMyLatestAdventures,
  } from "../api";
  import {
    type AgentPubKeyB64,
    decodeHashFromBase64,
    encodeHashToBase64,
  } from "@holochain/client";
  import {
    getIncomingPings,
    getIncomingPongs,
    getSentPings,
    getWantedSentPings,
    rawrAtAgent,
  } from "../api/test.svelte";
  import Dino from "./Dino.svelte";

  const otherDinoNames = $derived.by(() => {
    const adventureState = getMyLatestAdventures();
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
  let rawrCount = $state<number | null>(null);

  const startRawr = (event: SubmitEvent) => {
    event.preventDefault();

    const useSelectedRawr = selectedRawr;
    const useRawrCount = rawrCount;

    if (!useSelectedRawr) {
      console.error("No dino selected");
      return;
    }

    if (!useRawrCount) {
      console.error("No rawr count");
      return;
    }

    rawrAtAgent({
      to_agent: decodeHashFromBase64(useSelectedRawr),
      times: useRawrCount,
      interval_ms: 500,
    });
  };

  const dinoCardState = $derived.by(() => {
    let adventureState = getMyLatestAdventures();
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
        <select class="select" bind:value={selectedRawr} required>
          <option disabled value="">Who?</option>
          {#each Object.keys(otherDinoNames) as pubKey}
            <option value={pubKey}>{otherDinoNames[pubKey]}</option>
          {/each}
        </select>

        <input
          type="number"
          placeholder="How many times?"
          class="input"
          bind:value={rawrCount}
          required
        />

        <button type="submit" class="btn btn-primary">RAWR!</button>
      </form>
    </div>
  </div>
</div>

<div class="grid grid-cols-3 justify-items-center w-full">
  {#each dinoCardState as dinoCard}
    {#if encodeHashToBase64(dinoCard.dino.author) !== getAgentPubKeyB64()}
      <div class="card w-64 shadow-sm">
        <Dino authoredDino={dinoCard.dino} />
        <p>RAAWRs: {dinoCard.sentPings}/{dinoCard.wantedSentPings}</p>
        <p>
          Echoed RAAWRs: {dinoCard.incomingPongs.length > 0
            ? Math.round(
                (dinoCard.sentPings / dinoCard.incomingPongs.length) * 100,
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
        <p>Heard raws: {dinoCard.incomingPings}</p>
      </div>
    {/if}
  {/each}
</div>
