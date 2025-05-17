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
pub fn get_all_dinos() -> ExternResult<Vec<AuthoredDino>> {
    get_all_dinos_with_options(GetOptions::network())
}

#[hdk_extern]
pub fn get_all_dinos_local() -> ExternResult<Vec<AuthoredDino>> {
    get_all_dinos_with_options(GetOptions::local())
}

fn get_all_dinos_with_options(get_options: GetOptions) -> ExternResult<Vec<AuthoredDino>> {
    let path = Path::from("all_dinos");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDinos)?
            .get_options(get_options.strategy)
            .build(),
    )?;

    let mut out = Vec::with_capacity(links.len());
    for link in links {
        if let Ok(action_hash) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(action_hash, get_options.clone())?;

            if let Some(dino) = maybe_record.map(TryInto::try_into).transpose()? {
                out.push(dino);
            }
        }
    }

    Ok(out)
}
