use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;

use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::json_types::U128;
use schemars::JsonSchema;
use serde_json::json;
use mfight_sdk::nft::{ TokenRarity, UpdateOnFtTransferArgs };
use mfight_sdk::utils::near_ft;

const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";
const FT_WASM_FILEPATH: &str = "./out/ft/ft.wasm";

const TOTAL_SUPPLY: U128 = U128(1_000_000_000_000_000_000_000_000_000);
const ATTACHED_SUPPLY: u128 = 100_000_000_000_000_000_000_000;
const ONE_NEAR: u128 = 1000000000000000000000000;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Update(UpdateOnFtTransferArgs),
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
    user: &Account,
    rarity: &TokenRarity
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
            "rarity": rarity,            
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    assert!(nft_outcome.is_success());

    Ok(nft_outcome)
}

#[tokio::test]
async fn test_upgradable() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let alice = worker.dev_create_account().await?;
    let owner = worker.dev_create_account().await?;

    //Create FT contract
    let res = create_ft_contract(&worker, &ft_contract, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    //FT storage deposit for nft contract
    let res = storage_deposit_for_user(&worker, &ft_contract, &nft_contract.as_account()).await?;
    println!("Storage deposit FT for nft contract result: {:#?}", res);

    //FT storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    let price_in_ft = U128(1_000_000_000_000_000_000);

    //Send Alice some FT
    let res = owner
        .call(&worker, ft_contract.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": price_in_ft
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    let token_id = "nft".to_string();
    let rarity_1 = 1;

    //Mint nft token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id, &alice, &0).await?;
    println!("Nft_mint NFT to Alice outcome: {:#?}", res);

    let price_rarity1 = ONE_NEAR * 8;

    //Set upgrade price
    let res = nft_contract
        .call(&worker, "nft_set_upgrade_price")
        .args_json(
            json!({            
            "rarity": rarity_1,
            "ft_token_id": near_ft(),
            "price": U128(price_rarity1)
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_set_upgrade_price price_rarity1 outcome: {:#?}", res);
    assert!(res.is_success());

    //View upgrade price
    let res: serde_json::Value = alice
        .call(&worker, nft_contract.id(), "nft_upgrade_price")
        .args_json(json!({
            "token_id": token_id,    
        }))?
        .view().await?
        .json()?;

    println!("Nft_upgrade_price outcome: {:#?}", res);
    assert_eq!(res[0], json!(near_ft()));
    assert_eq!(res[1], json!(U128(price_rarity1)));

    //Upgrade nft
    let res = alice
        .call(&worker, nft_contract.id(), "nft_upgrade")
        .args_json(json!({
            "token_id": token_id,            
        }))?
        .deposit(price_rarity1)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_upgrade outcome: {:#?}", res);
    assert!(res.is_success());

    //Check nft rarity is upgraded
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice nft1 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);
    assert_eq!(res[0]["rarity"], rarity_1);

    let rarity_2 = 2;

    //Set upgrade price in FT
    let res = nft_contract
        .call(&worker, "nft_set_upgrade_price")
        .args_json(
            json!({            
            "rarity": rarity_2,
            "ft_token_id": ft_contract.id(),
            "price": price_in_ft
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_set_upgrade_price price_in_ft outcome: {:#?}", res);
    assert!(res.is_success());

    //View upgrade price
    let res: serde_json::Value = alice
        .call(&worker, nft_contract.id(), "nft_upgrade_price")
        .args_json(json!({
            "token_id": token_id,    
        }))?
        .view().await?
        .json()?;

    println!("Nft_upgrade_price outcome: {:#?}", res);
    assert_eq!(res[0], json!(ft_contract.id()));
    assert_eq!(res[1], json!(price_in_ft));

    //Make arg string to transfer FT to update nft
    let update_msg = serde_json
        ::to_string(
            &Args::Update(UpdateOnFtTransferArgs {
                token_id: token_id.clone(),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT amount to update nft
    let res = alice
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": nft_contract.id(),
            "amount": price_in_ft,
            "msg": update_msg
        })
        )?
        .deposit(1)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer to upgrade nft outcome: {:#?}", res);
    assert!(res.is_success());

    //Check nft rarity is upgraded
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice nft1 nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);
    assert_eq!(res[0]["rarity"], rarity_2);

    Ok(())
}