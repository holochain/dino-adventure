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

    let mut out = Vec::with_capacity(links.len());
    for link in links {
        if let Ok(action_hash) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(action_hash, GetOptions::local())?;

            if let Some(adventure) = maybe_record.map(TryInto::try_into).transpose()? {
                out.push(adventure);
            }
        }
    }

    Ok(out)
}

#[hdk_extern]
pub fn get_all_my_adventures_local() -> ExternResult<Vec<AuthoredAdventure>> {
    let current_agent = agent_info()?.agent_initial_pubkey;
    get_all_agent_adventures_local(current_agent)
}

pub fn get_all_agent_adventures_local(author: AgentPubKey) -> ExternResult<Vec<AuthoredAdventure>> {
    let path = Path::from("my_adventures");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::MyAdventures)?
            .get_options(GetStrategy::Local)
            .build(),
    )?;

    let mut out = Vec::with_capacity(links.len());
    for link in links {
        if let Ok(action_hash) = link.target.try_into() {
            let maybe_record = get::<ActionHash>(action_hash, GetOptions::local())?;

            // Ignore records not authored the current agent
            if maybe_record
                .as_ref()
                .map(|r| r.action().author() != &author)
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
