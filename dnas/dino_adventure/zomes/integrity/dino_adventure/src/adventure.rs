use hdi::prelude::*;

#[derive(Clone, PartialEq, Eq)]
#[hdk_entry_helper]
pub struct Adventure {
    pub participants: Vec<AgentPubKey>,
}

pub fn validate_create_adventure(
    _action: Action,
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
    _action: UpdateData,
    _adventure: Adventure,
    _original_action: Action,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be updated".to_string(),
    ))
}

pub fn validate_delete_adventure(
    _action: DeleteData,
    _original_action: Action,
    _original_adventure: Adventure,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_adventures(
    _action: CreateLinkData,
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
    _action: DeleteLinkData,
    _original_action: CreateLinkData,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Adventure cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_my_adventures(
    action_header: ActionHeader,
    _action: CreateLinkData,
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

    if &action_header.author != record.signed_action.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can link their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_my_adventures(
    action_header: ActionHeader,
    _action: DeleteLinkData,
    original_action_header: ActionHeader,
    _original_action: CreateLinkData,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action_header.author != original_action_header.author {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only the author can delete their own adventure".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
