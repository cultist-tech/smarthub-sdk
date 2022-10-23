use workspaces::network::DevAccountDeployer;
use near_sdk::ONE_YOCTO;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;
use mfight_sdk::nft_ido::{ NftIdoOnNftTransferArgs};
use mfight_sdk::nft::{TokenRarity, TokenCollection, TokenType};

const NFT_IDO_WASM_FILEPATH: &str = "./out/nft_ido/nft_ido.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";
const VALID_DATE: u64 = 1663513608939000001;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    NftIdo(NftIdoOnNftTransferArgs),
}

#[tokio::test]
async fn test_nft_ido() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let ido_wasm = std::fs::read(NFT_IDO_WASM_FILEPATH)?;
    let ido_contract = worker.dev_deploy(&ido_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    //Create nft contract
    let mut nft_outcome = nft_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": nft_contract.id(),
        }))?
        .transact().await?;

    println!("NFT new_default_meta outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    let token_id = "0";

    //Mint a new token
    nft_outcome = nft_contract
        .call(&worker,  "nft_mint")
        .args_json(
            json!({
            "token_id": token_id,
            "receiver_id": nft_contract.id(),
            "token_metadata": {
                "title": "Olympus Mons",
                "dscription": "Tallest mountain in charted solar system",
                "copies": 1,
            },
            "rarity": 0,
            "collection": TokenCollection::Fantasy,
            "token_type": TokenType::Avatar,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_mint outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    //Create an IDO contract
    let mut ido_outcome = ido_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": ido_contract.id(),
        }))?
        .transact().await?;

    println!("IDO new_default_meta outcome: {:#?}", ido_outcome);
    assert!(ido_outcome.is_success());

    let deposit = 100_000_000_000_000_000_000_000;
    let ido_id = "ido0";

    //Create new IDO
    ido_outcome = nft_contract.as_account()
        .call(&worker, ido_contract.id(), "nft_ido_add")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "ido_id": ido_id,
            "name": "ido0_name",
            "amount": 1,
            "price": "100000000000000000000",
            "per_transaction_min": 1,
            "per_transaction_max": 1,
            "buy_max": 1
        })
        )?
        .deposit(deposit)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_ido_add outcome: {:#?}", ido_outcome);
    assert!(ido_outcome.is_success());

    let ido_msg = serde_json
        ::to_string(&Args::NftIdo(NftIdoOnNftTransferArgs { ido_id: ido_id.to_string() }))
        .ok()
        .expect("Wrong struct to stringify");

    // Transfer minted NFT to IDO contract
    nft_outcome = nft_contract
        .call(&worker, "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": ido_contract.id(),
            "token_id": token_id,
            "msg": ido_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("nft_transfer outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    // Start IDO sale
    ido_outcome = nft_contract.as_account()
        .call(&worker, ido_contract.id(), "nft_ido_start")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "ido_id": ido_id,
            "date": VALID_DATE
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("nft_ido_start outcome: {:#?}", ido_outcome);
    assert!(ido_outcome.is_success());

    let alice = worker.dev_create_account().await?;

    // Buy the sale by Alice
    ido_outcome = alice
        .call(&worker, ido_contract.id(), "nft_ido_buy")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "receiver_id": alice.id(),
            "ido_id": ido_id,
            "amount": 1,
        })
        )?
        .deposit(deposit)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_ido_buy outcome: {:#?}", ido_outcome);
    assert!(ido_outcome.is_success());

    // Check Alice bought token from IDO
    let result: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("nft_view outcome: {:#?}", result);
    assert_eq!(result[0]["token_id"], token_id);

    Ok(())
}
