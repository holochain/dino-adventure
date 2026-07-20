use crate::Adventure;
use hdi::prelude::*;

#[derive(Clone, PartialEq, Eq)]
#[hdk_entry_helper]
pub struct NestBatch;

#[derive(Clone, PartialEq, Eq)]
#[hdk_entry_helper]
pub struct Nest {
    pub payload: Vec<u8>,
}

pub fn validate_create_nest_batch(
    _action: Action,
    _nest_batch: NestBatch,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_nest_batch(
    _action: UpdateData,
    _nest_batch: NestBatch,
    _original_action: Action,
    _original_nest_batch: NestBatch,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nest batches cannot be updated".to_string(),
    ))
}

pub fn validate_delete_nest_batch(
    _action: DeleteData,
    _original_action: Action,
    _original_nest_batch: NestBatch,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nest batches cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_adventure_nest_batch(
    action_header: ActionHeader,
    _action: CreateLinkData,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
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

    if &action_header.author != record.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Author of the link must be the same as the author of the adventure".to_string()
        )));
    }

    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _nest_batch: NestBatch = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    if &action_header.author != record.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Author of the link must be the same as the author of the nest batch".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_adventure_nest_batch(
    _action: DeleteLinkData,
    _original_action: CreateLinkData,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nest batch cannot be unlinked from their adventure".to_string(),
    ))
}

pub fn validate_create_nest(_action: Action, _nest: Nest) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_nest(
    _action: UpdateData,
    _nest: Nest,
    _original_action: Action,
    _original_nest: Nest,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nests cannot be updated".to_string(),
    ))
}

pub fn validate_delete_nest(
    _action: DeleteData,
    _original_action: Action,
    _original_nest: Nest,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nests cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_nest_batch_nest(
    action_header: ActionHeader,
    _action: CreateLinkData,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;

    let record = must_get_valid_record(action_hash)?;
    let _nest_batch: NestBatch = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    if &action_header.author != record.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Author of the link must be the same as the author of the nest batch".to_string()
        )));
    }

    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _nest: Nest = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    if &action_header.author != record.action().author() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Author of the link must be the same as the author of the nest".to_string()
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_nest_batch_nest(
    _action: DeleteLinkData,
    _original_action: CreateLinkData,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Nest cannot be unlinked from their nest batch".to_string(),
    ))
}
