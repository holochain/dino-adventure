<script lang="ts">
import type {
  ActionHash,
  AppClient,
  HolochainError,
  Link,
} from "@holochain/client";
import { getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import DinoDetail from "./DinoDetail.svelte";
import type { DinoAdventureSignal } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let hashes: Array<ActionHash> = [];
let loading = false;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  client = await appClientContext.getClient();
  await fetchDinos();
  client.on("signal", signal => {
    if (signal.type != "app") return;
    if (signal.value.zome_name !== "dino_adventure") return;
    const payload = signal.value.payload as DinoAdventureSignal;
    if (payload.type !== "EntryCreated") return;
    if (payload.app_entry.type !== "Dino") return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchDinos() {
  loading = true;
  try {
    const links: Array<Link> = await client.callZome({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    if (links.length) {
      hashes = links.map(l => l.target);
    }
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the dinos: {error.message}.</div>
{:else if !hashes.length}
  <div class="alert">No dinos found.</div>
{:else}
  <div>
    {#each hashes as hash}
      <DinoDetail dinoHash={hash} on:dino-deleted={() => fetchDinos()} />
    {/each}
  </div>
{/if}
