use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;
use near_sdk::ONE_YOCTO;
use near_sdk::AccountId;
use std::convert::TryFrom;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;
use near_sdk::json_types::U128;
use mfight_sdk::escrow::{ EscrowOnFtTransferArgs, EscrowOnNftTransferArgs };
use mfight_sdk::escrow::metadata::{ JsonEscrow };

const ESCROW_WASM_FILEPATH: &str = "./out/escrow/escrow.wasm";
const FT_WASM_FILEPATH: &str = "./out/ft/ft.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";

const PRICE: u128 = 10_000_000_000_000_000_000_000;
const ATTACHED_SUPPLY: u128 = 100_000_000_000_000_000_000_000;
const TOTAL_SUPPLY: U128 = U128(1_000_000_000_000_000_000_000_000_000);

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsFt {
    Escrow(EscrowOnFtTransferArgs),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsNft {
    Escrow(EscrowOnNftTransferArgs),
}

async fn create_escrow_contract(
    worker: &Worker<Sandbox>,
    escrow_contract: &Contract,
    owner: &Account
) -> anyhow::Result<()> {
    let escrow_outcome = escrow_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
        }))?
        .transact().await?;

    println!("Escrow create new_with_default_meta outcome: {:#?}", escrow_outcome);
    assert!(escrow_outcome.is_success());

    Ok(())
}

async fn create_ft_contract(
    worker: &Worker<Sandbox>,
    ft_contract: &Contract,
    owner: &Account
) -> anyhow::Result<CallExecutionDetails> {
    let ft_outcome = ft_contract
        .call(&worker, "new_default_meta")
        .args_json(
            json!({
            "owner_id": owner.id(),
            "total_supply": TOTAL_SUPPLY,
        })
        )?
        .transact().await?;

    assert!(ft_outcome.is_success());

    Ok(ft_outcome)
}

async fn create_nft_contract(
    worker: &Worker<Sandbox>,
    nft_contract: &Contract
) -> anyhow::Result<CallExecutionDetails> {
    let nft_outcome = nft_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": nft_contract.id(),
        }))?
        .transact().await?;

    assert!(nft_outcome.is_success());

    Ok(nft_outcome)
}

async fn storage_deposit_for_user(
    worker: &Worker<Sandbox>,
    ft_contract: &Contract,
    user: &Account
) -> anyhow::Result<serde_json::Value> {
    let ft_outcome: serde_json::Value = user
        .call(&worker, ft_contract.id(), "storage_deposit")
        .args_json(json!({
            "account_id": user.id(),
        }))?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?
        .json()?;

    Ok(ft_outcome)
}

