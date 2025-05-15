use dino_adventure_integrity::{Adventure, Dino};
use hdk::prelude::*;

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
                address: value.action_address().clone(),
            }),
            None => Err(wasm_error!(WasmErrorInner::Guest(
                "Not an Adventure Record".to_string()
            ))),
        }
    }
}
