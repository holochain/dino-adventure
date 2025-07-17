<script lang="ts">

import {getAllAdventures} from "../api";
import {encodeHashToBase64} from "@holochain/client";

const adventureHistory = $derived.by(() => {
    let all = Object.values(getAllAdventures());

    return all.sort((a, b) => {
        return a.created_at - b.created_at;
    });
});

</script>

<div>
    <p class="text-xl">All adventures</p>
    {#each adventureHistory as adventure}
        <p>Adventure by: { encodeHashToBase64(adventure.author) }</p>
        <p>Created at: { new Date(adventure.created_at).toLocaleString() }</p>
        <p>Participants: { adventure.adventure.participants.map(encodeHashToBase64).join(', ') }</p>
    {/each}
</div>
