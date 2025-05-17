use crate::AppRemoteSignal;
use hdk::prelude::*;

#[hdk_extern]
pub fn test_send_ping(to_agent: AgentPubKey) -> ExternResult<()> {
    let ping = AppRemoteSignal::TestPing {
        sent_at: sys_time()?,
    };
    send_remote_signal(ping, vec![to_agent])?;
    Ok(())
}
