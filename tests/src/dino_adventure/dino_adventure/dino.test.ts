import { assert, test } from "vitest";
import { dhtSync, runScenario } from "@holochain/tryorama";
import { AuthoredDino, createDino, sampleDino } from "./common.js";

test("create dino", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/DinoAdventure.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { type: "path", value: testAppPath } };

    const [alice] = await scenario.addPlayersWithApps([appSource]);

    // Alice creates a Dino
    const record: AuthoredDino = await createDino(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and get all dinos", async () => {
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

    const sample1 = sampleDino({
      name: "dino 1",
    });
    const sample2 = sampleDino({
      name: "dino 2",
    });

    // Alice creates a Dino
    const record1: AuthoredDino = await createDino(alice.cells[0], sample1);
    assert.ok(record1);

    // Bob creates a Dino
    const record2: AuthoredDino = await createDino(bob.cells[0], sample2);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Dinos
    const createReadOutput: AuthoredDino[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    createReadOutput.sort((a, b) => a.dino.name.localeCompare(b.dino.name));
    assert.deepEqual([record1, record2], createReadOutput);

    // Bob gets the created Dinos locally
    const createReadOutputLocal: AuthoredDino[] = await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos_local",
      payload: null,
    });
    createReadOutputLocal.sort((a, b) =>
      a.dino.name.localeCompare(b.dino.name),
    );
    assert.deepEqual(createReadOutput, createReadOutputLocal);

    // Alice gets the created Dinos
    const createReadOutput2: AuthoredDino[] = await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "get_all_dinos",
      payload: null,
    });
    createReadOutput2.sort((a, b) => a.dino.name.localeCompare(b.dino.name));
    assert.deepEqual([record1, record2], createReadOutput2);

    // Alice gets the created Dinos locally
    const createReadOutput2Local: AuthoredDino[] =
      await alice.cells[0].callZome({
        zome_name: "dino_adventure",
        fn_name: "get_all_dinos_local",
        payload: null,
      });
    createReadOutput2Local.sort((a, b) =>
      a.dino.name.localeCompare(b.dino.name),
    );
    assert.deepEqual(createReadOutput2, createReadOutput2Local);
  });
});
