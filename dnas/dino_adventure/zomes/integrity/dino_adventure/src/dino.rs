use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DinoKind {
    Allosaurus,
    Ankylosaurus,
    Apatosaurus,
    Archaeopteryx,
    Brachiosaurus,
    Corythosaurus,
    Dilophosaurus,
    Dimorphodon,
    Elasmosaurus,
    Mosasaurus,
    Spinosaurus,
    Stegosaurus,
    Triceratops,
    TyrannosaurusRex,
    Velociraptor,
}

#[derive(Clone, PartialEq, Eq)]
#[hdk_entry_helper]
pub struct Dino {
    pub name: String,
    pub dino_kind: DinoKind,
}

pub fn validate_create_dino(
    _action: TypedAction<EntryCreationData>,
    _dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_dino(
    _action: TypedAction<UpdateData>,
    _dino: Dino,
    _original_action: TypedAction<EntryCreationData>,
    _original_dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino cannot be updated".to_string(),
    ))
}

pub fn validate_delete_dino(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_dinos(
    action: TypedAction<CreateLinkData>,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = action
        .data
        .target_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let record = must_get_valid_record(action_hash)?;
    let _dino: Dino = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_dinos(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino link cannot be deleted".to_string(),
    ))
}
