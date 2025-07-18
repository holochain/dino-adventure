import { callZome } from "./common.svelte";
import type {
  AuthoredNest,
  AuthoredNestBatch,
  NestBatchesWithNests,
  CreateNestRequest,
} from "../types";
import type { ActionHash, AgentPubKey } from "@holochain/client";

export const createNestBatch = async (): Promise<AuthoredNestBatch> => {
  return await callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "create_nest_batch",
    payload: null,
  });
};

export const createNest = async (
  request: CreateNestRequest,
): Promise<AuthoredNest> => {
  return await callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "create_nest",
    payload: request,
  });
};

export const getNestBatchesWithNests = async (
  author: AgentPubKey,
): Promise<NestBatchesWithNests> => {
  return await callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_nest_batches_with_nests_local",
    payload: author,
  });
};

export const getNestBatchesForAdventure = async (
  adventureAddress: ActionHash,
): Promise<NestBatchesWithNests> => {
  return await callZome({
    role_name: "dino_adventure",
    zome_name: "dino_adventure",
    fn_name: "get_nest_batches_with_nests_by_adventure_hash_local",
    payload: adventureAddress,
  });
};
