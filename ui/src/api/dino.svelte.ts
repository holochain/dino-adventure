import type { AuthoredDino, Dino } from "../types";
import { callZome, signalHandler } from "./common.svelte";
import {
  type AgentPubKeyB64,
  encodeHashToBase64,
  type SignedActionHashed,
} from "@holochain/client";

let dinoState: Record<AgentPubKeyB64, AuthoredDino> = $state({});

let dinosFirstLoad = $state(false);

export const getDinoState = () => dinoState;

export const getDinosFirstLoaded = () => dinosFirstLoad;

export const createDino = async (dino: Dino): Promise<AuthoredDino> => {
  return callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "create_dino",
    payload: dino,
  });
};

let networkRefreshCounter = 0;
const fetchDinos = async (): Promise<void> => {
  let authoredDinos: AuthoredDino[];

  if (networkRefreshCounter % 5 === 0) {
    authoredDinos = await callZome<AuthoredDino[]>({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
  } else {
    authoredDinos = await callZome<AuthoredDino[]>({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos_local",
      payload: null,
    });
  }
  networkRefreshCounter++;

  const newDinos = authoredDinos.filter(
    (newDino) => !(encodeHashToBase64(newDino.author) in dinoState),
  );
  if (newDinos.length > 0) {
    dinoState = {
      // Keep existing dinos in the state
      ...dinoState,
      // Add any new dinos that were missing
      ...newDinos.reduce(
        (acc, dino) => {
          acc[encodeHashToBase64(dino.author)] = dino;
          return acc;
        },
        {} as Record<AgentPubKeyB64, AuthoredDino>,
      ),
    };
  }

  if (!dinosFirstLoad) {
    setTimeout(() => {
      dinosFirstLoad = true;
    }, 1000);
  }
};

(() => {
  signalHandler.addSignalHandler(
    "dino_adventure:EntryCreated:Dino",
    (dino: Dino, action: SignedActionHashed | null) => {
      dinoState[encodeHashToBase64(action!.hashed.content.author)] = {
        dino: dino,
        address: action!.hashed.hash,
        author: action!.hashed.content.author,
      };
    },
  );

  // Load all the dinos when the page loads
  fetchDinos().catch(console.error);

  // Poll for new dinos every 5 seconds
  setInterval(() => {
    fetchDinos().catch(console.error);
  }, 5000);
})();
