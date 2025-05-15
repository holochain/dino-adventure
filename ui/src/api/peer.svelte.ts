import { runOnClient } from "./common.svelte";
import {
  type DumpNetworkStatsResponse,
  type DumpNetworkMetricsResponse,
} from "@holochain/client";

let networkStatsState = $state<DumpNetworkStatsResponse>({
  backend: "",
  connections: [],
  peer_urls: [],
});

let networkMetricsState = $state<DumpNetworkMetricsResponse>({});

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
    return await client.dumpNetworkStats();
  })
    .then((stats) => {
      networkStatsState = stats;
    })
    .catch(console.error);
};

const updateNetworkMetrics = () => {
  runOnClient(async (client): Promise<DumpNetworkMetricsResponse> => {
    return await client.dumpNetworkMetrics({
      include_dht_summary: false,
    });
  })
    .then((metrics) => {
      networkMetricsState = metrics;
    })
    .catch(console.error);
};

(() => {
  // Retrieve network stats and metrics every second
  setInterval(() => {
    updateNetworkStats();
    updateNetworkMetrics();
  }, 1000);
})();
