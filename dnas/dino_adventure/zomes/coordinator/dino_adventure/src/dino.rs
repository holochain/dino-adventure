use crate::types::AuthoredDino;
use dino_adventure_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_dino(dino: Dino) -> ExternResult<AuthoredDino> {
    let dino_hash = create_entry(&EntryTypes::Dino(dino.clone()))?;
    let path = Path::from("all_dinos");
    create_link(
        path.path_entry_hash()?,
        dino_hash.clone(),
        LinkTypes::AllDinos,
        (),
    )?;

    let record = get(dino_hash.clone(), GetOptions::local())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Dino".to_string())
    ))?;
    record.try_into()
}

#[hdk_extern]
pub fn get_latest_dino(original_dino_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_dino_hash.clone(), LinkTypes::DinoUpdates)?.build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_dino_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_dino_hash.clone(),
    };
    get(latest_dino_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_dino(original_dino_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_dino_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn get_all_revisions_for_dino(original_dino_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_dino(original_dino_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(original_dino_hash.clone(), LinkTypes::DinoUpdates)?.build(),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "No action hash associated with link".to_string()
                    )))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDinoInput {
    pub original_dino_hash: ActionHash,
    pub previous_dino_hash: ActionHash,
    pub updated_dino: Dino,
}

#[hdk_extern]
pub fn update_dino(input: UpdateDinoInput) -> ExternResult<Record> {
    let updated_dino_hash = update_entry(input.previous_dino_hash.clone(), &input.updated_dino)?;
    create_link(
        input.original_dino_hash.clone(),
        updated_dino_hash.clone(),
        LinkTypes::DinoUpdates,
        (),
    )?;
    let record = get(updated_dino_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Dino".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_dino(original_dino_hash: ActionHash) -> ExternResult<ActionHash> {
    let path = Path::from("all_dinos");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDinos)?.build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == original_dino_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_dino_hash)
}

#[hdk_extern]
pub fn get_all_deletes_for_dino(
    original_dino_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_dino_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_dino(
    original_dino_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_dino(original_dino_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}
