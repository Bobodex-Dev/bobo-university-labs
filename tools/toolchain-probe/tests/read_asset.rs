// Step 0 runtime probe: anchor-lang 1.2.1 (solana-v4) reading a real Metaplex
// Core asset, run in LiteSVM 0.17 against the dumped mainnet Core program.
use anchor_lang::InstructionData;
use litesvm::LiteSVM;
use mpl_core::instructions::{CreateCollectionV2Builder, CreateV2Builder};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

const CORE: &str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";

fn send(svm: &mut LiteSVM, ix: Instruction, signers: &[&Keypair]) -> litesvm::types::TransactionResult {
    let payer = signers[0].pubkey();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap();
    svm.send_transaction(tx)
}

fn setup() -> (LiteSVM, Keypair, Pubkey) {
    let mut svm = LiteSVM::new();
    let core: Pubkey = CORE.parse().unwrap();
    svm.add_program(core, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/mpl_core.so"))).unwrap();
    let probe = Pubkey::new_from_array(toolchain_probe::ID.to_bytes());
    svm.add_program(probe, include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/toolchain_probe.so"))).unwrap();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, payer, probe)
}

fn read_ix(probe: Pubkey, account: Pubkey) -> Instruction {
    Instruction {
        program_id: probe,
        accounts: vec![AccountMeta::new_readonly(account, false)],
        data: toolchain_probe::instruction::ReadAsset {}.data(),
    }
}

#[test]
fn reads_a_core_asset() {
    let (mut svm, payer, probe) = setup();
    let asset = Keypair::new();
    let create = CreateV2Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .name("Probe".into())
        .uri("https://example.com/p.json".into())
        .instruction();
    send(&mut svm, create, &[&payer, &asset]).expect("core create");

    let meta = send(&mut svm, read_ix(probe, asset.pubkey()), &[&payer]).expect("read");
    let want = format!("Program log: owner {}", payer.pubkey());
    assert!(meta.logs.iter().any(|l| l == &want), "logs: {:#?}", meta.logs);
    println!("read_asset compute units: {}", meta.compute_units_consumed);
}

#[test]
fn refuses_a_collection_by_its_first_byte() {
    let (mut svm, payer, probe) = setup();
    let collection = Keypair::new();
    let create = CreateCollectionV2Builder::new()
        .collection(collection.pubkey())
        .payer(payer.pubkey())
        .name("Probe Collection".into())
        .uri("https://example.com/c.json".into())
        .instruction();
    send(&mut svm, create, &[&payer, &collection]).expect("collection create");

    let err = send(&mut svm, read_ix(probe, collection.pubkey()), &[&payer]).unwrap_err();
    println!("collection: {:?}", err.err);
    assert!(err.meta.logs.iter().any(|l| l.contains("NotAnAsset") && l.contains("6000")), "logs: {:#?}", err.meta.logs);
}

#[test]
fn refuses_an_account_core_does_not_own() {
    let (mut svm, payer, probe) = setup();
    let err = send(&mut svm, read_ix(probe, payer.pubkey()), &[&payer]).unwrap_err();
    println!("not core: {:?}", err.err);
    assert!(err.meta.logs.iter().any(|l| l.contains("ConstraintOwner") && l.contains("2004")), "logs: {:#?}", err.meta.logs);
}

// A collection whose name and uri are chosen so its bytes also decode as an
// asset: name length 2 lands in the update-authority tag (2 = Collection), and
// the uri's last nine bytes decode as an empty name, an empty uri and no seq.
// Only the first-byte check tells it apart.
#[test]
fn refuses_a_crafted_collection_that_decodes_as_an_asset() {
    let (mut svm, payer, probe) = setup();
    let collection = Keypair::new();
    let uri = format!("{}{}", "x".repeat(23), "\0".repeat(9));
    let create = CreateCollectionV2Builder::new()
        .collection(collection.pubkey())
        .payer(payer.pubkey())
        .name("AB".into())
        .uri(uri)
        .instruction();
    send(&mut svm, create, &[&payer, &collection]).expect("crafted collection create");
    let data = svm.get_account(&collection.pubkey()).unwrap().data;
    let decoded = mpl_core::accounts::BaseAssetV1::from_bytes(&data);
    println!("host decode of the crafted collection: {:?}", decoded.as_ref().map(|a| (a.key, a.owner)));

    let res = send(&mut svm, read_ix(probe, collection.pubkey()), &[&payer]);
    println!("crafted: {:?}", res.as_ref().map(|m| m.logs.clone()).map_err(|e| e.err.clone()));
    let err = res.unwrap_err();
    assert!(err.meta.logs.iter().any(|l| l.contains("NotAnAsset") && l.contains("6000")), "logs: {:#?}", err.meta.logs);
}
