<script lang="ts">

    import {createDino} from "../../api";
    import {DinoKinds} from "./types";
    import Dino from "../../components/Dino.svelte";

    let name = $state("");
    let dinoKind = $state("");
</script>

<div>
    <div class="flex flex-row justify-center my-2">
        <h2>Create Dino</h2>
    </div>

    <div class="grid grid-cols-2 gap-2">
        <div class="flex flex-col items-center h-32">
            <input type="text" placeholder="Name" class="input my-2" bind:value={name} required/>

            <select class="select my-2" name="Dino Kind" bind:value={dinoKind}>
                <option disabled selected value="">What kind of dinosaur?</option>
                {#each DinoKinds as kind}
                    <option>{kind}</option>
                {/each}
            </select>
        </div>
        <div class="flex flex-col h-32 items-center justify-center">
            {#if name === ""}
                <p>Give your Dino a name!</p>
            {:else if dinoKind === ""}
                <p>and a kind</p>
            {:else}
                <Dino authoredDino={{
        dino: {
          name: name,
          dino_kind: {
            type: dinoKind
          }
        }
      }}/>
            {/if}

        </div>
    </div>

    <div class="flex flex-row justify-center my-2">
        <button class="btn btn-primary" disabled={name === "" || dinoKind === ""} onclick={() => createDino({
        name: name,
        dino_kind: {
          type: dinoKind
        },
      })}>
            Create Dino
        </button>
    </div>
</div>
