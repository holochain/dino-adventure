import { test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { SignalType } from "@holochain/client";
import type {
  AdventureInvite,
  AdventureInviteAcceptance,
  DinoAdventureSignal,
} from "ui/src/dino_adventure/dino_adventure/types";

test("invite and accept", async () => {
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
      setTimeout(() => reject(new Error("bob did not get an invite")), 5000);
      bob.appWs.on("signal", (signal) => {
        if (signal.type == SignalType.App) {
          const sig = signal.value.payload as DinoAdventureSignal;
          if (sig.type == "AdventureInvite") {
            resolve(null);
          }
        }
      });
    });

    // Alice invites Bob to an adventure
    await alice.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "send_invite",
      payload: {
        to: bob.cells[0].cell_id[1],
      } as AdventureInvite,
    });

    await bobReceiver;

    const aliceReceiver = new Promise((resolve, reject) => {
      setTimeout(() => reject(new Error("alice did not get an accept")), 5000);
      alice.appWs.on("signal", (signal) => {
        if (signal.type == SignalType.App) {
          const sig = signal.value.payload as DinoAdventureSignal;
          if (sig.type == "InviteAcceptance") {
            resolve(null);
          }
        }
      });
    });

    // Bob accepts the adventure
    await bob.cells[0].callZome({
      zome_name: "dino_adventure",
      fn_name: "accept_invite",
      payload: {
        to: alice.cells[0].cell_id[1],
      } as AdventureInviteAcceptance,
    });

    await aliceReceiver;
  });
});
