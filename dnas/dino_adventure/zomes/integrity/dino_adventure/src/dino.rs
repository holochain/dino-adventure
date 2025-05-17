use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Dino {
    pub name: String,
    pub dino_kind: DinoKind,
}

pub fn validate_create_dino(
    _action: EntryCreationAction,
    _dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_dino(
    _action: Update,
    _dino: Dino,
    _original_action: EntryCreationAction,
    _original_dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino cannot be updated".to_string(),
    ))
}

pub fn validate_delete_dino(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_dino: Dino,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_dinos(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash =
        target_address
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
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Dino link cannot be deleted".to_string(),
    ))
}
