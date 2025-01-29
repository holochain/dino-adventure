use dino_adventure_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_all_dinos() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_dinos");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDinos)?.build())
}
