<script lang="ts">
  import { getMyLocalAgent } from "../api";
  import type { DhtArc } from "@holochain/client";

  const myLocalAgent = $derived.by(() => {
    return getMyLocalAgent();
  });

  type ArcType = null | [number, number];

  const arcEqual = (inputArc1: DhtArc, inputArc2: DhtArc): boolean => {
    const arc1 = inputArc1 as unknown as ArcType;
    const arc2 = inputArc2 as unknown as ArcType;

    if (arc1 === null && arc2 === null) {
      return true;
    } else if (Array.isArray(arc1) && Array.isArray(arc2)) {
      return arc1[0] === arc2[0] && arc1[1] === arc2[1];
    } else {
      return false;
    }
  };

  const renderArc = (inputArc: DhtArc): string => {
    // TODO defined wrongly in holochain-client
    const arc = inputArc as unknown as ArcType;

    if (arc === null) {
      return "Empty";
    } else if (Array.isArray(arc)) {
      if (arc[0] === 0 && arc[1] === 4294967295) {
        return "Full";
      } else {
        return `${arc[0]}..${arc[1]}`;
      }
    } else {
      console.warn("Invalid arc", arc);
      return "Invalid arc";
    }
  };

  const renderArcInfo = (inputArc: DhtArc): string => {
    const arc = inputArc as unknown as ArcType;

    if (arc === null) {
      return "Store and validate no data";
    } else if (Array.isArray(arc)) {
      if (arc[0] === 0 && arc[1] === 4294967295) {
        return "Full arc, store and validate all data";
      } else {
        return `Store and validate data in range ${arc[0]}..${arc[1]}`;
      }
    } else {
      return "Invalid arc";
    }
  };
</script>

{#if !myLocalAgent}
  <p>Loading...</p>
{:else if !arcEqual(myLocalAgent.target_arc, myLocalAgent.storage_arc)}
  <p
    class="tooltip tooltip-left"
    data-tip={`Targeting arc: ${renderArc(myLocalAgent.target_arc)}, currently at: ${renderArc(myLocalAgent.storage_arc)}`}
  >
    Initial sync...
  </p>
{:else}
  <p
    class="tooltip tooltip-left"
    data-tip={renderArcInfo(myLocalAgent.storage_arc)}
  >
    {renderArc(myLocalAgent.storage_arc)}
  </p>
{/if}
