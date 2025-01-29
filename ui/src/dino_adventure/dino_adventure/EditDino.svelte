<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Dino, DinoKind } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalDinoHash!: ActionHash;

let currentDino: Dino = decode((currentRecord.entry as any).Present.entry) as Dino;
let name: string | undefined = currentDino.name;
let dinoKind: DinoKind | undefined = currentDino.dino_kind;

$: name, dinoKind;
$: isDinoValid = true && name !== "" && true;

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditDino element`);
  }
  if (!originalDinoHash) {
    throw new Error(`The originalDinoHash input is required for the EditDino element`);
  }
  client = await appClientContext.getClient();
});

async function updateDino() {
  const dino: Dino = {
    name: name!,
    dino_kind: dinoKind!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "update_dino",
      payload: {
        original_dino_hash: originalDinoHash,
        previous_dino_hash: currentRecord.signed_action.hashed.hash,
        updated_dino: dino,
      },
    });

    dispatch("dino-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
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

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isDinoValid} on:click={() => updateDino()}>
      Edit Dino
    </button>
  </div>
</section>
