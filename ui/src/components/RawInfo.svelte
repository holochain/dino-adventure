<script lang="ts">
  import { getNetworkMetrics, getNetworkStats } from "../api";
  import {
    type AppInfo,
    type DnaHashB64,
    encodeHashToBase64,
  } from "@holochain/client";
  import { getAppInfo } from "../api/common.svelte";

  let checked = $state(false);

  const transformedMetrics = $derived.by<object>(() => {
    let metrics = getNetworkMetrics();

    if (!metrics) {
      return {};
    }

    let out = {} as Record<DnaHashB64, object>;
    for (const [key, value] of Object.entries(metrics)) {
      const peerMetaList = [];
      for (const [peerUrl, peerMeta] of Object.entries(
        value.gossip_state_summary.peer_meta,
      )) {
        peerMetaList.push({
          peer_url: peerUrl,
          meta: peerMeta,
        });
      }
      peerMetaList.sort((a, b) => a.peer_url.localeCompare(b.peer_url));

      out[key] = {
        fetch_state_summary: value.fetch_state_summary,
        gossip_state_summary: {
          initiated_round: value.gossip_state_summary.initiated_round,
          accepted_rounds: value.gossip_state_summary.accepted_rounds,
          dht_summary: value.gossip_state_summary.dht_summary,
          peer_meta: peerMetaList,
        },
        local_agents: value.local_agents.map((a) => {
          return {
            agent: encodeHashToBase64(a.agent),
            storage_arc: a.storage_arc,
            target_arc: a.target_arc,
          };
        }),
      };
    }

    return out;
  });

  const transformAppInfo = (appInfo: AppInfo | null | undefined): object => {
    const out = {} as Record<string, unknown>;

    if (appInfo) {
      let cellsOut = [];
      for (const [roleName, cellInfos] of Object.entries(appInfo.cell_info)) {
        cellsOut.push({
          role_name: roleName,
          cells: cellInfos.map((cell) => {
            if (cell.type == "provisioned") {
              return {
                name: cell.value.name,
                dna_modifiers: cell.value.dna_modifiers,
                cell_id: {
                  dna_hash: encodeHashToBase64(cell.value.cell_id[0]),
                  agent_pub_key: encodeHashToBase64(cell.value.cell_id[1]),
                },
              };
            } else {
              return {
                unhandled_cell_type: true,
              };
            }
          }),
        });
      }

      out["agent_pub_key"] = encodeHashToBase64(appInfo.agent_pub_key);
      out["installed_app_id"] = appInfo.installed_app_id;
      out["cell_info"] = cellsOut;
      out["status"] = appInfo.status;
      out["installed_at"] = new Date(appInfo.installed_at / 1000);
    }

    return out;
  };
</script>

<div class="flex flex-row items-center gap-1">
  <span class="floating-label">Raw info</span>
  <input id="raw-info" type="checkbox" bind:checked class="toggle" />
</div>

{#if checked}
  <div
    class="w-8/12 h-screen bg-gray-700/90 fixed z-2 top-0 left-1/6 overflow-y-scroll overflow-x-clip p-1"
  >
    <p class="text-xl bold py-3">App info</p>
    <pre>{JSON.stringify(transformAppInfo(getAppInfo()), null, 2)}</pre>

    <p class="text-xl bold py-3">Network stats</p>
    <pre>{JSON.stringify(getNetworkStats(), null, 2)}</pre>

    <p class="text-xl bold py-3">Network metrics</p>
    <pre>{JSON.stringify(transformedMetrics, null, 2)}</pre>
  </div>
{/if}
