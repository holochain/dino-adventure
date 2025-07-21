import { getAgentPubKeyB64, runOnClient } from "./common.svelte";
import {
  type DumpNetworkMetricsResponse,
  type DumpNetworkStatsResponse,
  type AgentMetaInfo,
  type AgentMetaInfoResponse,
  type DnaHashB64,
  encodeHashToBase64,
} from "@holochain/client";

let networkStatsState = $state<DumpNetworkStatsResponse>({
  backend: "",
  connections: [],
  peer_urls: [],
});

export const getNetworkStats = () => networkStatsState;

let networkMetricsState = $state<DumpNetworkMetricsResponse>({});

export const getNetworkMetrics = () => networkMetricsState;

// Map from DNA hash, through peer url, of peer meta key to AgentMetaInfo
const peerMetaState = $state<
  Record<DnaHashB64, Record<string, Record<string, AgentMetaInfo>>>
>({});

export const getPeerMeta = () => peerMetaState;

const myLocalAgent = $derived.by(() => {
  const metrics = Object.values(networkMetricsState);
  if (metrics.length === 0) {
    return;
  }
  const myAgentKey = getAgentPubKeyB64();

  return metrics[0].local_agents.find(
    (a) => encodeHashToBase64(a.agent) === myAgentKey,
  );
});

export const getMyLocalAgent = () => myLocalAgent;

export interface PeerConnections {
  connectedPeers: number;
  directConnectedPeers: number;
}

const peerConnectionsState = $derived<PeerConnections>({
  connectedPeers: networkStatsState.connections.length,
  directConnectedPeers: networkStatsState.connections.filter((c) => c.is_webrtc)
    .length,
});

const fetchQueueCountState = $derived.by(() => {
  return Object.keys(networkMetricsState).reduce((acc, key) => {
    return (
      acc +
      Object.keys(networkMetricsState[key].fetch_state_summary.pending_requests)
        .length
    );
  }, 0);
});

export const getPeerConnectionsState = () => peerConnectionsState;

export const getFetchQueueCount = () => fetchQueueCountState;

const updateNetworkStats = () => {
  runOnClient(async (client): Promise<DumpNetworkStatsResponse> => {
    return await client.dumpNetworkStats(1000);
  })
    .then((stats) => {
      networkStatsState = stats;
    })
    .catch(console.error);
};

const updateNetworkMetrics = () => {
  runOnClient(async (client): Promise<DumpNetworkMetricsResponse> => {
    return await client.dumpNetworkMetrics(
      {
        include_dht_summary: false,
      },
      1000,
    );
  })
    .then((metrics) => {
      networkMetricsState = metrics;
    })
    .catch(console.error);
};

const updatePeerMetaState = () => {
  runOnClient(async (client) => {
    return await client.agentInfo(
      {
        dna_hashes: null,
      },
      1000,
    );
  })
    .then((agentInfos) => {
      const decodedInfos = agentInfos.map((info): { url?: string } => {
        const infoObj = JSON.parse(info) as unknown;
        if (!infoObj || typeof infoObj !== "object") {
          console.warn(`Invalid agent info format: ${info}`);
          return { url: undefined };
        }

        const outer = infoObj?.agentInfo as unknown;
        if (!outer || typeof outer !== "string") {
          console.warn(`Invalid agent info format: ${info}`);
          return { url: undefined };
        }

        return JSON.parse(outer) as { url?: string };
      });

      for (const connStats of networkStatsState.connections) {
        const matchedInfo = decodedInfos.find((info) =>
          info.url?.includes(connStats.pub_key),
        );
        if (!matchedInfo) {
          console.warn(
            `No valid agent info found for connection: ${connStats.pub_key}`,
          );
          continue;
        }

        if (!matchedInfo.url) {
          continue;
        }

        const url = matchedInfo.url;

        runOnClient(async (client): Promise<AgentMetaInfoResponse> => {
          return await client.agentMetaInfo(
            {
              url,
            },
            1000,
          );
        })
          .then((meta) => {
            for (const [dnaHash, value] of Object.entries(meta)) {
              if (!peerMetaState[dnaHash]) {
                peerMetaState[dnaHash] = {};
              }
              peerMetaState[dnaHash][url] = value;
            }
          })
          .catch(console.error);
      }
    })
    .catch(console.error);
};

(() => {
  // Retrieve network stats and metrics every second
  setInterval(() => {
    updateNetworkStats();
    updateNetworkMetrics();
    updatePeerMetaState();
  }, 1000);
})();
