import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  hashFrom32AndType,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function sampleDino(cell: CallableCell, partialDino = {}) {
  return {
    ...{
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      dino_kind: { type: "Apatosaurus" },
    },
    ...partialDino,
  };
}

export async function createDino(cell: CallableCell, dino = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "dino_adventure",
    fn_name: "create_dino",
    payload: dino || await sampleDino(cell),
  });
}
