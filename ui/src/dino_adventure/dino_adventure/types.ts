import type {
  ActionHash,
  AgentPubKey,
  Create,
  CreateLink,
  Delete,
  DeleteLink,
  SignedActionHashed,
  Update,
} from "@holochain/client";

export type DinoAdventureSignal =
  | {
      type: "EntryCreated";
      action: SignedActionHashed<Create>;
      app_entry: EntryTypes;
    }
  | {
      type: "EntryUpdated";
      action: SignedActionHashed<Update>;
      app_entry: EntryTypes;
      original_app_entry: EntryTypes;
    }
  | {
      type: "EntryDeleted";
      action: SignedActionHashed<Delete>;
      original_app_entry: EntryTypes;
    }
  | {
      type: "LinkCreated";
      action: SignedActionHashed<CreateLink>;
      link_type: string;
    }
  | {
      type: "LinkDeleted";
      action: SignedActionHashed<DeleteLink>;
      link_type: string;
    }
  | {
      type: "AdventureInvite";
      sender: AgentPubKey;
    }
  | {
      type: "InviteAcceptance";
      accepted_by: AgentPubKey;
    };

export type EntryTypes = { type: "Dino" } & Dino;

export const DinoKinds = [
  "Allosaurus",
  "Ankylosaurus",
  "Apatosaurus",
  "Archaeopteryx",
  "Brachiosaurus",
  "Corythosaurus",
  "Dilophosaurus",
  "Dimorphodon",
  "Elasmosaurus",
  "Mosasaurus",
  "Spinosaurus",
  "Stegosaurus",
  "Triceratops",
  "TyrannosaurusRex",
  "Velociraptor",
] as const;

export type DinoKind = { type: (typeof DinoKinds)[number] };

export interface Dino {
  name: string;
  dino_kind: DinoKind;
}

export interface AuthoredDino {
  dino: Dino;
  author: AgentPubKey;
  address: ActionHash;
}

export interface AdventureInvite {
  to: AgentPubKey;
}

export interface AdventureInviteAcceptance {
  to: AgentPubKey;
}

export interface Adventure {
  participants: AgentPubKey[];
}

export interface AuthoredAdventure {
  adventure: Adventure;
  author: AgentPubKey;
  address: ActionHash;
}
