<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditDino from "./EditDino.svelte";
import type { Dino, DinoKind } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let dino: Dino | undefined;

export let dinoHash: ActionHash;

$: editing, error, loading, record, dino;

onMount(async () => {
  if (dinoHash === undefined) {
    throw new Error(`The dinoHash input is required for the DinoDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchDino();
});

async function fetchDino() {
  loading = true;
  try {
    record = await client.callZome({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "get_latest_dino",
      payload: dinoHash,
    });
    if (record) {
      dino = decode((record.entry as any).Present.entry) as Dino;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteDino() {
  try {
    await client.callZome({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "delete_dino",
      payload: dinoHash,
    });
    dispatch("dino-deleted", { dinoHash: dinoHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the dino: {error.message}</div>
{:else if editing}
  <EditDino
    originalDinoHash={dinoHash}
    currentRecord={record}
    on:dino-updated={async () => {
      editing = false;
      await fetchDino();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <section>
    <div>
      <span><strong>Name:</strong></span>
      <span>{dino?.name}</span>
    </div>
    <div>
      <span><strong>Dino Kind:</strong></span>
      <span>{
        dino?.dino_kind.type === "Apatosaurus" ? `Apatosaurus` : `Spinosaurus`
      }</span>
    </div>

    <div>
      <button
        on:click={() => {
          editing = true;
        }}
      >edit</button>
      <button on:click={() => deleteDino()}>delete</button>
    </div>
  </section>
{/if}
