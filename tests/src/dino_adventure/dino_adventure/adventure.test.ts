import { assert, test } from "vitest";
import { dhtSync, runScenario } from "@holochain/tryorama";
import { createAdventure, sampleAdventure } from "./common";
import { AuthoredAdventure } from "ui/src/dino_adventure/dino_adventure/types";

test("create adventure and get", async () => {
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
    const adventure1 = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const record1: AuthoredAdventure = await createAdventure(
      alice.cells[0],
      adventure1,
    );

    // Bob creates an adventure
    const adventure2 = sampleAdventure({
      participants: [bob.cells[0].cell_id[1], alice.cells[0].cell_id[1]],
    });
    const record2: AuthoredAdventure = await createAdventure(
      bob.cells[0],
      adventure2,
    );

    // Sync the DHT
    await scenario.shareAllAgents();
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice checks all adventures
    const createReadOutput1: AuthoredAdventure[] =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_all_adventures_local",
        payload: null,
      });
    createReadOutput1.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record1, record2], createReadOutput1);

    // Bob sees the same adventures
    const createReadOutput2: AuthoredAdventure[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_adventures_local",
      payload: null,
    });
    createReadOutput2.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual(createReadOutput1, createReadOutput2);
  });
});

test("my adventures", async () => {
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
    const adventure1 = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const record1: AuthoredAdventure = await createAdventure(
      alice.cells[0],
      adventure1,
    );

    // Bob creates an adventure
    const adventure2 = sampleAdventure({
      participants: [bob.cells[0].cell_id[1], alice.cells[0].cell_id[1]],
    });
    const record2: AuthoredAdventure = await createAdventure(
      bob.cells[0],
      adventure2,
    );

    // Sync the DHT
    await scenario.shareAllAgents();
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice checks her adventures
    const createReadOutput1: AuthoredAdventure[] =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_all_my_adventures_local",
        payload: null,
      });
    createReadOutput1.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record1], createReadOutput1);

    // Bob checks his adventures
    const createReadOutput2: AuthoredAdventure[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_my_adventures_local",
      payload: null,
    });
    createReadOutput2.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record2], createReadOutput2);
  });
});

test("end my adventure", async () => {
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
    const adventure1 = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const record1: AuthoredAdventure = await createAdventure(
      alice.cells[0],
      adventure1,
    );

    // Alice creates a second adventure
    const adventure2 = sampleAdventure({
      participants: [alice.cells[0].cell_id[1], bob.cells[0].cell_id[1]],
    });
    const record2: AuthoredAdventure = await createAdventure(
      alice.cells[0],
      adventure2,
    );

    // Bob creates an adventure
    const adventure3 = sampleAdventure({
      participants: [bob.cells[0].cell_id[1], alice.cells[0].cell_id[1]],
    });
    const record3: AuthoredAdventure = await createAdventure(
      bob.cells[0],
      adventure3,
    );

    // Sync the DHT
    await scenario.shareAllAgents();
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice checks her adventures
    const createReadOutput1: AuthoredAdventure[] =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_all_my_adventures_local",
        payload: null,
      });
    createReadOutput1.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record1, record2], createReadOutput1);

    // Bob checks his adventures
    const createReadOutput2: AuthoredAdventure[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_my_adventures_local",
      payload: null,
    });
    createReadOutput2.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record3], createReadOutput2);

    // Alice tries to unlink bob's adventure
    let err: unknown;
    try {
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "unlink_my_adventure",
        payload: record3.address,
      });
    } catch (e) {
      err = e;
    }
    assert.ok(err);

    // Alice unlinks her latest adventure
    await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "unlink_my_adventure",
      payload: record2.address,
    });

    // Alice checks her adventures
    const createReadOutput3: AuthoredAdventure[] =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_all_my_adventures_local",
        payload: null,
      });
    createReadOutput3.sort((a, b) => a.created_at - b.created_at);
    assert.deepEqual([record1], createReadOutput3);

    // Bob unlinks his adventure
    await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "unlink_my_adventure",
      payload: record3.address,
    });

    // Bob checks his adventures
    const createReadOutput4: AuthoredAdventure[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_my_adventures_local",
      payload: null,
    });
    assert.deepEqual([], createReadOutput4);
  });
});
