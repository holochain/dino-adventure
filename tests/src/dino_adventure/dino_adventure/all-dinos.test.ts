import { assert, test } from "vitest";

import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  Link,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell, dhtSync, runScenario } from "@holochain/tryorama";
import { decode } from "@msgpack/msgpack";

import { createDino, type AuthoredDino } from "./common.js";

test("create a Dino and get all dinos", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/DinoAdventure.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { type: "path", value: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all dinos
    let collectionOutput = await bob.cells[0].callZome<AuthoredDino[]>({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a Dino
    const authoredDino = await createDino(alice.cells[0]);
    assert.ok(authoredDino);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all dinos again
    collectionOutput = await bob.cells[0].callZome<AuthoredDino[]>({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(authoredDino, collectionOutput[0]);

    // Alice deletes the Dino
    await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "delete_dino",
      payload: authoredDino.address,
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all dinos again
    collectionOutput = await bob.cells[0].callZome<AuthoredDino[]>({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    assert.equal(collectionOutput.length, 0);
  });
});
