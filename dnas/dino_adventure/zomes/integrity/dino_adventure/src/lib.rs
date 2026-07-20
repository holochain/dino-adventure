use hdi::prelude::*;

pub mod dino;
pub use dino::*;

pub mod adventure;
pub use adventure::*;

pub mod nest;
pub use nest::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Dino(Dino),
    Adventure(Adventure),
    NestBatch(NestBatch),
    Nest(Nest),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AllDinos,
    AllAdventures,
    MyAdventures,
    AdventureNestBatches,
    NestBatchNests,
}

// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
// There *is* access to network calls in this function
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// This is the unified validation callback for all entries and link types in this integrity zome
// Below is a match template for all of the variants of `DHT Ops` and entry and link types
// Holochain has already performed the following validation for you:
// - The action signature matches on the hash of its content and is signed by its author
// - The previous action exists, has a lower timestamp than the new action, and incremented sequence number
// - The previous action author is the same as the new action author
// - The timestamp of each action is after the DNA's origin time
// - AgentActivity authorities check that the agent hasn't forked their chain
// - The entry hash in the action matches the entry content
// - The entry type in the action matches the entry content
// - The entry size doesn't exceed the maximum entry size (currently 4MB)
// - Private entry types are not included in the Op content, and public entry types are
// - If the `Op` is an update or a delete, the original action exists and is a `Create` or `Update` action
// - If the `Op` is an update, the original entry exists and is of the same type as the new one
// - If the `Op` is a delete link, the original action exists and is a `CreateLink` action
// - Link tags don't exceed the maximum tag size (currently 1KB)
// - Countersigned entries include an action from each required signer
// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::CreateEntry(create_entry) => match create_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::Dino(dino) => validate_create_dino(action, dino),
                EntryTypes::Adventure(adventure) => validate_create_adventure(action, adventure),
                EntryTypes::NestBatch(nest_batch) => validate_create_nest_batch(action, nest_batch),
                EntryTypes::Nest(nest) => validate_create_nest(action, nest),
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::Dino(dino) => validate_create_dino(action, dino),
                EntryTypes::Adventure(adventure) => validate_create_adventure(action, adventure),
                EntryTypes::NestBatch(nest_batch) => validate_create_nest_batch(action, nest_batch),
                EntryTypes::Nest(nest) => validate_create_nest(action, nest),
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::Update(update_entry) => match update_entry {
            OpUpdate::Entry { app_entry, action } => {
                let update = match &action.data {
                    ActionData::Update(update) => update,
                    _ => unreachable!(),
                };

                let original_action = must_get_action(update.original_action_address.clone())?
                    .action()
                    .to_owned();
                let original_create_action = match &original_action.data {
                    ActionData::Create(_) => original_action,
                    ActionData::Update(_) => original_action,
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(format!(
                            "Expected to get create/update action: {original_action:?}"
                        )));
                    }
                };
                match app_entry {
                    EntryTypes::Dino(dino) => {
                        let original_app_entry =
                            must_get_valid_record(update.original_action_address.clone())?;
                        let original_dino = match Dino::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Dino from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_dino(
                            update.clone(),
                            dino,
                            original_create_action,
                            original_dino,
                        )
                    }
                    EntryTypes::Adventure(adventure) => {
                        let original_app_entry =
                            must_get_valid_record(update.original_action_address.clone())?;
                        let original_adventure = match Adventure::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Adventure from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_adventure(
                            update.clone(),
                            adventure,
                            original_create_action,
                            original_adventure,
                        )
                    }
                    EntryTypes::NestBatch(nest_batch) => {
                        let original_app_entry =
                            must_get_valid_record(update.original_action_address.clone())?;
                        let original_nest_batch = match NestBatch::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get NestBatch from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_nest_batch(
                            update.clone(),
                            nest_batch,
                            original_create_action,
                            original_nest_batch,
                        )
                    }
                    EntryTypes::Nest(nest) => {
                        let original_app_entry =
                            must_get_valid_record(update.original_action_address.clone())?;
                        let original_nest = match Nest::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get Nest from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_nest(
                            update.clone(),
                            nest,
                            original_create_action,
                            original_nest,
                        )
                    }
                }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::Delete(delete_entry) => {
            let delete = match &delete_entry.action.data {
                ActionData::Delete(delete) => delete,
                _ => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected to get delete action: {:?}",
                        delete_entry.action
                    )));
                }
            };

            let original_action_hash = delete.deletes_address.clone();
            let original_record = must_get_valid_record(original_action_hash)?;
            let original_record_action = original_record.action().clone();
            let original_action = match &original_record_action.data {
                ActionData::Create(_) => original_record_action,
                ActionData::Update(_) => original_record_action,
                _ => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected to get create/update action: {original_record_action:?}"
                    )));
                }
            };
            let app_entry_type = match original_action.entry_type() {
                Some(EntryType::App(app_entry_type)) => app_entry_type,
                _ => {
                    return Ok(ValidateCallbackResult::Valid);
                }
            };
            let entry = match original_record.entry().as_option() {
                Some(entry) => entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original record for a delete must contain an entry".to_string(),
                    ));
                }
            };
            let original_app_entry = match EntryTypes::deserialize_from_type(
                app_entry_type.zome_index,
                app_entry_type.entry_index,
                entry,
            )? {
                Some(app_entry) => app_entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original app entry must be one of the defined entry types for this zome"
                            .to_string(),
                    ));
                }
            };
            match original_app_entry {
                EntryTypes::Dino(original_dino) => {
                    validate_delete_dino(delete.clone(), original_action, original_dino)
                }
                EntryTypes::Adventure(original_adventure) => {
                    validate_delete_adventure(delete.clone(), original_action, original_adventure)
                }
                EntryTypes::NestBatch(original_nest_batch) => {
                    validate_delete_nest_batch(delete.clone(), original_action, original_nest_batch)
                }
                EntryTypes::Nest(original_nest) => {
                    validate_delete_nest(delete.clone(), original_action, original_nest)
                }
            }
        }
        FlatOp::Link(OpLink::CreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        }) => {
            let create_link = match &action.data {
                ActionData::CreateLink(create_link) => create_link.clone(),
                _ => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected a create link action: {action:?}"
                    )));
                }
            };

            match link_type {
                LinkTypes::AllDinos => {
                    validate_create_link_all_dinos(create_link, base_address, target_address, tag)
                }
                LinkTypes::AllAdventures => validate_create_link_all_adventures(
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::MyAdventures => validate_create_link_my_adventures(
                    action.header,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::AdventureNestBatches => validate_create_link_adventure_nest_batch(
                    action.header,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::NestBatchNests => validate_create_link_nest_batch_nest(
                    action.header,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
            }
        }
        FlatOp::Link(OpLink::DeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        }) => {
            let delete_link = match &action.data {
                ActionData::DeleteLink(delete_link) => delete_link.clone(),
                _ => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected delete link action: {action:?}"
                    )));
                }
            };

            let create_link = match &original_action.data {
                ActionData::CreateLink(create_link) => create_link.clone(),
                _ => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected previous action to be a create link action {original_action:?}"
                    )));
                }
            };

            match link_type {
                LinkTypes::AllDinos => validate_delete_link_all_dinos(
                    delete_link,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::AllAdventures => validate_delete_link_all_adventures(
                    delete_link,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::MyAdventures => validate_delete_link_my_adventures(
                    action.header,
                    delete_link,
                    original_action.header,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::AdventureNestBatches => validate_delete_link_adventure_nest_batch(
                    delete_link,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::NestBatchNests => validate_delete_link_nest_batch_nest(
                    delete_link,
                    create_link,
                    base_address,
                    target_address,
                    tag,
                ),
            }
        }
        FlatOp::CreateRecord(store_record) => {
            match store_record {
                // Complementary validation to the `StoreEntry` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `StoreEntry` validation failed
                OpRecord::CreateEntry { app_entry, action } => match app_entry {
                    EntryTypes::Dino(dino) => validate_create_dino(action, dino),
                    EntryTypes::Adventure(adventure) => {
                        validate_create_adventure(action, adventure)
                    }
                    EntryTypes::NestBatch(nest_batch) => {
                        validate_create_nest_batch(action, nest_batch)
                    }
                    EntryTypes::Nest(nest) => validate_create_nest(action, nest),
                },
                // Complementary validation to the `RegisterUpdate` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry` and in `RegisterUpdate`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let update_data = match &action.data {
                        ActionData::Update(update_data) => update_data,
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(format!(
                                "Expected an update action: {action:?}"
                            )))
                        }
                    };

                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match &original_action.data {
                        ActionData::Create(_) => original_action,
                        ActionData::Update(_) => original_action,
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for an update must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    match app_entry {
                        EntryTypes::Dino(dino) => {
                            let result = validate_create_dino(action.clone(), dino.clone())?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_dino: Option<Dino> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_dino = match original_dino {
                                    Some(dino) => dino,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_dino(
                                    update_data.clone(),
                                    dino,
                                    original_action,
                                    original_dino,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Adventure(adventure) => {
                            let result =
                                validate_create_adventure(action.clone(), adventure.clone())?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_adventure: Option<Adventure> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_adventure = match original_adventure {
                                    Some(adventure) => adventure,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_adventure(
                                    update_data.clone(),
                                    adventure,
                                    original_action,
                                    original_adventure,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::NestBatch(nest_batch) => {
                            let result =
                                validate_create_nest_batch(action.clone(), nest_batch.clone())?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_nest_batch: Option<NestBatch> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_nest_batch = match original_nest_batch {
                                    Some(nest_batch) => nest_batch,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_nest_batch(
                                    update_data.clone(),
                                    nest_batch,
                                    original_action,
                                    original_nest_batch,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Nest(nest) => {
                            let result = validate_create_nest(action.clone(), nest.clone())?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_nest: Option<Nest> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_nest = match original_nest {
                                    Some(nest) => nest,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_nest(
                                    update_data.clone(),
                                    nest,
                                    original_action,
                                    original_nest,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                // Complementary validation to the `RegisterDelete` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDelete`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDelete` validation failed
                OpRecord::DeleteEntry {
                    original_action_hash,
                    action,
                    ..
                } => {
                    let delete_data = match &action.data {
                        ActionData::Delete(delete_data) => delete_data,
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(format!(
                                "Expected a delete action: {action:?}"
                            )))
                        }
                    };

                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match &original_action.data {
                        ActionData::Create(_) => original_action,
                        ActionData::Update(_) => original_action,
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for a delete must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        Some(EntryType::App(app_entry_type)) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original record for a delete must contain an entry".to_string(),
                            ));
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index,
                        app_entry_type.entry_index,
                        entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Dino(original_dino) => validate_delete_dino(
                            delete_data.clone(),
                            original_action,
                            original_dino,
                        ),
                        EntryTypes::Adventure(original_adventure) => validate_delete_adventure(
                            delete_data.clone(),
                            original_action,
                            original_adventure,
                        ),
                        EntryTypes::NestBatch(original_nest_batch) => validate_delete_nest_batch(
                            delete_data.clone(),
                            original_action,
                            original_nest_batch,
                        ),
                        EntryTypes::Nest(original_nest) => validate_delete_nest(
                            delete_data.clone(),
                            original_action,
                            original_nest,
                        ),
                    }
                }
                // Complementary validation to the `RegisterCreateLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterCreateLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterCreateLink` validation failed
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    let create_link = match &action.data {
                        ActionData::CreateLink(create_link) => create_link,
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(format!(
                                "Expected a create link action {action:?}"
                            )))
                        }
                    };

                    match link_type {
                        LinkTypes::AllDinos => validate_create_link_all_dinos(
                            create_link.clone(),
                            base_address,
                            target_address,
                            tag,
                        ),
                        LinkTypes::AllAdventures => validate_create_link_all_adventures(
                            create_link.clone(),
                            base_address,
                            target_address,
                            tag,
                        ),
                        LinkTypes::MyAdventures => validate_create_link_my_adventures(
                            action.header,
                            create_link.clone(),
                            base_address,
                            target_address,
                            tag,
                        ),
                        LinkTypes::AdventureNestBatches => {
                            validate_create_link_adventure_nest_batch(
                                action.header,
                                create_link.clone(),
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::NestBatchNests => validate_create_link_nest_batch_nest(
                            action.header,
                            create_link.clone(),
                            base_address,
                            target_address,
                            tag,
                        ),
                    }
                }
                // Complementary validation to the `RegisterDeleteLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDeleteLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDeleteLink` validation failed
                OpRecord::DeleteLink {
                    original_action_hash,
                    base_address,
                    action,
                } => {
                    let delete_link = match &action.data {
                        ActionData::DeleteLink(delete_link) => delete_link.clone(),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(format!(
                                "Expected a delete link action: {action:?}"
                            )))
                        }
                    };

                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match &record.action().data {
                        ActionData::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "The action that a DeleteLink deletes must be a CreateLink"
                                    .to_string(),
                            ));
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index,
                        create_link.link_type,
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::AllDinos => validate_delete_link_all_dinos(
                            delete_link,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllAdventures => validate_delete_link_all_adventures(
                            delete_link,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::MyAdventures => validate_delete_link_my_adventures(
                            action.header,
                            delete_link,
                            record.action().header.clone(),
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AdventureNestBatches => {
                            validate_delete_link_adventure_nest_batch(
                                delete_link,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::NestBatchNests => validate_delete_link_nest_batch_nest(
                            delete_link,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::AgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let prev = action
                    .prev_action()
                    .ok_or_else(|| {
                        wasm_error!(WasmErrorInner::Guest("expected a prior action".into()))
                    })?
                    .clone();

                let previous_action = must_get_action(prev)?;
                match &previous_action.action().data {
                        ActionData::AgentValidationPkg(
                            AgentValidationPkgData { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
