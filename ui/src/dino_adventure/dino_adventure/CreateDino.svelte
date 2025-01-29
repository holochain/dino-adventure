<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Dino, DinoKind } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let name: string = "";
let dinoKind: DinoKind = { type: "Apatosaurus" };

$: name, dinoKind;
$: isDinoValid = true && name !== "" && true;

onMount(async () => {
  client = await appClientContext.getClient();
});

async function createDino() {
  const dinoEntry: Dino = {
    name: name!,
    dino_kind: dinoKind!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "create_dino",
      payload: dinoEntry,
    });
    dispatch("dino-created", { dinoHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Dino</h3>

  <div>
    <label for="Name">Name</label>
    <input name="Name" bind:value={name} required />
  </div>
  <div>
    <label for="Dino Kind">Dino Kind:</label>
    <select name="Dino Kind" bind:value={dinoKind?.type}>
      <option value="Apatosaurus">Apatosaurus</option>
      <option value="Spinosaurus">Spinosaurus</option>
    </select>
  </div>

  <button disabled={!isDinoValid} on:click={() => createDino()}>
    Create Dino
  </button>
</div>
