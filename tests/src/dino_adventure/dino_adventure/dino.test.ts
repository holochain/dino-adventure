import { assert, test } from "vitest";

import {
  ActionHash,
  AppBundleSource,
  CreateLink,
  DeleteLink,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  Link,
  NewEntryAction,
  Record,
  SignedActionHashed,
} from "@holochain/client";
import { CallableCell, dhtSync, runScenario } from "@holochain/tryorama";
import { decode } from "@msgpack/msgpack";

import { AuthoredDino, createDino, sampleDino } from "./common.js";

test("create Dino", async () => {
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

    // Alice creates a Dino
    const record: Record = await createDino(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read Dino", async () => {
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

    const sample = await sampleDino(alice.cells[0]);

    // Alice creates a Dino
    const record: AuthoredDino = await createDino(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Dino
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_original_dino",
      payload: record.address,
    });
    assert.deepEqual(
      sample,
      decode((createReadOutput.entry as any).Present.entry) as any,
    );
  });
});

test("create and delete Dino", async () => {
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

    const sample = await sampleDino(alice.cells[0]);

    // Alice creates a Dino
    const authoredDino = await createDino(alice.cells[0], sample);
    assert.ok(authoredDino);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice deletes the Dino
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "delete_dino",
      payload: authoredDino.address,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the Dino
    const oldestDeleteForDino: SignedActionHashed = await bob.cells[0].callZome(
      {
        zome_name: "dino_adventure",
        fn_name: "get_oldest_delete_for_dino",
        payload: authoredDino.address,
      },
    );
    assert.ok(oldestDeleteForDino);

    // Bob gets the deletions for the Dino
    const deletesForDino: SignedActionHashed[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_deletes_for_dino",
      payload: authoredDino.address,
    });
    assert.equal(deletesForDino.length, 1);
  });
});
