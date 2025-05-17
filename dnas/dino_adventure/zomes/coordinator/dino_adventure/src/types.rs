use dino_adventure_integrity::{Adventure, Dino, Nest, NestBatch};
use hdk::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthoredDino {
    pub dino: Dino,
    pub author: AgentPubKey,
    pub address: ActionHash,
}

impl TryFrom<Record> for AuthoredDino {
    type Error = WasmError;

    fn try_from(value: Record) -> ExternResult<Self> {
        let maybe_dino: Option<Dino> = value.entry.to_app_option().map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize Dino: {:?}",
                e
            )))
        })?;

        match maybe_dino {
            Some(dino) => Ok(AuthoredDino {
                dino,
                author: value.action().author().clone(),
                address: value.action_address().clone(),
            }),
            None => Err(wasm_error!(WasmErrorInner::Guest(
                "Not a Dino Record".to_string()
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthoredAdventure {
    pub adventure: Adventure,
    pub author: AgentPubKey,
    pub created_at: Timestamp,
    pub address: ActionHash,
}

impl TryFrom<Record> for AuthoredAdventure {
    type Error = WasmError;

    fn try_from(value: Record) -> ExternResult<Self> {
        let maybe_adventure: Option<Adventure> = value.entry.to_app_option().map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize Adventure: {:?}",
                e
            )))
        })?;

        match maybe_adventure {
            Some(adventure) => Ok(AuthoredAdventure {
                adventure,
                author: value.action().author().clone(),
                created_at: value.action().timestamp().clone(),
                address: value.action_address().clone(),
            }),
            None => Err(wasm_error!(WasmErrorInner::Guest(
                "Not an Adventure Record".to_string()
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthoredNestBatch {
    pub nest_batch: NestBatch,
    pub author: AgentPubKey,
    pub created_at: Timestamp,
    pub address: ActionHash,
    pub adventure_address: ActionHash,
}

impl TryFrom<(Record, ActionHash)> for AuthoredNestBatch {
    type Error = WasmError;

    fn try_from((value, adventure_address): (Record, ActionHash)) -> ExternResult<Self> {
        let maybe_nest_batch: Option<NestBatch> = value.entry.to_app_option().map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize NestBatch: {:?}",
                e
            )))
        })?;

        match maybe_nest_batch {
            Some(nest_batch) => Ok(AuthoredNestBatch {
                nest_batch,
                author: value.action().author().clone(),
                created_at: value.action().timestamp().clone(),
                address: value.action_address().clone(),
                adventure_address,
            }),
            None => Err(wasm_error!(WasmErrorInner::Guest(
                "Not a NestBatch Record".to_string()
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthoredNest {
    pub nest: Nest,
    pub author: AgentPubKey,
    pub address: ActionHash,
    pub nest_batch_address: ActionHash,
}

impl TryFrom<(Record, ActionHash)> for AuthoredNest {
    type Error = WasmError;

    fn try_from((value, nest_batch_address): (Record, ActionHash)) -> ExternResult<Self> {
        let maybe_nest: Option<Nest> = value.entry.to_app_option().map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize Nest: {:?}",
                e
            )))
        })?;

        match maybe_nest {
            Some(nest) => Ok(AuthoredNest {
                nest,
                author: value.action().author().clone(),
                address: value.action_address().clone(),
                nest_batch_address,
            }),
            None => Err(wasm_error!(WasmErrorInner::Guest(
                "Not a Nest Record".to_string()
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NestBatchesWithNests {
    pub nest_batches: Vec<AuthoredNestBatch>,
    pub nests: HashMap<ActionHashB64, Vec<AuthoredNest>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNestRequest {
    pub nest_batch_address: ActionHash,
    pub size: u32,
}
