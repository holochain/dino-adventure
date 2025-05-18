import type {
  ActionHash,
  ActionHashB64,
  AgentPubKey,
  Create,
  CreateLink,
  Delete,
  DeleteLink,
  SignedActionHashed,
  Timestamp,
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
    }
  | {
      type: "IncomingPing";
      sent_at: number;
      sender: AgentPubKey;
    }
  | {
      type: "IncomingPong";
      sender: AgentPubKey;
      round_trip_ms: number;
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
  created_at: Timestamp;
  address: ActionHash;
}

export interface AuthoredNestBatch {
  nest_batch: object;
  author: AgentPubKey;
  created_at: Timestamp;
  address: ActionHash;
  adventure_address: ActionHash;
}

export interface Nest {
  payload: number[];
}

export interface AuthoredNest {
  nest: Nest;
  author: AgentPubKey;
  created_at: Timestamp;
  address: ActionHash;
  nest_batch_address: ActionHash;
}

export interface NestBatchesWithNests {
  nest_batches: AuthoredNestBatch[];
  nests: Record<ActionHashB64, AuthoredNest[]>;
}

export interface CreateNestRequest {
  nest_batch_address: ActionHash;
  size: number;
}
