use crate::types::AuthoredAdventure;
use dino_adventure_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_all_adventures_local() -> ExternResult<Vec<AuthoredAdventure>> {
    let path = Path::from("all_adventures");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAdventures)?
            .get_options(GetStrategy::Local)
            .build(),
    )?;

    let myself = agent_info()?.agent_initial_pubkey;
    let mut out = Vec::with_capacity(links.len());
    for link in links {
        if let Ok(action_hash) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(action_hash, GetOptions::local())?;

            // Skip records that weren't authored by the current agent, we only want to see our own
            // adventures in the UI.
            if maybe_record
                .as_ref()
                .map(|r| r.action().author() != &myself)
                .unwrap_or(false)
            {
                continue;
            }

            if let Some(adventure) = maybe_record.map(TryInto::try_into).transpose()? {
                out.push(adventure);
            }
        }
    }

    Ok(out)
}
