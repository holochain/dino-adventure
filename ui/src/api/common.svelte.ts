import {
  AppWebsocket,
  type CallZomeRequest,
  encodeHashToBase64,
  HolochainError,
  type RoleNameCallZomeRequest,
  type Signal,
  SignalType,
  type SignedActionHashed,
} from "@holochain/client";
import type { DinoAdventureSignal } from "../dino_adventure/dino_adventure/types";

let client: AppWebsocket | null = null;

let isConnected = $state(false);
let isConnecting = $state(false);

let myPubKeyB64 = $state("");

export const getIsConnected = () => isConnected;

export const getAgentPubKeyB64 = () => myPubKeyB64;

export const getAppInfo = () => client?.cachedAppInfo;

export const runOnClient = async <T>(
  cb: (client: AppWebsocket) => Promise<T>,
): Promise<T> => {
  if (!client) {
    if (isConnecting) {
      // Try again later
      return new Promise((resolve) => {
        setTimeout(resolve, 100);
      }).then(() => runOnClient(cb));
    }
    isConnecting = true;

    try {
      client = await AppWebsocket.connect();
      client.on("signal", (signal) => signalHandler.handleSignal(signal));
    } finally {
      isConnecting = false;
    }
  }

  isConnected = true;
  myPubKeyB64 = encodeHashToBase64(client.myPubKey);

  try {
    return await cb(client);
  } catch (e) {
    if (e instanceof HolochainError) {
      if (e.name == "ConnectionError") {
        // Clear the client so that it will be reconnected on the next call
        client = null;
        isConnected = false;
      }
    }

    throw e;
  }
};

export const callZome = async <T>(
  request: CallZomeRequest | RoleNameCallZomeRequest,
): Promise<T> => {
  return runOnClient<T>(async (client) => {
    return await client.callZome<T>(request);
  });
};

export type SignalHandlerCb<T> = (
  value: T,
  action: SignedActionHashed | null,
) => void;

export class SignalHandler {
  private store: { [key: string]: SignalHandlerCb<unknown> } = {};

  constructor() {
    this.handleSignal = this.handleSignal.bind(this);
  }

  handleSignal(signal: Signal) {
    if (signal.type == SignalType.App) {
      const appSignal = signal.value;

      const payload = appSignal.payload as DinoAdventureSignal;

      let handler;
      switch (appSignal.zome_name) {
        case "dino_adventure":
          switch (payload.type) {
            case "EntryCreated":
              handler =
                this.store[
                  `${appSignal.zome_name}:${payload.type}:${payload.app_entry.type}`
                ];
              if (handler) {
                handler(payload.app_entry, payload.action);
              }

              break;
            case "AdventureInvite":
              handler = this.store[`${appSignal.zome_name}:${payload.type}`];

              if (handler) {
                handler(
                  {
                    sender: payload.sender,
                  },
                  null,
                );
              }
              break;
            case "InviteAcceptance":
              handler = this.store[`${appSignal.zome_name}:${payload.type}`];

              if (handler) {
                handler(
                  {
                    accepted_by: payload.accepted_by,
                  },
                  null,
                );
              }
              break;
            case "IncomingPing":
              handler = this.store[`${appSignal.zome_name}:${payload.type}`];

              if (handler) {
                handler(
                  {
                    sender: payload.sender,
                    sent_at: payload.sent_at,
                  },
                  null,
                );
              }
              break;
            case "IncomingPong":
              handler = this.store[`${appSignal.zome_name}:${payload.type}`];

              if (handler) {
                handler(
                  {
                    sender: payload.sender,
                    round_trip_ms: payload.round_trip_ms,
                  },
                  null,
                );
              }
              break;
          }

          break;
      }
    }
  }

  addSignalHandler<T>(name: string, cb: SignalHandlerCb<T>) {
    this.store[name] = cb as SignalHandlerCb<unknown>;
  }
}

export const signalHandler = new SignalHandler();