async fn mint_token_to_user(
    worker: &Worker<Sandbox>,
    nft_contract: &Contract,
    token: &String,
    user: &Account
) -> anyhow::Result<CallExecutionDetails> {
    let nft_outcome = nft_contract
        .call(&worker, "nft_mint")
        .args_json(
            json!({
            "token_id": token,
            "receiver_id": user.id(),
            "token_metadata": {
                "title": "Olympus Mons",
                "dscription": "Tallest mountain in charted solar system",
                "copies": 1,
            },
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    assert!(nft_outcome.is_success());

    Ok(nft_outcome)
}

async fn remove_offer(
    worker: &Worker<Sandbox>,
    escrow_contract: &Contract,
    user: &Account,
    offer_id: &String
) -> anyhow::Result<CallExecutionDetails> {
    let escrow_outcome = user
        .call(&worker, escrow_contract.id(), "escrow_remove_offer")
        .args_json(json!({
            "offer_id": offer_id,
        }))?
        .deposit(PRICE)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    assert!(escrow_outcome.is_success());

    Ok(escrow_outcome)
}

#[tokio::test]
async fn test_ft_to_ft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract1 = worker.dev_deploy(&ft_wasm).await?;
    let ft_contract2 = worker.dev_deploy(&ft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create FT1 contract1
    let res = create_ft_contract(&worker, &ft_contract1, &owner).await?;
    println!("FT1 create new_default_meta outcome: {:#?}", res);

    //FT1 storage deposit for escrow
    let res = storage_deposit_for_user(
        &worker,
        &ft_contract1,
        &escrow_contract.as_account()
    ).await?;
    println!("Storage deposit FT1 for escrow result: {:#?}", res);

    //FT1 storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract1, &alice).await?;
    println!("Storage deposit FT1 for Alice result: {:#?}", res);

    //FT1 storage deposit for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract1, &bob).await?;
    println!("Storage deposit FT1 for Bob result: {:#?}", res);

    let ft_offer_by_alice = U128(1_000_000_000_000_000_000);

    //Send Alice some FT1
    let res = owner
        .call(&worker, ft_contract1.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": ft_offer_by_alice
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT1 transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    //Create FT2 contract2
    let res = create_ft_contract(&worker, &ft_contract2, &owner).await?;
    println!("FT2 create new_default_meta outcome: {:#?}", res);

    //FT2 storage deposit for escrow
    let res = storage_deposit_for_user(
        &worker,
        &ft_contract2,
        &escrow_contract.as_account()
    ).await?;
    println!("Storage deposit FT2 for escrow result: {:#?}", res);

    //FT2 storage deposit for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract2, &bob).await?;
    println!("Storage deposit FT2 for Bob result: {:#?}", res);

    //FT2 storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract2, &alice).await?;
    println!("Storage deposit FT2 for Alice result: {:#?}", res);

    let ft_amount_for_alice_by_bob = U128(10_000_000_000_000_000_000);

    //Send Bob some FT2
    let res = owner
        .call(&worker, ft_contract2.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": bob.id(),
            "amount": ft_amount_for_alice_by_bob
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT2 transfer to Bob result: {:#?}", res);
    assert!(res.is_success());

    //Make arg string to offer FT1 to escrow
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: Some(AccountId::try_from(ft_contract2.id().clone()).unwrap()),
                ft_amount_out: Some(ft_amount_for_alice_by_bob),
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT1 amount to escrow contract for offer
    let res = alice
        .call(&worker, ft_contract1.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //Make arg string to take an offer from escrow
    let escrow_ft_take_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: None,
                offer_id: Some(offer_id),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT2 amount to escrow contract to take an offer
    let res = bob
        .call(&worker, ft_contract2.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_amount_for_alice_by_bob,
            "msg": escrow_ft_take_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT_transfer take an offer escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice have FT2
    let res: U128 = alice
        .call(&worker, ft_contract2.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice FT2 balance outcome: {:#?}", res);
    assert_eq!(res, ft_amount_for_alice_by_bob);

    //Check Bob have FT1
    let res: U128 = bob
        .call(&worker, ft_contract1.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob FT1 balance outcome: {:#?}", res);
    assert_eq!(res, ft_offer_by_alice);

    Ok(())
}

#[tokio::test]
async fn test_nft_to_nft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract1 = worker.dev_deploy(&nft_wasm).await?;
    let nft_contract2 = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create NFT1 contract
    let res = create_nft_contract(&worker, &nft_contract1).await?;
    println!("NFT1 create new_default_meta outcome: {:#?}", res);

    let token_id1 = "nft1";

    //Mint nft1 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract1, &token_id1.to_string(), &alice).await?;
    println!("Nft_mint NFT1 to Alice outcome: {:#?}", res);

    //Create NFT2 contract
    let res = create_nft_contract(&worker, &nft_contract2).await?;
    println!("NFT2 create new_default_meta outcome: {:#?}", res);

    let token_id2 = "nft2";

    //Mint nft2 token to Bob
    let res = mint_token_to_user(&worker, &nft_contract2, &token_id2.to_string(), &bob).await?;
    println!("Nft_mint NFT2 to Bob outcome: {:#?}", res);

    //Make arg string to offer NFT1 to escrow
    let escrow_nft_offer_msg = serde_json
        ::to_string(
            &ArgsNft::Escrow(EscrowOnNftTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: Some(AccountId::try_from(nft_contract2.id().clone()).unwrap()),
                nft_token_id_out: Some(token_id2.to_string()),
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT1 token to escrow contract to make an offer
    let res = alice
        .call(&worker, nft_contract1.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id1,
            "msg": escrow_nft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT1 offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //Make arg string to take offer NFT1 from escrow
    let escrow_nft_take_offer_msg = serde_json
        ::to_string(
            &ArgsNft::Escrow(EscrowOnNftTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: None,
                offer_id: Some(offer_id),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT2 token to escrow contract to take an offer
    let res = bob
        .call(&worker, nft_contract2.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id2,
            "msg": escrow_nft_take_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT2 to take offer from escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice have nft2
    let res: serde_json::Value = nft_contract2
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice nft2 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id2);

    //Check Bob have nft1
    let res: serde_json::Value = nft_contract1
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob nft1 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id1);

    Ok(())
}

#[tokio::test]
async fn test_nft_to_ft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create FT contract
    let res = create_ft_contract(&worker, &ft_contract, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT storage deposit for escrow
    let res = storage_deposit_for_user(&worker, &ft_contract, &escrow_contract.as_account()).await?;

    println!("Storage deposit FT for escrow result: {:#?}", res);

    //FT storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    //FT storage deposit for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract, &bob).await?;
    println!("Storage deposit FT for Bob result: {:#?}", res);

    let ft_amount_for_alice_by_bob = U128(1_000_000_000_000_000_000);

    //Send Bob some FT
    let res = owner
        .call(&worker, ft_contract.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": bob.id(),
            "amount": ft_amount_for_alice_by_bob
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft transfer to Bob result: {:#?}", res);
    assert!(res.is_success());

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id = "nft1";

    //Mint nft1 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id.to_string(), &alice).await?;
    println!("Nft_mint NFT to Alice outcome: {:#?}", res);

    //Make arg string to offer NFT to escrow
    let escrow_nft_offer_msg = serde_json
        ::to_string(
            &ArgsNft::Escrow(EscrowOnNftTransferArgs {
                ft_contract_id_out: Some(AccountId::try_from(ft_contract.id().clone()).unwrap()),
                ft_amount_out: Some(ft_amount_for_alice_by_bob),
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT1 token to escrow contract to make an offer
    let res = alice
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id,
            "msg": escrow_nft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //Make arg string to take an offer from escrow
    let escrow_ft_take_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: None,
                offer_id: Some(offer_id),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT amount to escrow contract to take an offer
    let res = bob
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_amount_for_alice_by_bob,
            "msg": escrow_ft_take_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer take an offer escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice have FT
    let res: U128 = alice
        .call(&worker, ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice FT balance outcome: {:#?}", res);
    assert_eq!(res, ft_amount_for_alice_by_bob);

    //Check Bob have nft1
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob nft1 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    Ok(())
}

#[tokio::test]
async fn test_ft_to_nft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create FT contract
    let res = create_ft_contract(&worker, &ft_contract, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT storage deposit for escrow
    let res = storage_deposit_for_user(&worker, &ft_contract, &escrow_contract.as_account()).await?;
    println!("Storage deposit FT for escrow result: {:#?}", res);

    //FT storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    //FT storage deposit for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract, &bob).await?;
    println!("Storage deposit FT for Bob result: {:#?}", res);

    let ft_offer_by_alice = U128(1_000_000_000_000_000_000);

    //Send Alice some FT
    let res = owner
        .call(&worker, ft_contract.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": ft_offer_by_alice,
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id = "nft1";

    //Mint NFT token to Bob
    let res = mint_token_to_user(&worker, &nft_contract, &token_id.to_string(), &bob).await?;
    println!("Nft_mint NFT to Bob outcome: {:#?}", res);

    //Make arg string to transfer FT to escrow to make an offer
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: Some(AccountId::try_from(nft_contract.id().clone()).unwrap()),
                nft_token_id_out: Some(token_id.to_string()),
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT amount to escrow contract to make an offer
    let res = alice
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //Make arg string to take an offer from escrow
    let escrow_nft_take_offer_msg = serde_json
        ::to_string(
            &ArgsNft::Escrow(EscrowOnNftTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: None,
                offer_id: Some(offer_id),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT token to escrow contract to take an offer
    let res = bob
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id,
            "msg": escrow_nft_take_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT to take offer from escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice have nft1
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice nft1 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    //Check Bob have FT
    let res: U128 = bob
        .call(&worker, ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob FT balance outcome: {:#?}", res);
    assert_eq!(res, ft_offer_by_alice);

    Ok(())
}

#[tokio::test]
async fn test_remove_offer() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract1 = worker.dev_deploy(&ft_wasm).await?;
    let ft_contract2 = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract1 = worker.dev_deploy(&nft_wasm).await?;
    let nft_contract2 = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create FT1 contract
    let res = create_ft_contract(&worker, &ft_contract1, &owner).await?;

    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT1 storage deposit for escrow
    let res = storage_deposit_for_user(
        &worker,
        &ft_contract1,
        &escrow_contract.as_account()
    ).await?;

    println!("Storage deposit FT for escrow result: {:#?}", res);

    //FT1 storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract1, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    let balance_for_offers = U128(2_000_000_000_000_000_000);
    let ft_offer_by_alice = U128(1_000_000_000_000_000_000);
    let ft_amount_for_alice_by_bob = U128(1_000_000_000_000_000_000);

    //Send Alice some FT1
    let res = owner
        .call(&worker, ft_contract1.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": balance_for_offers,
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    //Make arg string to transfer FT1 to escrow to make an offer
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: Some(AccountId::try_from(ft_contract2.id().clone()).unwrap()),
                ft_amount_out: Some(ft_amount_for_alice_by_bob),
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT1 amount to escrow contract to make an FT to FT offer
    let res = alice
        .call(&worker, ft_contract1.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let token_id2 = "nft2";

    //Make arg string to make FT to NFT offer to escrow
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: Some(AccountId::try_from(nft_contract2.id().clone()).unwrap()),
                nft_token_id_out: Some(token_id2.to_string()),
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT1 amount to escrow contract to make FT to NFT offer
    let res = alice
        .call(&worker, ft_contract1.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //Remove FT to FT offer from escrow contract
    let res = remove_offer(&worker, &escrow_contract, &alice, &offer_id).await?;
    println!("Escrow remove ft to ft offer outcome: {:#?}", res);

    let offer_id2 = format!("{}-{}", alice.id().clone(), 2);

    //Remove FT to NFT offer from escrow contract
    let res = remove_offer(&worker, &escrow_contract, &alice, &offer_id2).await?;
    println!("Escrow remove offer ft for nft outcome: {:#?}", res);

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract1).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id1 = "nft1";

    //Mint nft1 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract1, &token_id1.to_string(), &alice).await?;
    println!("Nft_mint NFT to Alice outcome: {:#?}", res);

    //Make arg string to transfer NFT1 to escrow to make NFT to NFT offer
    let escrow_nft_offer_msg = serde_json
        ::to_string(
            &ArgsNft::Escrow(EscrowOnNftTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: Some(AccountId::try_from(nft_contract2.id().clone()).unwrap()),
                nft_token_id_out: Some(token_id2.to_string()),
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT1 token to escrow contract to make NFT to NFT offer
    let res = alice
        .call(&worker, nft_contract1.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id1,
            "msg": escrow_nft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT1 for NFT to NFT offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Remove NFT to NFT offer from escrow contract
    let res = remove_offer(&worker, &escrow_contract, &alice, &offer_id).await?;
    println!("Escrow remove offer nft for nft outcome: {:#?}", res);

    //Make arg string to offer NFT to escrow to make NFT to FT offer
    let escrow_nft_for_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: Some(AccountId::try_from(ft_contract2.id().clone()).unwrap()),
                ft_amount_out: Some(ft_amount_for_alice_by_bob),
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer NFT1 token to escrow contract to make NFT to FT offer
    let res = alice
        .call(&worker, nft_contract1.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "token_id": token_id1,
            "msg": escrow_nft_for_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer NFT1 offer NFT to FT to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Remove NFT to FT offer from escrow contract
    let res = remove_offer(&worker, &escrow_contract, &alice, &offer_id).await?;
    println!("Escrow remove offer nft for ft outcome: {:#?}", res);

    Ok(())
}

#[tokio::test]
async fn test_enumeration_for_offers() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let escrow_wasm = std::fs::read(ESCROW_WASM_FILEPATH)?;
    let escrow_contract = worker.dev_deploy(&escrow_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract1 = worker.dev_deploy(&ft_wasm).await?;
    let ft_contract2 = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create Escrow contract
    create_escrow_contract(&worker, &escrow_contract, &owner).await?;

    //Create FT1 contract
    let res = create_ft_contract(&worker, &ft_contract1, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT1 storage deposit for escrow
    let res = storage_deposit_for_user(
        &worker,
        &ft_contract1,
        &escrow_contract.as_account()
    ).await?;
    println!("Storage deposit FT for escrow result: {:#?}", res);

    //FT1 storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract1, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    let balance_for_offers = U128(2_000_000_000_000_000_000);
    let ft_offer_by_alice = U128(1_000_000_000_000_000_000);
    let ft_amount_for_alice_by_bob = U128(1_000_000_000_000_000_000);

    //Send Alice some FT1
    let res = owner
        .call(&worker, ft_contract1.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": balance_for_offers,
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    //Make arg string to transfer FT1 to escrow to make an an FT to FT offer
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: Some(AccountId::try_from(ft_contract2.id().clone()).unwrap()),
                ft_amount_out: Some(ft_amount_for_alice_by_bob),
                nft_contract_id_out: None,
                nft_token_id_out: None,
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT1 amount to escrow contract to make an FT to FT offer
    let res = alice
        .call(&worker, ft_contract1.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let token_id2 = "nft2";

    //Make arg string to make FT to NFT offer to escrow
    let escrow_ft_offer_msg = serde_json
        ::to_string(
            &ArgsFt::Escrow(EscrowOnFtTransferArgs {
                ft_contract_id_out: None,
                ft_amount_out: None,
                nft_contract_id_out: Some(AccountId::try_from(nft_contract.id().clone()).unwrap()),
                nft_token_id_out: Some(token_id2.to_string()),
                receiver_id: Some(AccountId::try_from(bob.id().clone()).unwrap()),
                offer_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT1 amount to escrow contract to make FT to NFT offer
    let res = alice
        .call(&worker, ft_contract1.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": escrow_contract.id(),
            "amount": ft_offer_by_alice,
            "msg": escrow_ft_offer_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    let offer_id = format!("{}-{}", alice.id().clone(), 1);

    //View offer_id
    let res: std::option::Option<JsonEscrow> = alice
        .call(&worker, escrow_contract.id(), "escrow_offer")
        .args_json(json!({
            "offer_id": offer_id ,
        }))?
        .view().await?
        .json()?;

    println!("Alice escrow_offers_by_owner outcome: {:#?}", res);
    assert_eq!(res.unwrap().offer_id , offer_id);

    //View  Alice offers
    let res: serde_json::Value = alice
        .call(&worker, escrow_contract.id(), "escrow_offers_by_owner")
        .args_json(
            json!({
            "account_id": AccountId::try_from(alice.id().clone()).unwrap(),
        })
        )?
        .view().await?
        .json()?;

    println!("Alice escrow_offers_by_owner outcome: {:#?}", res);
    assert_eq!(res[0]["data"]["FtToFt"]["amount_in"] , json!(ft_offer_by_alice));

    //View  Alice offers total
    let res: u64 = alice
        .call(&worker, escrow_contract.id(), "escrow_offers_total_by_owner")
        .args_json(
            json!({
            "account_id": AccountId::try_from(alice.id().clone()).unwrap(),
        })
        )?
        .view().await?
        .json()?;

    println!("Alice escrow_offers_total_by_owner outcome: {:#?}", res);
    assert_eq!(res , 2);

    //View Bob offers
    let res: serde_json::Value = alice
        .call(&worker, escrow_contract.id(), "escrow_offers_for_owner")
        .args_json(
            json!({
            "account_id": AccountId::try_from(bob.id().clone()).unwrap(),
        })
        )?
        .view().await?
        .json()?;

    println!("Bob escrow_offers_for_owner outcome: {:#?}", res);
    assert_eq!(res[0]["data"]["FtToFt"]["amount_in"] , json!(ft_offer_by_alice));

    //View Bob offers total
    let res: u64 = bob
        .call(&worker, escrow_contract.id(), "escrow_offers_total_for_owner")
        .args_json(
            json!({
            "account_id": AccountId::try_from(bob.id().clone()).unwrap(),
        })
        )?
        .view().await?
        .json()?;

    println!("Bob escrow_offers_total_for_owner outcome: {:#?}", res);
    assert_eq!(res , 2);

    Ok(())
}
