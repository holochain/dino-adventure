use crate::AppRemoteSignal;
use hdk::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdventureInvite {
    to: AgentPubKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdventureInviteAcceptance {
    to: AgentPubKey,
}

#[hdk_extern]
pub fn send_invite(invite: AdventureInvite) -> ExternResult<()> {
    send_remote_signal(AppRemoteSignal::AdventureInvite, vec![invite.to])
}

#[hdk_extern]
pub fn accept_invite(acceptance: AdventureInviteAcceptance) -> ExternResult<()> {
    send_remote_signal(
        AppRemoteSignal::AdventureInviteAcceptance,
        vec![acceptance.to],
    )
}
