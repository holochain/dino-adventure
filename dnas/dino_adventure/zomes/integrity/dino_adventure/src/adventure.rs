use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Adventure {
    pub participants: Vec<AgentPubKey>,
}

pub fn validate_create_adventure(
    _action: EntryCreationAction,
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
    _action: Update,
    _adventure: Adventure,
    _original_action: EntryCreationAction,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be updated".to_string(),
    ))
}

pub fn validate_delete_adventure(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_adventures(
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
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_my_adventures(
    action: CreateLink,
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
    let _adventure: Adventure = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    if &action.author != record.signed_action.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can link their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_my_adventures(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can delete their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
