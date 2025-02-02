<script lang="ts">
    import logo from "./assets/holochainLogo.svg";
    import {getAllDinos, getIsConnected} from "./api";
    import Connected from "./components/Connected.svelte";
    import CreateDino from "./dino_adventure/dino_adventure/CreateDino.svelte";
    import Dino from "./components/Dino.svelte";

    let dinos = $state(getAllDinos());
</script>

<main>
    <img class="w-dvw fixed h-dvh -z-10 opacity-10" alt="" src={logo} />

    <CreateDino />

    {#await dinos}
        <p>loading...</p>
    {:then dinoList}
            {#each dinoList as dino}
                <li><Dino authoredDino={dino} /></li>
            {/each}
    {:catch _error}
        <p>Failed to load Dinos!</p>
    {/await}

    <a href="https://www.flaticon.com/free-icons/dinosaur" title="dinosaur icons">Dinosaur icons created by max.icons - Flaticon</a>

    <div class="status-position">
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
