<script lang="ts">
  import { getDinoState } from "../api";
  import Dino from "./Dino.svelte";
  import type { AuthoredDino } from "../types";

  const calculateHeight = (index: number, max: number): number => {
    const pos = index - max / 2;

    const a = 4;
    const b = 10;
    const h = (1 + Math.pow(pos, 2) / Math.pow(b, 2)) * Math.pow(a, 2);

    return (h - Math.floor(h)) * 100;
  };

  const size = 5;
  let scroll = $state(0);

  const dinos = $derived.by(() => {
    let dinoState = Object.values(getDinoState());

    let useSize = Math.min(dinoState.length, size);

    let out: (AuthoredDino | null)[] = [];
    let pos = scroll % dinoState.length;
    for (let i = 0; i < useSize; i++) {
      out.push(dinoState[pos % dinoState.length]);
      pos = (pos + 1) % dinoState.length;
    }

    if (out.length < size) {
      if (out.length % 2 === 0) {
        out.splice(out.length / 2, 0, null);
      }

      while (out.length < size) {
        out.splice(0, 0, null);
        out.push(null);
      }
    }

    return out;
  });
</script>

<div class="flex flex-row m-2">
  <div class="flex flex-col justify-center">
    <button class="btn" onclick={() => (scroll -= 1)}>&lt;</button>
  </div>
  <div class="w-full h-48 px-5 flex flex-row justify-between">
    {#each dinos as v, index}
      <div style={`margin-top: ${calculateHeight(index, size - 1)}px`}>
        <div>
          {#if v}
            <Dino authoredDino={v} enableCopyAgentKey={true} />
          {:else}
            <div class="w-16"></div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
  <div class="flex flex-col justify-center">
    <button class="btn" onclick={() => (scroll += 1)}>&gt;</button>
  </div>
</div>
