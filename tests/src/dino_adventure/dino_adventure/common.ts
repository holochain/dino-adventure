import { ActionHash, type AgentPubKey } from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";
import {
  Adventure,
  AuthoredAdventure,
  AuthoredNestBatch,
  CreateNestRequest,
} from "ui/src/types";

export interface Dino {
  name: string;
  dino_kind: { type: string };
}

export interface AuthoredDino {
  dino: Dino;
  author: AgentPubKey;
  address: ActionHash;
}

export const sampleDino = (partialDino: Partial<Dino> = {}) => {
  return {
    ...{
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      dino_kind: { type: "Apatosaurus" },
    },
    ...partialDino,
  };
};

export const createDino = async (
  cell: CallableCell,
  dino: Dino | undefined = undefined,
): Promise<AuthoredDino> => {
  return await cell.callZome({
    zome_name: "dino_adventure",
    fn_name: "create_dino",
    payload: dino || sampleDino(),
  });
};

export const sampleAdventure = (partialAdventure: Partial<Adventure> = {}) => {
  return {
    ...{
      participants: [],
    },
    ...partialAdventure,
  };
};

export const createAdventure = async (
  cell: CallableCell,
  adventure: Adventure | undefined = undefined,
): Promise<AuthoredAdventure> => {
  return await cell.callZome({
    zome_name: "dino_adventure",
    fn_name: "create_adventure",
    payload: adventure || sampleAdventure(),
  });
};

export const createNestBatch = async (
  cell: CallableCell,
): Promise<AuthoredNestBatch> => {
  return await cell.callZome({
    zome_name: "dino_adventure",
    fn_name: "create_nest_batch",
    payload: null,
  });
};

export const createNest = async (
  cell: CallableCell,
  nestBatchAddress: ActionHash,
  size: number = 50,
): Promise<AuthoredNestBatch> => {
  return await cell.callZome({
    zome_name: "dino_adventure",
    fn_name: "create_nest",
    payload: {
      nest_batch_address: nestBatchAddress,
      size,
    } as CreateNestRequest,
  });
};
