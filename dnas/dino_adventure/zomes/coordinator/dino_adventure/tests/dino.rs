use dino_adventure::types::AuthoredDino;
use dino_adventure_integrity::{Dino, DinoKind};
use holochain::sweettest::*;
use holochain_trace::test_run;
use std::path::Path;

#[tokio::test(flavor = "multi_thread")]
async fn create_dino() {
    test_run();

    // Create a conductor with the standard config
    let mut conductor = SweetConductor::from_standard_config().await;
    let dna_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/dino_adventure.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let app = conductor.setup_app("test-app", &[dna_file]).await.unwrap();
    let zome = app.cells()[0].zome("dino_adventure");

    // Agent creates a Dino
    let dino = Dino {
        name: "Dean O".to_string(),
        dino_kind: DinoKind::Apatosaurus,
    };
    let authored_dino: AuthoredDino = conductor.call(&zome, "create_dino", dino.clone()).await;
    assert_eq!(authored_dino.dino, dino);
}

#[tokio::test(flavor = "multi_thread")]
async fn create_and_get_all_dinos() {
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
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("dino_adventure");

    let sample1 = Dino {
        name: "dino 1".to_string(),
        dino_kind: DinoKind::Apatosaurus,
    };
    let sample2 = Dino {
        name: "dino 2".to_string(),
        dino_kind: DinoKind::Apatosaurus,
    };

    // Alice creates a Dino
    let record1: AuthoredDino = alice_conductor
        .call(&alice_zome, "create_dino", sample1.clone())
        .await;

    // Bob creates a Dino
    let record2: AuthoredDino = bob_conductor
        .call(&bob_zome, "create_dino", sample2.clone())
        .await;

    // Wait for dht sync
    await_consistency_s(120, &cells).await.unwrap();

    // Bob gets the created Dinos
    let mut create_read_output: Vec<AuthoredDino> =
        bob_conductor.call(&bob_zome, "get_all_dinos", ()).await;
    create_read_output.sort_by(|a, b| a.dino.name.cmp(&b.dino.name));

    let mut expected = vec![record1.clone(), record2.clone()];
    expected.sort_by(|a, b| a.dino.name.cmp(&b.dino.name));
    assert_eq!(expected, create_read_output);

    // Bob gets the created Dinos locally
    let mut create_read_output_local: Vec<AuthoredDino> = bob_conductor
        .call(&bob_zome, "get_all_dinos_local", ())
        .await;
    create_read_output_local.sort_by(|a, b| a.dino.name.cmp(&b.dino.name));
    assert_eq!(create_read_output, create_read_output_local);

    // Alice gets the created Dinos
    let mut create_read_output2: Vec<AuthoredDino> =
        alice_conductor.call(&alice_zome, "get_all_dinos", ()).await;
    create_read_output2.sort_by(|a, b| a.dino.name.cmp(&b.dino.name));
    assert_eq!(expected, create_read_output2);

    // Alice gets the created Dinos locally
    let mut create_read_output2_local: Vec<AuthoredDino> = alice_conductor
        .call(&alice_zome, "get_all_dinos_local", ())
        .await;
    create_read_output2_local.sort_by(|a, b| a.dino.name.cmp(&b.dino.name));
    assert_eq!(create_read_output2, create_read_output2_local);
}
