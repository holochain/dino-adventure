import {
  AppWebsocket,
  HolochainError,
  type CallZomeRequest,
  type RoleNameCallZomeRequest,
  type Signal,
  type SignedActionHashed,
  encodeHashToBase64,
} from "@holochain/client";
import type { DinoAdventureSignal } from "../dino_adventure/dino_adventure/types";

let client: AppWebsocket | null = null;

let isConnected = $state(false);

let myPubKeyB64 = $state("");

export const getIsConnected = () => isConnected;

export const getAgentPubKeyB64 = () => myPubKeyB64;

export const runOnClient = async <T>(
  cb: (client: AppWebsocket) => Promise<T>,
): Promise<T> => {
  if (!client) {
    client = await AppWebsocket.connect();
    client.on("signal", signalHandler.handleSignal);
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

export type SignalHandlerCb<T> = (value: T, action: SignedActionHashed) => void;

export class SignalHandler {
  private store: { [key: string]: SignalHandlerCb<unknown> } = {};

  constructor() {
    this.handleSignal = this.handleSignal.bind(this);
  }

  handleSignal(signal: Signal) {
    if (signal.type == "app") {
      const appSignal = signal.value;

      const payload = appSignal.payload as DinoAdventureSignal;

      switch (appSignal.zome_name) {
        case "dino_adventure":
          switch (payload.type) {
            case "EntryCreated":
              const handler =
                this.store[
                  `${appSignal.zome_name}:${payload.type}:${payload.app_entry.type}`
                ];
              if (handler) {
                handler(payload.app_entry, payload.action);
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
