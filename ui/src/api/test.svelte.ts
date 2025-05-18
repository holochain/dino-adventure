import {
  type AgentPubKey,
  type AgentPubKeyB64,
  encodeHashToBase64,
} from "@holochain/client";
import { callZome, signalHandler } from "./common.svelte";

let wantedSentPings = $state<Record<AgentPubKeyB64, number>>({});

export const getWantedSentPings = () => wantedSentPings;

let sentPings = $state<Record<AgentPubKeyB64, number>>({});

export const getSentPings = () => sentPings;

let incomingPingsState = $state<Record<AgentPubKeyB64, number>>({});

export const getIncomingPings = () => incomingPingsState;

let incomingPongsState = $state<Record<AgentPubKeyB64, number[]>>({});

export const getIncomingPongs = () => incomingPongsState;

const testSendPing = async (to_agent: AgentPubKey) => {
  return await callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "test_send_ping",
    payload: to_agent,
  });
};

export interface RawrConfig {
  to_agent: AgentPubKey;
  times: number;
  interval_ms: number;
}

export const rawrAtAgent = (config: RawrConfig) => {
  const agentB64 = encodeHashToBase64(config.to_agent);
  if (agentB64 in wantedSentPings) {
    wantedSentPings[agentB64] += config.times;
  } else {
    wantedSentPings[agentB64] = config.times;
  }

  const sendPingRecursive = (config: RawrConfig) => {
    testSendPing(config.to_agent)
      .then(() => {
        if (agentB64 in sentPings) {
          sentPings[agentB64] += 1;
        } else {
          sentPings[agentB64] = 1;
        }

        config.times -= 1;
        if (config.times > 0) {
          setTimeout(() => sendPingRecursive(config), config.interval_ms);
        }
      })
      .catch(console.error);
  };

  sendPingRecursive(config);
};

export const clearAdventureState = () => {
  wantedSentPings = {};
  sentPings = {};
  incomingPingsState = {};
  incomingPongsState = {};
};

(() => {
  signalHandler.addSignalHandler(
    "dino_adventure:IncomingPing",
    (signal: { sender: AgentPubKey; sent_at: number }) => {
      const senderB64 = encodeHashToBase64(signal.sender);
      if (senderB64 in incomingPingsState) {
        incomingPingsState[senderB64] += 1;
      } else {
        incomingPingsState[senderB64] = 1;
      }
    },
  );

  signalHandler.addSignalHandler(
    "dino_adventure:IncomingPong",
    (signal: { sender: AgentPubKey; round_trip_ms: number }) => {
      const senderB64 = encodeHashToBase64(signal.sender);
      if (!(senderB64 in incomingPongsState)) {
        incomingPongsState[senderB64] = [signal.round_trip_ms];
      } else {
        incomingPongsState[senderB64].push(signal.round_trip_ms);
      }
    },
  );
})();
