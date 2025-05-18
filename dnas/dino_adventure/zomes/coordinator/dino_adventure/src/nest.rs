use crate::adventure::{get_all_agent_adventures_local, get_all_my_adventures_local};
use crate::types::{AuthoredNest, AuthoredNestBatch, CreateNestRequest, NestBatchesWithNests};
use dino_adventure_integrity::{EntryTypes, LinkTypes, Nest, NestBatch};
use hdk::prelude::*;
use rand::RngCore;
use std::collections::HashMap;

#[hdk_extern]
pub fn create_nest_batch() -> ExternResult<AuthoredNestBatch> {
    let mut all_my_adventures = get_all_my_adventures_local(())?;
    if all_my_adventures.is_empty() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "No adventures found".to_string(),
        )));
    }
    all_my_adventures.sort_by_key(|a| a.created_at);

    let my_current_adventure = all_my_adventures.last().ok_or_else(|| {
        wasm_error!(WasmErrorInner::Guest(
            "No current adventure found".to_string(),
        ))
    })?;

    let adventure_hash = create_entry(&EntryTypes::NestBatch(NestBatch))?;

    create_link(
        my_current_adventure.address.clone(),
        adventure_hash.clone(),
        LinkTypes::AdventureNestBatches,
        (),
    )?;

    let record = get(adventure_hash.clone(), GetOptions::local())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created NestBatch".to_string())
    ))?;
    (record, my_current_adventure.address.clone()).try_into()
}

#[hdk_extern]
pub fn create_nest(request: CreateNestRequest) -> ExternResult<AuthoredNest> {
    let size = rand::random::<u32>() % request.size;
    let mut payload = vec![0; size as usize];
    rand::thread_rng().fill_bytes(&mut payload);

    let nest = Nest { payload };
    let nest_hash = create_entry(EntryTypes::Nest(nest))?;
    create_link(
        request.nest_batch_address.clone(),
        nest_hash.clone(),
        LinkTypes::NestBatchNests,
        (),
    )?;

    let record = get(nest_hash.clone(), GetOptions::local())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Nest".to_string())
    ))?;
    (record, request.nest_batch_address).try_into()
}

#[hdk_extern]
pub fn get_nest_batches_with_nests_local(
    author: AgentPubKey,
) -> ExternResult<NestBatchesWithNests> {
    let mut all_agent_adventures = get_all_agent_adventures_local(author)?;
    if all_agent_adventures.is_empty() {
        return Ok(NestBatchesWithNests::default());
    }
    all_agent_adventures.sort_by_key(|a| a.created_at);

    let agent_current_adventure = all_agent_adventures.last().ok_or_else(|| {
        wasm_error!(WasmErrorInner::Guest(
            "No current adventure found".to_string(),
        ))
    })?;

    let nest_batch_links = get_links(
        GetLinksInputBuilder::try_new(
            agent_current_adventure.address.clone(),
            LinkTypes::AdventureNestBatches,
        )?
        .get_options(GetStrategy::Local)
        .build(),
    )?;

    let mut out = NestBatchesWithNests {
        nest_batches: Vec::with_capacity(nest_batch_links.len()),
        nests: HashMap::new(),
    };

    for link in nest_batch_links {
        if let Ok(target) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(target, GetOptions::local())?;

            if let Some(nest_batch) = maybe_record
                .map(|r| (r, agent_current_adventure.address.clone()))
                .map(TryInto::try_into)
                .transpose()?
            {
                out.nest_batches.push(nest_batch);

                let nest_batch_address = out.nest_batches.last().as_ref().unwrap().address.clone();

                let nest_links = get_links(
                    GetLinksInputBuilder::try_new(
                        nest_batch_address.clone(),
                        LinkTypes::NestBatchNests,
                    )?
                    .get_options(GetStrategy::Local)
                    .build(),
                )?;

                out.nests.reserve(nest_links.len());

                for nest_link in nest_links {
                    if let Ok(target) = nest_link.target.try_into() {
                        let maybe_record = get::<ActionHash>(target, GetOptions::local())?;

                        if let Some(nest) = maybe_record
                            .map(|r| (r, nest_batch_address.clone()))
                            .map(TryInto::try_into)
                            .transpose()?
                        {
                            out.nests
                                .entry(ActionHashB64::from(nest_batch_address.clone()))
                                .or_default()
                                .push(nest);
                        }
                    }
                }
            }
        };
    }

    Ok(out)
}
