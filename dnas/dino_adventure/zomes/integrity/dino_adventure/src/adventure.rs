use hdi::prelude::*;

#[derive(Clone, PartialEq, Eq)]
#[hdk_entry_helper]
pub struct Adventure {
    pub participants: Vec<AgentPubKey>,
}

pub fn validate_create_adventure(
    _action: TypedAction<EntryCreationData>,
    adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    if adventure.participants.len() < 2 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Adventure must have at least 2 participants".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_adventure(
    _action: TypedAction<UpdateData>,
    _adventure: Adventure,
    _original_action: TypedAction<EntryCreationData>,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be updated".to_string(),
    ))
}

pub fn validate_delete_adventure(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_adventures(
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
    let _adventure: Adventure = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_adventures(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_my_adventures(
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
    let _adventure: Adventure = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    if &action.header.author != record.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can link their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_my_adventures(
    action: TypedAction<DeleteLinkData>,
    original_action: TypedAction<CreateLinkData>,
) -> ExternResult<ValidateCallbackResult> {
    if action.header.author != original_action.header.author {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can delete their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
