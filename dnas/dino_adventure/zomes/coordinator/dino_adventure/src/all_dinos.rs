use crate::types::AuthoredDino;
use dino_adventure_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_all_dinos_local() -> ExternResult<Vec<AuthoredDino>> {
    let path = Path::from("all_dinos");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDinos)?
            .get_options(GetStrategy::Local)
            .build(),
    )?;

    let mut out = Vec::with_capacity(links.len());
    for link in links {
        if let Ok(action_hash) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(action_hash, GetOptions::local())?;

            if let Some(dino) = maybe_record.map(TryInto::try_into).transpose()? {
                out.push(dino);
            }
        }
    }

    Ok(out)
}
