<script lang="ts">
    import {getDinoState} from "../api";
    import Dino from "./Dino.svelte";

    const calculateHeight = (index: number, max: number): number => {
        console.log('index', index, 'max', max);

        const pos = index - (max / 2);

        const a = 4;
        const b = 10;
        const h = (1 + (Math.pow(pos, 2) / Math.pow(b, 2))) * Math.pow(a, 2);

        return (h - Math.floor(h)) * 100;
    }

    const size = 5;
    let scroll = $state(0);

    const dinos = $derived([
        ...getDinoState().slice(scroll % getDinoState().length, Math.min((scroll + size) % getDinoState().length)),
        ...getDinoState().slice()
    ])
</script>

<div class="flex flex-row m-2">
    <div class="flex flex-col justify-center">
        <button class="btn" onclick={() => scroll -= 1}>&lt;</button>
    </div>
    {#if getDinoState().length < 2}
        <div class="w-full h-32 flex flex-row justify-center gap-5">
            {#each getDinoState() as v}
                <div>
                    <Dino authoredDino={v}/>
                </div>
            {/each}
        </div>
    {:else}
        <div class="w-full h-64 flex flex-row justify-between">
            {#each getDinoState().slice(scroll % getDinoState().length, (scroll + size) % getDinoState().length) as v, index}
                <div class={["relative", "p-2", ""]}
                     style={`top: ${calculateHeight(index, Math.min(getDinoState().length, 5) - 1)}%`}>
                    <Dino authoredDino={v}/>
                </div>
            {/each}
        </div>
    {/if}
    <div class="flex flex-col justify-center">
        <button class="btn" onclick={() => scroll += 1}>&gt;</button>
    </div>
</div>
