import type {
  AuthoredDino,
  Dino,
} from "../dino_adventure/dino_adventure/types";
import { callZome, signalHandler } from "./common.svelte";
import { encodeHashToBase64, type SignedActionHashed } from "@holochain/client";

let dinoState: AuthoredDino[] = $state([]);

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

const fetchDinos = async (): Promise<AuthoredDino[]> => {
  const authoredDinos = await callZome<AuthoredDino[]>({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_all_dinos",
    payload: null,
  });

  const newDinos = authoredDinos.filter(
    (newDino) =>
      dinoState.find(
        (existingDino) =>
          encodeHashToBase64(existingDino.address) ===
          encodeHashToBase64(newDino.address),
      ) == undefined,
  );
  if (newDinos.length > 0) {
    dinoState = [
      // Keep existing dinos in the state
      ...dinoState,
      // Add any new dinos that were missing
      ...newDinos,
    ];
  }

  if (!dinosFirstLoad) {
    setTimeout(() => {
      dinosFirstLoad = true;
    }, 1000);
  }

  return authoredDinos;
};

(async () => {
  signalHandler.addSignalHandler(
    "dino_adventure:EntryCreated:Dino",
    (dino: Dino, action: SignedActionHashed) => {
      dinoState.push({
        dino: dino,
        address: action.hashed.hash,
        author: action.hashed.content.author,
      });
    },
  );

  // Load all the dinos when the page loads
  await fetchDinos();

  // Poll for new dinos every 5 seconds
  setInterval(() => {
    fetchDinos().catch(console.error);
  }, 5000);
})();
