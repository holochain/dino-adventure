import { test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { SignalType } from "@holochain/client";
import type { DinoAdventureSignal } from "ui/src/types";

test("send ping", async () => {
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

    // Skip peer discovery
    await scenario.shareAllAgents();

    const bobReceiver = new Promise((resolve, reject) => {
      setTimeout(() => reject(new Error("bob did not get a ping")), 30_000);
      bob.appWs.on("signal", (signal) => {
        if (signal.type == SignalType.App) {
          const sig = signal.value.payload as DinoAdventureSignal;
          if (sig.type == "IncomingPing") {
            resolve(null);
          }
        }
      });
    });

    const aliceReceiver = new Promise((resolve, reject) => {
      setTimeout(() => reject(new Error("alice did not get a pong")), 30_000);
      alice.appWs.on("signal", (signal) => {
        if (signal.type == SignalType.App) {
          const sig = signal.value.payload as DinoAdventureSignal;
          if (sig.type == "IncomingPong") {
            resolve(null);
          }
        }
      });
    });

    // Alice sends a ping to bob
    await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "test_send_ping",
      payload: bob.cells[0].cell_id[1],
    });

    await bobReceiver;
    await aliceReceiver;
  });
});
