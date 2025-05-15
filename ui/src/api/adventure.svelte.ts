import type {
  Adventure,
  AuthoredAdventure,
  AuthoredDino,
  Dino,
} from "../dino_adventure/dino_adventure/types";
import { callZome, signalHandler } from "./common.svelte";
import { encodeHashToBase64, type SignedActionHashed } from "@holochain/client";

let adventureState: AuthoredAdventure[] = $state([]);

export const getAdventureState = () => adventureState;

export const createAdventure = async (
  adventure: Adventure,
): Promise<AuthoredAdventure> => {
  return callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "create_adventure",
    payload: adventure,
  });
};

const fetchAdventures = async (): Promise<void> => {
  const authoredAdventures = await callZome<AuthoredAdventure[]>({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_all_adventures_local",
    payload: null,
  });

  const newAdventures = authoredAdventures.filter(
    (newAdventure) =>
      adventureState.find(
        (existingAdventure) =>
          encodeHashToBase64(existingAdventure.address) ===
          encodeHashToBase64(newAdventure.address),
      ) == undefined,
  );
  if (newAdventures.length > 0) {
    adventureState = [
      // Keep existing dinos in the state
      ...adventureState,
      // Add any new dinos that were missing
      ...newAdventures,
    ];
  }
};

(() => {
  signalHandler.addSignalHandler(
    "dino_adventure:EntryCreated:Adventure",
    (adventure: Adventure, action: SignedActionHashed | null) => {
      adventureState.push({
        adventure: adventure,
        address: action!.hashed.hash,
        author: action!.hashed.content.author,
      });
    },
  );

  // Load all the adventures when the page loads
  fetchAdventures().catch(console.error);

  // Poll for new adventures every 5 seconds
  setInterval(() => {
    fetchAdventures().catch(console.error);
  }, 5000);
})();
