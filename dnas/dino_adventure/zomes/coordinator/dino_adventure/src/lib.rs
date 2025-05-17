use dino_adventure_integrity::*;
use hdk::prelude::*;

mod adventure;
mod all_adventures;
mod all_dinos;
mod dino;
mod invite;
mod nest;
mod ping;
mod types;

// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init() -> ExternResult<InitCallbackResult> {
    let mut listed = BTreeSet::new();
    listed.insert((zome_info()?.name, "recv_remote_signal".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".to_string(),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(listed),
    })?;

    Ok(InitCallbackResult::Pass)
}

// Don't modify this enum if you want the scaffolding tool to generate appropriate signals for your entries and links
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated {
        action: SignedActionHashed,
        link_type: LinkTypes,
    },
    LinkDeleted {
        action: SignedActionHashed,
        create_link_action: SignedActionHashed,
        link_type: LinkTypes,
    },
    EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    },
    AdventureInvite {
        sender: AgentPubKey,
    },
    InviteAcceptance {
        accepted_by: AgentPubKey,
    },
    IncomingPing {
        sent_at: Timestamp,
        sender: AgentPubKey,
    },
    IncomingPong {
        sender: AgentPubKey,
        round_trip_ms: i64,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AppRemoteSignal {
    AdventureInvite,
    AdventureInviteAcceptance,
    TestPing { sent_at: Timestamp },
    TestPong { ping_sent_at: Timestamp },
}

#[hdk_extern]
pub fn recv_remote_signal(signal: AppRemoteSignal) -> ExternResult<()> {
    if AppRemoteSignal::AdventureInvite == signal {
        let caller = call_info()?.provenance;
        emit_signal(Signal::AdventureInvite { sender: caller })?;
    } else if AppRemoteSignal::AdventureInviteAcceptance == signal {
        let caller = call_info()?.provenance;
        emit_signal(Signal::InviteAcceptance {
            accepted_by: caller,
        })?;
    } else if let AppRemoteSignal::TestPing { sent_at } = signal {
        let caller = call_info()?.provenance;

        // Tell the UI that we received a ping
        emit_signal(Signal::IncomingPing {
            sent_at: sent_at.clone(),
            sender: caller.clone(),
        })?;

        // Send a pong back to the caller
        send_remote_signal(
            AppRemoteSignal::TestPong {
                ping_sent_at: sent_at,
            },
            vec![caller],
        )?;
    } else if let AppRemoteSignal::TestPong { ping_sent_at } = signal {
        let caller = call_info()?.provenance;
        let round_trip_ms = sys_time()?.as_millis() - ping_sent_at.as_millis();

        // Tell the UI that we received a pong
        emit_signal(Signal::IncomingPong {
            sender: caller.clone(),
            round_trip_ms,
        })?;
    }

    Ok(())
}

// Whenever an action is committed, we emit a signal to the UI elements to reactively update them
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    // Don't modify the for loop if you want the scaffolding tool to generate appropriate signals for your entries and links
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}

// Don't modify this function if you want the scaffolding tool to generate appropriate signals for your entries and links
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type)) =
                LinkTypes::from_type(create_link.zome_index, create_link.link_type)
            {
                emit_signal(Signal::LinkCreated { action, link_type })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(delete_link.link_add_address.clone(), GetOptions::default())?.ok_or(
                wasm_error!(WasmErrorInner::Guest(
                    "Failed to fetch CreateLink action".to_string()
                )),
            )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type)) =
                        LinkTypes::from_type(create_link.zome_index, create_link.link_type)
                    {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                            create_link_action: record.signed_action.clone(),
                        })?;
                    }
                    Ok(())
                }
                _ => Err(wasm_error!(WasmErrorInner::Guest(
                    "Create Link should exist".to_string()
                ))),
            }
        }
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated { action, app_entry })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry)) =
                    get_entry_for_action(&update.original_action_address)
                {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => return Ok(None),
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => return Ok(None),
    };
    EntryTypes::deserialize_from_type(*zome_index, *entry_index, entry)
}
