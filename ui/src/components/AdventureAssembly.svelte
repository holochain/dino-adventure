<script lang="ts">
  import {
    acceptInvite,
    createAdventure,
    getAcceptedInvites,
    getAgentPubKeyB64,
    getDinoState,
    getOpenInvites,
    getSentInvites,
    injectOurAcceptedInvite,
    removeOpenInvite,
    sendInvite,
  } from "../api";
  import {
    type AgentPubKey,
    decodeHashFromBase64,
    encodeHashToBase64,
  } from "@holochain/client";
  import Dino from "./Dino.svelte";
  import type { AuthoredDino } from "../dino_adventure/dino_adventure/types";

  let dragHighlight = $state(false);

  const handleDragover = (event: DragEvent) => {
    event.preventDefault();
    event.dataTransfer!.dropEffect = "move";
    dragHighlight = true;
  };

  const handleDinoDrop = async (event: DragEvent) => {
    event.preventDefault();
    event.stopPropagation();

    dragHighlight = false;

    const data = event.dataTransfer?.getData("text/plain");
    if (data) {
      try {
        let req: {
          name: string;
          author: string;
        } = JSON.parse(data);

        if (req.author == getAgentPubKeyB64()) {
          console.log("Cannot invite yourself");
          return;
        }

        await sendInvite({
          name: req.name,
          to: decodeHashFromBase64(req.author) as AgentPubKey,
        });
      } catch (err) {
        console.error("Failed to invite", err);
      }
    }
  };

  const openInvites = $derived.by<{ name: string; sender: AgentPubKey }[]>(
    () => {
      let openInvites = getOpenInvites();
      let dinos = getDinoState();

      return openInvites.reduce(
        (acc: { name: string; sender: AgentPubKey }[], invite) => {
          let dino = dinos.find((dino) => {
            return (
              encodeHashToBase64(dino.author) ===
              encodeHashToBase64(invite.sender)
            );
          });

          if (dino) {
            acc.push({
              name: dino.dino.name,
              sender: dino.author,
            });
          }

          return acc;
        },
        [],
      );
    },
  );

  const assembledDinos = $derived.by<AuthoredDino[]>(() => {
    let acceptedInvites = getAcceptedInvites();

    let assembledDinos = getDinoState();

    return acceptedInvites.reduce((acc: AuthoredDino[], accepted) => {
      let dino = assembledDinos.find((dino) => {
        return (
          encodeHashToBase64(dino.author) ===
          encodeHashToBase64(accepted.accepted_by)
        );
      });

      if (dino) {
        acc.push(dino);
      }

      return acc;
    }, []);
  });

  $effect(() => {
    let ourKey = getAgentPubKeyB64();
    injectOurAcceptedInvite(ourKey);
  });

  const startAdventure = async () => {
    const participants = assembledDinos.reduce((acc, assembledDino) => {
      acc.push(assembledDino.author);
      return acc;
    }, [] as AgentPubKey[]);

    try {
      await createAdventure({
        participants,
      });
    } catch (err) {
      console.error("Failed to create adventure", err);
    }
  };
</script>

<div class="grid grid-cols-3">
  <!-- Column 1 -->
  <div>
    <span class="text-lg bold px-2">Send invites</span>

    <div
      class={[
        "flex",
        "grow",
        "items-center",
        "justify-center",
        "mx-5",
        "border-dotted",
        "border-2",
        "min-h-16",
        dragHighlight ? "shadow-2xl" : "",
        dragHighlight ? "shadow-cyan-500/50" : "",
      ]}
      ondrop={handleDinoDrop}
      ondragover={handleDragover}
      ondragleave={() => (dragHighlight = false)}
      role="form"
    >
      {#if getSentInvites().length === 0}
        <p>Drop a Dino here to invite them</p>
      {:else}
        <p>
          {getSentInvites()
            .map((i) => i.name)
            .join(", ")}
        </p>
      {/if}
    </div>
  </div>

  <!-- Column 2 -->
  <div>
    <span class="text-lg bold px-2">Invites from other Dinos</span>

    <div class="flex flex-col items-center">
      {#if openInvites.length === 0}
        <div class="flex flex-col">
          <p>Waiting for invites...</p>
        </div>
      {:else}
        {#each openInvites as invite}
          <div class="flex flex-col items-center">
            <span class="text-lg">{invite.name} sent you an invite</span>
            <div class="join">
              <button
                class="join-item btn btn-primary"
                onclick={() => acceptInvite({ to: invite.sender })}
                >Accept</button
              >
              <button
                class="join-item btn btn-error"
                onclick={() => removeOpenInvite({ sender: invite.sender })}
                >Decline</button
              >
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Column 3 -->
  <div>
    <span class="text-lg bold px-2">Assembled Dinos</span>

    <div class="grid grid-cols-3">
      {#each assembledDinos as dino}
        <Dino authoredDino={dino} />
      {/each}
    </div>
  </div>
</div>

<div class="w-full mt-5 flex flex-row justify-center">
  <button
    class="btn btn-primary"
    disabled={assembledDinos.length < 2}
    onclick={startAdventure}>Begin</button
  >
</div>
