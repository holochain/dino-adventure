import type {
  AdventureInvite,
  AdventureInviteAcceptance,
} from "../dino_adventure/dino_adventure/types";
import { runOnClient, signalHandler } from "./common.svelte";
import {
  type AgentPubKey,
  decodeHashFromBase64,
  encodeHashToBase64,
} from "@holochain/client";

/**
 * Invites that we have received but not yet accepted.
 */
let openInvites = $state<{ sender: AgentPubKey }[]>([]);

/**
 * Invites that we have sent to other dinos.
 */
let sentInvites = $state<
  {
    name: string;
    to: AgentPubKey;
  }[]
>([]);

/**
 * Responses for invites that we've sent that were accepted.
 */
let acceptedInvites = $state<{ accepted_by: AgentPubKey }[]>([]);

export const getOpenInvites = () => openInvites;

export const getSentInvites = () => sentInvites;

export const getAcceptedInvites = () => acceptedInvites;

export const removeOpenInvite = (invite: { sender: AgentPubKey }) => {
  openInvites = openInvites.filter(
    (existingInvite) =>
      encodeHashToBase64(existingInvite.sender) !==
      encodeHashToBase64(invite.sender),
  );
};

const removeSentInvite = (invite: { to: AgentPubKey }) => {
  sentInvites = sentInvites.filter(
    (existingInvite) =>
      encodeHashToBase64(existingInvite.to) !== encodeHashToBase64(invite.to),
  );
};

export const sendInvite = async (invite: {
  name: string;
  to: AgentPubKey;
}): Promise<void> => {
  return runOnClient(async (client) => {
    try {
      const hasAcceptedInvite =
        acceptedInvites.find((existingInvite) => {
          return (
            encodeHashToBase64(existingInvite.accepted_by) ===
            encodeHashToBase64(invite.to)
          );
        }) != undefined;
      if (hasAcceptedInvite) {
        console.log("Already have an accepted invite for this dino");
        return;
      }

      const hashSentInvite = sentInvites.find((existingInvite) => {
        return (
          encodeHashToBase64(existingInvite.to) ===
          encodeHashToBase64(invite.to)
        );
      });
      if (hashSentInvite) {
        console.log("Already have a pending invite for this dino");
        return;
      }

      await client.callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "send_invite",
        payload: {
          to: invite.to,
        } as AdventureInvite,
      });

      sentInvites = [...sentInvites, invite];

      // Remove sent invites after 30 seconds, to allow the invite to be resent
      setTimeout(() => {
        sentInvites = sentInvites.filter((existingInvite) => {
          return (
            encodeHashToBase64(existingInvite.to) !==
            encodeHashToBase64(invite.to)
          );
        });
      }, 30_000);
    } catch (err) {
      console.error("Error sending invite", err);
    }
  });
};

export const acceptInvite = async (invite: AdventureInviteAcceptance) => {
  return runOnClient(async (client) => {
    try {
      await client.callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "accept_invite",
        payload: invite,
      });

      removeOpenInvite({
        sender: invite.to,
      });
    } catch (err) {
      console.error("Error accepting invite", err);
    }
  });
};

export const injectOurAcceptedInvite = (ourKey: string) => {
  const alreadyInAccepted = acceptedInvites.find((invite) => {
    return encodeHashToBase64(invite.accepted_by) === ourKey;
  });

  if (alreadyInAccepted) {
    return;
  }

  try {
    acceptedInvites = [
      ...acceptedInvites,
      {
        accepted_by: decodeHashFromBase64(ourKey),
      },
    ];
  } catch (err) {
    console.error("Error injecting our accepted invite", err);
  }
};

(() => {
  signalHandler.addSignalHandler(
    "dino_adventure:AdventureInvite",
    (request: { sender: AgentPubKey }) => {
      const haveInvite =
        openInvites.find(
          (invite) =>
            encodeHashToBase64(invite.sender) ===
            encodeHashToBase64(request.sender),
        ) != undefined;

      if (!haveInvite) {
        openInvites = [...openInvites, request];

        // Remove open invites after 30 seconds, because the invite expires for the sender
        // after that, and they will ignore the response anyway.
        setTimeout(() => {
          openInvites = openInvites.filter((invite) => {
            return (
              encodeHashToBase64(invite.sender) !==
              encodeHashToBase64(request.sender)
            );
          });
        }, 30_000);
      }
    },
  );

  signalHandler.addSignalHandler(
    "dino_adventure:InviteAcceptance",
    (request: { accepted_by: AgentPubKey }) => {
      const haveMatchingInvite =
        sentInvites.find((invite) => {
          return (
            encodeHashToBase64(invite.to) ===
            encodeHashToBase64(request.accepted_by)
          );
        }) != undefined;

      if (haveMatchingInvite) {
        removeSentInvite({
          to: request.accepted_by,
        });

        acceptedInvites = [
          ...acceptedInvites,
          {
            accepted_by: request.accepted_by,
          },
        ];
      }
    },
  );
})();
