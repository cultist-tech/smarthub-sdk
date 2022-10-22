use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;

use serde_json::json;
use near_sdk::json_types::U128;
use mfight_sdk::nft::TokenRarity;

const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";

const ONE_NEAR: u128 = 1000000000000000000000000;

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

async fn mint_token_to_user(
    worker: &Worker<Sandbox>,
    nft_contract: &Contract,
    token: &String,
    user: &Account,
    rarity: &TokenRarity,   
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
    
    let alice = worker.dev_create_account().await?;
  
    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

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
            "price": U128(price_rarity1)
        })
        )?        
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_set_upgrade_price price_rarity1 outcome: {:#?}", res);
    assert!(res.is_success());
    
    //View upgrade price
    let res: U128 = alice
        .call(&worker, nft_contract.id(), "nft_upgrade_price")
        .args_json(json!({
            "token_id": token_id,    
        }))?
         .view().await?
         .json()?;

    println!("Nft_upgrade_price outcome: {:#?}", res);
    assert_eq!(res, U128(price_rarity1));
    
    //Upgrade nft
    let res = alice
        .call(&worker, nft_contract.id(), "nft_upgrade")
        .args_json(
            json!({
            "token_id": token_id,            
        })
        )?
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

    Ok(())
}
