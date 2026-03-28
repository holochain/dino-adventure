use dino_adventure::types::AuthoredAdventure;
use dino_adventure_integrity::Adventure;
use holochain::sweettest::*;
use holochain_client::AgentPubKey;
use holochain_trace::test_run;
use std::path::Path;

#[tokio::test(flavor = "multi_thread")]
async fn create_adventure() {
    test_run();

    // Create conductors with the standard config
    let mut alice_conductor = SweetConductor::from_standard_config().await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let app = alice_conductor
        .setup_app("test-app", &[dna_file])
        .await
        .unwrap();
    let cells = app.cells();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure1 = Adventure {
        participants: vec![
            alice_agent.clone(),
            AgentPubKey::from_raw_32([0; 32].into()),
        ],
    };
    let record1: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure1.clone())
        .await;
    assert_eq!(record1.adventure, adventure1);
}

#[tokio::test(flavor = "multi_thread")]
async fn create_adventure_and_get_all_my_adventures() {
    test_run();

    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    conductors.exchange_peer_info().await;
    await_consistency_s(120, &cells).await.unwrap();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure1 = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let record1: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure1)
        .await;

    // Bob creates an adventure
    let adventure2 = Adventure {
        participants: vec![bob_agent.clone(), alice_agent.clone()],
    };
    let record2: AuthoredAdventure = bob_conductor
        .call(&bob_zome, "create_adventure", adventure2)
        .await;

    // Wait for the dht sync
    await_consistency_s(120, &cells).await.unwrap();

    // Alice checks her adventures
    let mut create_read_output1: Vec<AuthoredAdventure> = alice_conductor
        .call(&alice_zome, "get_all_my_adventures_local", ())
        .await;
    create_read_output1.sort_by_key(|a| a.created_at);
    assert_eq!(vec![record1], create_read_output1);

    // Bob checks his adventures
    let mut create_read_output2: Vec<AuthoredAdventure> = bob_conductor
        .call(&bob_zome, "get_all_my_adventures_local", ())
        .await;
    create_read_output2.sort_by_key(|a| a.created_at);
    assert_eq!(vec![record2], create_read_output2);
}

#[tokio::test(flavor = "multi_thread")]
async fn end_my_adventure() {
    test_run();

    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    conductors.exchange_peer_info().await;
    await_consistency_s(120, &cells).await.unwrap();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure1 = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let record1: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure1)
        .await;

    // Alice creates a second adventure
    let adventure2 = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let record2: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure2)
        .await;

    // Bob creates an adventure
    let adventure3 = Adventure {
        participants: vec![bob_agent.clone(), alice_agent.clone()],
    };
    let record3: AuthoredAdventure = bob_conductor
        .call(&bob_zome, "create_adventure", adventure3)
        .await;

    // Wait for the dht sync
    await_consistency_s(120, &cells).await.unwrap();

    // Alice checks her adventures
    let mut create_read_output1: Vec<AuthoredAdventure> = alice_conductor
        .call(&alice_zome, "get_all_my_adventures_local", ())
        .await;
    create_read_output1.sort_by_key(|a| a.created_at);

    let mut expected = vec![record1.clone(), record2.clone()];
    expected.sort_by_key(|a| a.created_at);
    assert_eq!(expected, create_read_output1);

    // Bob checks his adventures
    let mut create_read_output2: Vec<AuthoredAdventure> = bob_conductor
        .call(&bob_zome, "get_all_my_adventures_local", ())
        .await;
    create_read_output2.sort_by_key(|a| a.created_at);
    assert_eq!(vec![record3.clone()], create_read_output2);

    // Alice tries to unlink bob's adventure (should fail)
    let result: Result<(), _> = alice_conductor
        .call_fallible(&alice_zome, "unlink_my_adventure", record3.address.clone())
        .await;
    assert!(result.is_err());

    // Alice unlinks her latest adventure
    let _: () = alice_conductor
        .call(&alice_zome, "unlink_my_adventure", record2.address)
        .await;

    // Alice checks her adventures
    let mut create_read_output3: Vec<AuthoredAdventure> = alice_conductor
        .call(&alice_zome, "get_all_my_adventures_local", ())
        .await;
    create_read_output3.sort_by_key(|a| a.created_at);
    assert_eq!(vec![record1], create_read_output3);

    // Bob unlinks his adventure
    let _: () = bob_conductor
        .call(&bob_zome, "unlink_my_adventure", record3.address)
        .await;

    // Bob checks his adventures
    let create_read_output4: Vec<AuthoredAdventure> = bob_conductor
        .call(&bob_zome, "get_all_my_adventures_local", ())
        .await;
    assert_eq!(Vec::<AuthoredAdventure>::new(), create_read_output4);
}

#[tokio::test(flavor = "multi_thread")]
async fn create_adventure_and_get_all_adventures_local() {
    test_run();

    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    conductors.exchange_peer_info().await;
    await_consistency_s(120, &cells).await.unwrap();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure1 = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let _: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure1)
        .await;

    // Bob creates an adventure
    let adventure2 = Adventure {
        participants: vec![bob_agent.clone(), alice_agent.clone()],
    };
    let _: AuthoredAdventure = bob_conductor
        .call(&bob_zome, "create_adventure", adventure2)
        .await;

    // Wait for dht sync
    await_consistency_s(120, &cells).await.unwrap();

    // Alice gets all adventures
    let mut all_adventures1: Vec<AuthoredAdventure> = alice_conductor
        .call(&alice_zome, "get_all_adventures_local", ())
        .await;
    assert_eq!(2, all_adventures1.len());
    all_adventures1.sort_by_key(|a| a.created_at);

    // Bob gets all adventures
    let mut all_adventures2: Vec<AuthoredAdventure> = bob_conductor
        .call(&bob_zome, "get_all_adventures_local", ())
        .await;
    assert_eq!(2, all_adventures2.len());
    all_adventures2.sort_by_key(|a| a.created_at);

    assert_eq!(all_adventures1, all_adventures2);
}
