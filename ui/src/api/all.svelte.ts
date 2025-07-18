import { type ActionHashB64, encodeHashToBase64 } from "@holochain/client";
import type { AuthoredAdventure } from "../types";
import { callZome } from "./common.svelte";

let allAdventures: Record<ActionHashB64, AuthoredAdventure> = $state({});

export const getAllAdventures = () => allAdventures;

const fetchAllAdventures = async (): Promise<void> => {
  const authoredAdventures = await callZome<AuthoredAdventure[]>({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_all_adventures_local",
    payload: null,
  });

  const newAdventures = authoredAdventures.filter(
    (newAdventure) =>
      !(encodeHashToBase64(newAdventure.address) in allAdventures),
  );
  if (newAdventures.length > 0) {
    allAdventures = {
      // Keep existing adventures in the state
      ...allAdventures,
      // Add any new adventures that were missing
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
  setInterval(() => {
    fetchAllAdventures().catch(console.error);
  }, 5000);
})();
