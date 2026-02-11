use dino_adventure::Signal as DinoAdventureSignal;
use holochain::prelude::*;
use holochain::sweettest::*;
use holochain_client::{AdminWebsocket, AppWebsocket, ClientAgentSigner};
use holochain_types::websocket::AllowedOrigins;
use std::net::Ipv4Addr;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn send_ping() {
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_cell_id = cells[0].cell_id().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_cell_id = cells[1].cell_id().clone();

    // Get installed app IDs
    let installed_app_id_alice = apps.iter().next().unwrap().installed_app_id().clone();
    let installed_app_id_bob = apps.iter().nth(1).unwrap().installed_app_id().clone();

    // Set up admin and app websockets for Alice
    let alice_admin_port = alice_conductor
        .get_arbitrary_admin_websocket_port()
        .unwrap();
    let alice_admin_ws = AdminWebsocket::connect((Ipv4Addr::LOCALHOST, alice_admin_port), None)
        .await
        .unwrap();
    let alice_app_port = alice_admin_ws
        .attach_app_interface(0, None, AllowedOrigins::Any, None)
        .await
        .unwrap();
    let alice_token = alice_admin_ws
        .issue_app_auth_token(installed_app_id_alice.into())
        .await
        .unwrap();
    let alice_signer = ClientAgentSigner::default();
    let alice_app_ws = AppWebsocket::connect(
        (Ipv4Addr::LOCALHOST, alice_app_port),
        alice_token.token,
        alice_signer.into(),
        None,
    )
    .await
    .unwrap();

    // Set up admin and app websockets for Bob
    let bob_admin_port = bob_conductor.get_arbitrary_admin_websocket_port().unwrap();
    let bob_admin_ws = AdminWebsocket::connect((Ipv4Addr::LOCALHOST, bob_admin_port), None)
        .await
        .unwrap();
    let bob_app_port = bob_admin_ws
        .attach_app_interface(0, None, AllowedOrigins::Any, None)
        .await
        .unwrap();
    let bob_token = bob_admin_ws
        .issue_app_auth_token(installed_app_id_bob.into())
        .await
        .unwrap();
    let bob_signer = ClientAgentSigner::default();
    let bob_app_ws = AppWebsocket::connect(
        (Ipv4Addr::LOCALHOST, bob_app_port),
        bob_token.token,
        bob_signer.into(),
        None,
    )
    .await
    .unwrap();

    // Set up signal handlers
    let bob_received_ping = Arc::new(AtomicBool::new(false));
    let bob_received_ping_clone = bob_received_ping.clone();
    let alice_pub_key_clone = alice_cell_id.agent_pubkey().clone();
    bob_app_ws
        .on_signal(move |signal| {
            if let Signal::App { signal, .. } = signal {
                if let Ok(DinoAdventureSignal::IncomingPing { sender, .. }) =
                    signal.clone().into_inner().decode::<DinoAdventureSignal>()
                {
                    if sender == alice_pub_key_clone {
                        bob_received_ping_clone.store(true, Ordering::SeqCst);
                    }
                }
            }
        })
        .await;

    let alice_received_pong = Arc::new(AtomicBool::new(false));
    let alice_received_pong_clone = alice_received_pong.clone();
    let bob_pub_key_clone = bob_cell_id.agent_pubkey().clone();
    alice_app_ws
        .on_signal(move |signal| {
            if let Signal::App { signal, .. } = signal {
                if let Ok(DinoAdventureSignal::IncomingPong { sender, .. }) =
                    signal.clone().into_inner().decode::<DinoAdventureSignal>()
                {
                    if sender == bob_pub_key_clone {
                        alice_received_pong_clone.store(true, Ordering::SeqCst);
                    }
                }
            }
        })
        .await;

    // Alice sends a ping to Bob
    let _: () = alice_conductor
        .call(
            &alice_zome,
            "test_send_ping",
            bob_cell_id.agent_pubkey().clone(),
        )
        .await;

    // Wait for Bob to receive the ping signal
    let bob_timeout = Duration::from_secs(30);
    tokio::time::timeout(bob_timeout, async {
        while !bob_received_ping.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .expect("Bob did not get a ping");

    // Wait for Alice to receive the pong signal
    let alice_timeout = Duration::from_secs(30);
    tokio::time::timeout(alice_timeout, async {
        while !alice_received_pong.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .expect("Alice did not get a pong");
}
