use crate::types::AuthoredAdventure;
use dino_adventure_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_adventure(adventure: Adventure) -> ExternResult<AuthoredAdventure> {
    let adventure_hash = create_entry(&EntryTypes::Adventure(adventure.clone()))?;
    let path = Path::from("all_adventures");
    create_link(
        path.path_entry_hash()?,
        adventure_hash.clone(),
        LinkTypes::AllAdventures,
        (),
    )?;
    let path = Path::from("my_adventures");
    create_link(
        path.path_entry_hash()?,
        adventure_hash.clone(),
        LinkTypes::MyAdventures,
        (),
    )?;

    let record = get(adventure_hash.clone(), GetOptions::local())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Adventure".to_string())
    ))?;
    record.try_into()
}

#[hdk_extern]
pub fn delete_adventure(adventure_hash: ActionHash) -> ExternResult<ActionHash> {
    unlink_adventure(adventure_hash.clone())?;
    unlink_my_adventure(adventure_hash.clone())?;
    delete_entry(adventure_hash)
}

pub fn unlink_adventure(adventure_hash: ActionHash) -> ExternResult<()> {
    let path = Path::from("all_adventures");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAdventures)?.build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == adventure_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }

    Ok(())
}

#[hdk_extern]
pub fn unlink_my_adventure(adventure_hash: ActionHash) -> ExternResult<()> {
    let path = Path::from("my_adventures");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::MyAdventures)?.build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == adventure_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }

    Ok(())
}
