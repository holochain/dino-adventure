import { assert, test } from "vitest";
import { dhtSync, runScenario } from "@holochain/tryorama";
import {
  createAdventure,
  createNest,
  createNestBatch,
  sampleAdventure,
} from "./common";
import { AuthoredAdventure, NestBatchesWithNests } from "ui/src/types";
import { encodeHashToBase64 } from "@holochain/client";

test("create nest batch, nests and get", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/DinoAdventure.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { type: "path", value: testAppPath } };

    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Alice creates an adventure
    const adventure = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    await createAdventure(alice.cells[0], adventure);

    // Alice creates a nest batch
    const nestBatch = await createNestBatch(alice.cells[0]);

    // Alice creates several nests
    const nests = [];
    for (let i = 0; i < 3; i++) {
      nests.push(await createNest(alice.cells[0], nestBatch.address));
    }

    // Alice lists her nest batches
    const batches: NestBatchesWithNests = await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_nest_batches_with_nests_local",
      payload: alice.cells[0].cell_id[1],
    });

    assert.equal(1, batches.nest_batches.length);
    assert.equal(1, Object.keys(batches.nests).length);
    assert.equal(
      3,
      batches.nests[encodeHashToBase64(batches.nest_batches[0].address)].length,
    );

    assert.deepEqual(nestBatch, batches.nest_batches[0]);
    assert.deepEqual(
      nests,
      batches.nests[encodeHashToBase64(batches.nest_batches[0].address)].sort(
        (a, b) => a.created_at - b.created_at,
      ),
    );

    // Sync the DHT
    await scenario.shareAllAgents();
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob lists the same nest batches
    const batchesBob: NestBatchesWithNests = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_nest_batches_with_nests_local",
      payload: alice.cells[0].cell_id[1],
    });

    assert.deepEqual(batches, batchesBob);
  });
});

test("check batch after unlink adventure", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/DinoAdventure.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { type: "path", value: testAppPath } };

    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Alice creates an adventure
    const adventure = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const authoredAdventure: AuthoredAdventure = await createAdventure(
      alice.cells[0],
      adventure,
    );

    // Alice creates a nest batch
    const nestBatch = await createNestBatch(alice.cells[0]);

    // Alice creates several nests
    const nests = [];
    for (let i = 0; i < 3; i++) {
      nests.push(await createNest(alice.cells[0], nestBatch.address));
    }

    // Alice lists her nest batches
    const batches: NestBatchesWithNests = await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_nest_batches_with_nests_local",
      payload: alice.cells[0].cell_id[1],
    });

    assert.equal(1, batches.nest_batches.length);

    // Alice unlinks her adventure
    await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "unlink_my_adventure",
      payload: authoredAdventure.address,
    });

    // Alice lists her nest batches
    const batchesAfterUnlink: NestBatchesWithNests =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_nest_batches_with_nests_local",
        payload: alice.cells[0].cell_id[1],
      });

    assert.equal(0, batchesAfterUnlink.nest_batches.length);
    assert.equal(0, Object.keys(batchesAfterUnlink.nests).length);
  });
});

test("Get nests for adventure", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/DinoAdventure.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { type: "path", value: testAppPath } };

    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Alice creates an adventure
    const adventure = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const createdAdventure = await createAdventure(alice.cells[0], adventure);

    // Alice creates a nest batch
    const nestBatch = await createNestBatch(alice.cells[0]);

    // Alice creates several nests
    const nests = [];
    for (let i = 0; i < 3; i++) {
      nests.push(await createNest(alice.cells[0], nestBatch.address));
    }

    // Sync the DHT
    await scenario.shareAllAgents();
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice gets the nest batches for her adventure
    const batchesAlice: NestBatchesWithNests = await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_nest_batches_with_nests_by_adventure_hash_local",
      payload: createdAdventure.address,
    });

    assert.equal(3, Object.values(batchesAlice.nests).flat().length);

    // Bob gets the nest batches for Alice's adventure
    const batchesBob: NestBatchesWithNests = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_nest_batches_with_nests_by_adventure_hash_local",
      payload: createdAdventure.address,
    });

    assert.deepEqual(batchesAlice, batchesBob);
  });
});
