use dino_adventure::types::{AuthoredAdventure, NestBatchesWithNests};
use dino_adventure::types::{AuthoredNest, AuthoredNestBatch, CreateNestRequest};
use dino_adventure_integrity::Adventure;
use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;

#[tokio::test(flavor = "multi_thread")]
async fn create_nest_batch_nests_and_get() {
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let _: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure)
        .await;

    // Alice creates a nest batch
    let nest_batch: AuthoredNestBatch = alice_conductor
        .call(&alice_zome, "create_nest_batch", ())
        .await;

    // Alice creates several nests
    let mut nests = Vec::new();
    for _ in 0..3 {
        let request = CreateNestRequest {
            nest_batch_address: nest_batch.address.clone(),
            size: 50,
        };
        let nest: AuthoredNest = alice_conductor
            .call(&alice_zome, "create_nest", request)
            .await;

        nests.push(nest);
    }

    // Alice lists her nest batches
    let batches: NestBatchesWithNests = alice_conductor
        .call(
            &alice_zome,
            "get_nest_batches_with_nests_local",
            alice_agent.clone(),
        )
        .await;

    assert_eq!(1, batches.nest_batches.len());
    assert_eq!(1, batches.nests.len());

    let nest_batch_key = ActionHashB64::from(batches.nest_batches[0].address.clone());
    let nests_for_batch = batches.nests.get(&nest_batch_key).unwrap();
    assert_eq!(3, nests_for_batch.len());

    assert_eq!(nest_batch, batches.nest_batches[0]);

    let mut sorted_nests = nests.clone();
    sorted_nests.sort_by_key(|n| n.created_at);
    let mut sorted_retrieved_nests = nests_for_batch.clone();
    sorted_retrieved_nests.sort_by_key(|n| n.created_at);
    assert_eq!(sorted_nests, sorted_retrieved_nests);

    // Wait for dht sync
    await_consistency(&cells).await.unwrap();

    // Bob lists the same nest batches
    let batches_bob: NestBatchesWithNests = bob_conductor
        .call(
            &bob_zome,
            "get_nest_batches_with_nests_local",
            alice_agent.clone(),
        )
        .await;

    assert_eq!(batches, batches_bob);
}

#[tokio::test(flavor = "multi_thread")]
async fn check_batch_after_unlink_adventure() {
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let authored_adventure: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure)
        .await;

    // Alice creates a nest batch
    let nest_batch: AuthoredNestBatch = alice_conductor
        .call(&alice_zome, "create_nest_batch", ())
        .await;

    // Alice creates several nests
    for _ in 0..3 {
        let request = CreateNestRequest {
            nest_batch_address: nest_batch.address.clone(),
            size: 50,
        };
        let _: AuthoredNest = alice_conductor
            .call(&alice_zome, "create_nest", request)
            .await;
    }

    // Alice lists her nest batches
    let batches: NestBatchesWithNests = alice_conductor
        .call(
            &alice_zome,
            "get_nest_batches_with_nests_local",
            alice_agent.clone(),
        )
        .await;

    assert_eq!(1, batches.nest_batches.len());

    // Alice unlinks her adventure
    let _: () = alice_conductor
        .call(
            &alice_zome,
            "unlink_my_adventure",
            authored_adventure.address,
        )
        .await;

    // Alice lists her nest batches
    let batches_after_unlink: NestBatchesWithNests = alice_conductor
        .call(
            &alice_zome,
            "get_nest_batches_with_nests_local",
            alice_agent.clone(),
        )
        .await;

    assert_eq!(0, batches_after_unlink.nest_batches.len());
    assert_eq!(0, batches_after_unlink.nests.len());
}

#[tokio::test(flavor = "multi_thread")]
async fn get_nests_for_adventure() {
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("dino_adventure");
    let alice_agent = cells[0].agent_pubkey().clone();
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");
    let bob_agent = cells[1].agent_pubkey().clone();

    // Alice creates an adventure
    let adventure = Adventure {
        participants: vec![alice_agent.clone(), bob_agent.clone()],
    };
    let created_adventure: AuthoredAdventure = alice_conductor
        .call(&alice_zome, "create_adventure", adventure)
        .await;

    // Alice creates a nest batch
    let nest_batch: AuthoredNestBatch = alice_conductor
        .call(&alice_zome, "create_nest_batch", ())
        .await;

    // Alice creates several nests
    for _ in 0..3 {
        let request = CreateNestRequest {
            nest_batch_address: nest_batch.address.clone(),
            size: 50,
        };
        let _: AuthoredNest = alice_conductor
            .call(&alice_zome, "create_nest", request)
            .await;
    }

    // Wait for dht sync
    await_consistency(&cells).await.unwrap();

    // Alice gets the nest batches for her adventure
    let batches_alice: NestBatchesWithNests = alice_conductor
        .call(
            &alice_zome,
            "get_nest_batches_with_nests_by_adventure_hash_local",
            created_adventure.address.clone(),
        )
        .await;

    let total_nests: usize = batches_alice.nests.values().map(|v| v.len()).sum();
    assert_eq!(3, total_nests);

    // Bob gets the nest batches for Alice's adventure
    let batches_bob: NestBatchesWithNests = bob_conductor
        .call(
            &bob_zome,
            "get_nest_batches_with_nests_by_adventure_hash_local",
            created_adventure.address,
        )
        .await;

    assert_eq!(batches_alice, batches_bob);
}
