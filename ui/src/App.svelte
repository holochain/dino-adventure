<script lang="ts">
    import type {AppClient, HolochainError} from "@holochain/client";
    import logo from "./assets/holochainLogo.svg";

    import {getAllDinos, getIsConnected, setIsConnected} from "./api";

    let client: AppClient | undefined;
    let error: HolochainError | undefined;
    let loading = false;

    let dinos = $state(getAllDinos());

    let connected = getIsConnected();
</script>

<main>
    {#await dinos}
        <p>loading...</p>
    {:then dinoList}
        <ul>
            {#each dinoList as dino}
                <li>{dino.name}</li>
            {/each}
        </ul>
    {:catch _error}
        <p>Failed to load Dinos!</p>
    {/await}

    {#if connected}
        <p>Client is connected.</p>
    {:else}
        <p>connecting...</p>
    {/if}

    <button onclick={() => setIsConnected(!connected)}>Disconnect</button>

    <div>
        <a href="https://developer.holochain.org/get-started/" target="_blank">
            <img src={logo} class="logo holochain" alt="holochain logo"/>
        </a>
    </div>
    <h1>Holochain Svelte hApp</h1>
    <div>
        <div class="card">
            {#if loading}
                <p>connecting...</p>
            {:else if error}
                <p>{error.message}</p>
            {:else}
                <p>Client is connected.</p>
            {/if}
        </div>
        <p>
            Import scaffolded components into <code>src/App.svelte</code> to use your
            hApp
        </p>
        <p class="read-the-docs">Click on the Holochain logo to learn more</p>
    </div>
</main>

<style>
    .logo {
        height: 15em;
        padding: 1.5em;
        will-change: filter;
        transition: filter 300ms;
        width: auto;
    }

    .logo:hover {
        filter: drop-shadow(0 0 2em #646cffaa);
    }

    .logo.holochain:hover {
        filter: drop-shadow(0 0 2em #61dafbaa);
    }

    .card {
        padding: 2em;
    }

    .read-the-docs {
        color: #888;
    }
</style>
