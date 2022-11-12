use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::json_types::U128;
use schemars::JsonSchema;
use mfight_sdk::market::{ MarketOnNftApproveArgs, SaleConditions };
use mfight_sdk::utils::near_ft;
use mfight_sdk::reputation::{ SALE_INCREMENT, BUY_INCREMENT };
use std::collections::HashMap;

const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";
const MARKET_WASM_FILEPATH: &str = "./out/market/market.wasm";

const PRICE: u128 = 10_000_000_000_000_000_000_000;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Market(MarketOnNftApproveArgs),
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

async fn create_market_contract(
    worker: &Worker<Sandbox>,
    market_contract: &Contract
) -> anyhow::Result<CallExecutionDetails> {
    let market_outcome = market_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": market_contract.id(),
        }))?
        .transact().await?;

    assert!(market_outcome.is_success());

    Ok(market_outcome)
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

#[tokio::test]
async fn test_reputation() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let market_wasm = std::fs::read(MARKET_WASM_FILEPATH)?;
    let market_contract = worker.dev_deploy(&market_wasm).await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    //Create Market contract
    let res = create_market_contract(&worker, &market_contract).await?;
    println!("Market create new_default_meta outcome: {:#?}", res);

    let token_id = "nft".to_string();

    //Mint nft token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id, &alice).await?;
    println!("Nft_mint NFT to Alice outcome: {:#?}", res);

    //Check reputation before sale
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice reputation outcome: {:#?}", res);
    assert_eq!(res, 0);

    // Price for sale
    let price = U128(PRICE);

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(), price);

    //Make arg string to approve NFT to market
    let approve_msg = serde_json
        ::to_string(
            &Args::Market(MarketOnNftApproveArgs {
                sale_conditions: sale_conditions,
                is_auction: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    // Approve nft to market
    let res = alice
        .call(&worker, nft_contract.id(), "nft_approve")
        .args_json(
            json!({        
            "token_id": token_id,
            "account_id": market_contract.id(),            
            "msg": approve_msg,            
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_approve NFT outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Bobs reputation before sale
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob reputation before sale outcome: {:#?}", res);
    assert_eq!(res, 0);

    let fee = (PRICE * 300u128) / 10_000u128;

    let price_and_fee = PRICE + fee;

    //Buy nft from market
    let res = bob
        .call(&worker, market_contract.id(), "market_offer")
        .args_json(
            json!({        
            "nft_contract_id": nft_contract.id(),
            "token_id": token_id,                           
        })
        )?
        .deposit(price_and_fee)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Market_offer NFT outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice reputation after sale
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice reputation after sale outcome: {:#?}", res);
    assert_eq!(res, SALE_INCREMENT);

    //Check Bob reputation after sale
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob reputation after sale outcome: {:#?}", res);
    assert_eq!(res, BUY_INCREMENT);

    let shared_amount = 4;

    //Alice shares reputation with bob
    let res = alice
        .call(&worker, market_contract.id(), "share_reputation_with")
        .args_json(
            json!({
            "account_id": bob.id(),
            "amount": shared_amount,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Alice share reputation outcome: {:#?}", res);
    assert!(res.is_success());

    //Check Alice reputation after share
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice reputation after share outcome: {:#?}", res);
    assert_eq!(res, SALE_INCREMENT - shared_amount);

    //Check Bob reputation after share
    let res: serde_json::Value = market_contract
        .call(&worker, "reputation")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob reputation after share outcome: {:#?}", res);
    assert_eq!(res, BUY_INCREMENT + shared_amount);

    Ok(())
}