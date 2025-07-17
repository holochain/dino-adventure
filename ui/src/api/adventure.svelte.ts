import type { Adventure, AuthoredAdventure } from "../types";
import { callZome, getAgentPubKeyB64, signalHandler } from "./common.svelte";
import {
  type ActionHashB64,
  encodeHashToBase64,
  type SignedActionHashed,
} from "@holochain/client";

let myAdventuresState: Record<ActionHashB64, AuthoredAdventure> = $state({});

const myLatestAdventureState = $derived.by(() => {
  const all = Object.values(myAdventuresState);
  all.sort((a, b) => a.created_at - b.created_at);

  return all.length > 0 ? all[0] : null;
});

export const getMyAdventures = () => myAdventuresState;

export const getMyLatestAdventure = () => myLatestAdventureState;

export const createAdventure = async (
  adventure: Adventure,
): Promise<AuthoredAdventure> => {
  try {
    return await callZome({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "create_adventure",
      payload: adventure,
    });
  } catch (error) {
    console.error("Error creating adventure:", error);
    throw error;
  }
};

export const endAdventure = async () => {
  const latestAdventureState = myLatestAdventureState;
  if (!latestAdventureState) {
    console.error("No adventure to end");
    return;
  }

  try {
    await callZome({
      role_name: "dino_adventure",
      zome_name: "dino_adventure",
      fn_name: "unlink_my_adventure",
      payload: latestAdventureState.address,
    });

    delete myAdventuresState[encodeHashToBase64(latestAdventureState.address)];
  } catch (error) {
    console.error("Error ending adventure:", error);
    throw error;
  }
};

const fetchMyAdventures = async (): Promise<void> => {
  const authoredAdventures = await callZome<AuthoredAdventure[]>({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_all_my_adventures_local",
    payload: null,
  });

  const newAdventures = authoredAdventures.filter(
    (newAdventure) =>
      !(encodeHashToBase64(newAdventure.address) in myAdventuresState),
  );
  if (newAdventures.length > 0) {
    myAdventuresState = {
      // Keep existing dinos in the state
      ...myAdventuresState,
      // Add any new dinos that were missing
      ...newAdventures.reduce(
        (acc, adventure) => {
          acc[encodeHashToBase64(adventure.address)] = adventure;
          return acc;
        },
        {} as Record<ActionHashB64, AuthoredAdventure>,
      ),
    };
  }
};

(() => {
  signalHandler.addSignalHandler(
    "dino_adventure:EntryCreated:Adventure",
    (adventure: Adventure, action: SignedActionHashed | null) => {
      if (
        encodeHashToBase64(action!.hashed.content.author) !==
        getAgentPubKeyB64()
      ) {
        return;
      }

      myAdventuresState[encodeHashToBase64(action!.hashed.hash)] = {
        adventure: adventure,
        address: action!.hashed.hash,
        created_at: action!.hashed.content.timestamp,
        author: action!.hashed.content.author,
      };
    },
  );

  // Load all the adventures when the page loads
  fetchMyAdventures().catch(console.error);

  // Poll for new adventures every 5 seconds
  setInterval(() => {
    fetchMyAdventures().catch(console.error);
  }, 5000);
})();
