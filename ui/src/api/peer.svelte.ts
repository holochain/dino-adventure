import {runOnClient} from "./common.svelte";

export interface PeerInfo {
    connected_peers: number;
    direct_connected_peers: number;
}

export const getPeerInfo = async (): Promise<PeerInfo> => {
    return runOnClient(async (client): Promise<PeerInfo> => {
        const stats = await client.dumpNetworkStats();

        return {
            connected_peers: stats.connections.length,
            direct_connected_peers: stats.connections.filter((c) => c.is_webrtc).length,
        }
    })
};
