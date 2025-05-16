<script lang="ts">
  import logo from "./assets/holochainLogo.svg";
  import {
    getMyAdventuresState,
    getAgentPubKeyB64,
    getDinosFirstLoaded,
    getDinoState,
    getMyLatestAdventuresState,
  } from "./api";
  import Connected from "./components/Connected.svelte";
  import CreateDino from "./dino_adventure/dino_adventure/CreateDino.svelte";
  import DinoGathering from "./components/DinoGathering.svelte";
  import { encodeHashToBase64 } from "@holochain/client";
  import ConnectionsState from "./components/ConnectionsState.svelte";
  import FetchCount from "./components/FetchCount.svelte";
  import AdventureAssembly from "./components/AdventureAssembly.svelte";
  import MyArc from "./components/MyArc.svelte";

  let thisAgentHasNoDinos = $derived(
    Object.values(getDinoState()).find(
      (d) => encodeHashToBase64(d.author) === getAgentPubKeyB64(),
    ) === undefined,
  );
</script>

<main>
  <img class="w-dvw fixed h-dvh -z-10 opacity-10" alt="" src={logo} />

  {#if !getDinosFirstLoaded()}
    <div class="flex flex-row w-full justify-center items-center h-dvh">
      <p class="text-2xl">Preparing your adventure!</p>
    </div>
  {:else if thisAgentHasNoDinos}
    <CreateDino />
  {:else if !!getMyLatestAdventuresState()}
    <p>Live adventure!</p>
    <button class="btn btn-ghost">End adventure</button>
  {:else}
    <DinoGathering />
    {#if Object.values(getDinoState()).length === 1}
      <div class="w-full flex flex-row justify-center">
        <p>Waiting for more Dinos to gather</p>
      </div>
    {:else if Object.values(getDinoState()).length > 1}
      <AdventureAssembly />
    {/if}
  {/if}

  {#each Object.values(getMyAdventuresState()) as adv}
    <p>
      {adv.adventure.participants.map((p) => encodeHashToBase64(p)).join(", ")}
      {adv.address}
    </p>
  {/each}

  <a
    class="absolute bottom-2 left-2"
    href="https://www.flaticon.com/free-icons/dinosaur"
    title="dinosaur icons">Dinosaur icons created by max.icons - Flaticon</a
  >

  <div class="flex flex-row gap-3 status-position">
    <MyArc />
    <FetchCount />
    <ConnectionsState />
    <Connected />
  </div>
</main>

<style>
  .status-position {
    position: fixed;
    bottom: 1em;
    right: 1em;
  }
</style>
