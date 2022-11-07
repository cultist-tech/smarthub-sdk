use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;

use serde_json::json;
use mfight_sdk::nft::{ TokenRarity, TokenTypes, TOKEN_TYPE };
use std::collections::HashMap;

const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";

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
    token_type: &String
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
            "token_type": Some(token_type),
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    assert!(nft_outcome.is_success());

    Ok(nft_outcome)
}

#[tokio::test]
async fn test_burner() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let alice = worker.dev_create_account().await?;

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id_to_upgrade = "nft".to_string();
    let rarity_1 = 1;
    let token_type = "Armor".to_string();

    //Mint nft token to upgrade to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id_to_upgrade, &alice, &0, &token_type).await?;
    println!("Nft_mint NFT to Alice outcome: {:#?}", res);
    
    let token_id_to_burn1 = "nft1".to_string();
    let token_id_to_burn2 = "nft2".to_string();

    let res = mint_token_to_user(&worker, &nft_contract, &token_id_to_burn1, &alice, &0, &token_type).await?;
    println!("Nft_mint NFT1 to Alice outcome: {:#?}", res);

    let res = mint_token_to_user(&worker, &nft_contract, &token_id_to_burn2, &alice, &0, &token_type).await?;
    println!("Nft_mint NFT2 to Alice outcome: {:#?}", res);
 
    // Create types
    let mut token_types_map: TokenTypes = HashMap::new();
    token_types_map.insert(TOKEN_TYPE.to_string(), token_type.clone());        
    
    let rarity_sum = 2;

    //Set burner upgrade price
    let res = nft_contract
        .call(&worker, "nft_set_burner_upgrade_price")
        .args_json(
            json!({        
            "types": token_types_map,
            "rarity": rarity_1,            
            "burning_rarity_sum": rarity_sum,            
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_set_burner_upgrade_price price_rarity1 outcome: {:#?}", res);
    assert!(res.is_success());

    //View upgrade price
    let res: serde_json::Value = alice
        .call(&worker, nft_contract.id(), "nft_burner_upgrade_price")
        .args_json(json!({
            "token_id": token_id_to_upgrade,    
        }))?
        .view().await?
        .json()?;

    println!("Nft_burner_upgrade_price outcome: {:#?}", res);
    assert_eq!(res, rarity_sum);
    
    let burning_tokens = vec!(token_id_to_burn1.clone(), token_id_to_burn2.clone());

    //Upgrade nft
    let res = alice
        .call(&worker, nft_contract.id(), "nft_burner_upgrade")
        .args_json(json!({
            "token_id": token_id_to_upgrade,  
            "burning_tokens": burning_tokens
        }))?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_burning_upgrade outcome: {:#?}", res);
    assert!(res.is_success());

    //Check nft rarity is upgraded
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice nft nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id_to_upgrade);
    assert_eq!(res[0]["rarity"], rarity_1);
    
    //Check nft1 is burned
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_token")
        .args_json(json!({
            "token_id": token_id_to_burn1,
        }))?
        .view().await?
        .json()?;

    println!("Nft_token nft1 view outcome: {:#?}", res);
    assert_eq!(res, json!(None::<bool>));
    
    //Check nft2 is burned
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_token")
        .args_json(json!({
            "token_id": token_id_to_burn2,
        }))?
        .view().await?
        .json()?;

    println!("Nft_token nft2 view outcome: {:#?}", res);
    assert_eq!(res, json!(None::<bool>));

    let rarity_2 = 2;
    let rarity_sum = 3;

    //Set upgrade price for rarity 2
    let res = nft_contract
        .call(&worker, "nft_set_burner_upgrade_price")
        .args_json(
            json!({            
            "types": token_types_map,            
            "rarity": rarity_2,
            "burning_rarity_sum": rarity_sum,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_set_burner_upgrade_price outcome: {:#?}", res);
    assert!(res.is_success());

    //View upgrade price rarity 2
    let res: serde_json::Value = alice
        .call(&worker, nft_contract.id(), "nft_burner_upgrade_price")
        .args_json(json!({
            "token_id": token_id_to_upgrade,    
        }))?
        .view().await?
        .json()?;

    println!("Nft_upgrade_price outcome: {:#?}", res);
    assert_eq!(res, rarity_sum);
        
    //Remove burner upgrade price
    let res = nft_contract
        .call(&worker, "nft_remove_burner_upgrade_price")
        .args_json(
            json!({            
            "types": token_types_map,            
            "rarity": rarity_2,            
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_remove_burner_upgrade_price outcome: {:#?}", res);
    assert!(res.is_success());
        
    //View upgrade price is removed
    let res: serde_json::Value = alice
        .call(&worker, nft_contract.id(), "nft_burner_upgrade_price")
        .args_json(json!({
            "token_id": token_id_to_upgrade,    
        }))?
        .view().await?
        .json()?;

    println!("Nft_upgrade_price outcome: {:#?}", res);
    assert_eq!(res, json!(None::<bool>));    

    Ok(())
}
