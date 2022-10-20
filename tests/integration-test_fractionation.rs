use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;
use near_sdk::ONE_YOCTO;

use serde_json::json;
use near_sdk::json_types::U128;
use mfight_sdk::nft_fractionation::{ Fractionation, FractionationNftOnTransferArgs };

const FRACTIONATION_WASM_FILEPATH: &str = "./out/nft_fractionation/nft_fractionation.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";

async fn create_fractionation_contract(
    worker: &Worker<Sandbox>,
    fractionation_contract: &Contract,
    owner: &Account,
    nft_contract: &Contract
) -> anyhow::Result<()> {
    let fractionation_outcome = fractionation_contract
        .call(&worker, "new_with_default_meta")
        .args_json(
            json!({
            "owner_id": owner.id(),
            "nft_contract_id": nft_contract.id()
        })
        )?
        .transact().await?;

    println!("Fractionation create new_with_default_meta outcome: {:#?}", fractionation_outcome);
    assert!(fractionation_outcome.is_success());

    Ok(())
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
async fn test_fractionation() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let fractionation_wasm = std::fs::read(FRACTIONATION_WASM_FILEPATH)?;
    let fractionation_contract = worker.dev_deploy(&fractionation_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;
    let alice = worker.dev_create_account().await?;

    //Create Fractionation contract
    create_fractionation_contract(&worker, &fractionation_contract, &owner, &nft_contract).await?;

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id1 = "nft1".to_string();

    //Mint nft1 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id1, &alice).await?;
    println!("Nft_mint NFT1 to Alice outcome: {:#?}", res);

    let token_id2 = "nft2".to_string();

    //Mint nft2 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id2, &alice).await?;
    println!("Nft_mint NFT2 to Alice outcome: {:#?}", res);

    let token_id_super = "nft_super".to_string();

    //Mint nft_super token to Owner
    let res = mint_token_to_user(&worker, &nft_contract, &token_id_super, &owner).await?;
    println!("Nft_mint NFT_SUPER to Owner outcome: {:#?}", res);

    let vec_t = vec![token_id1.clone(), token_id2.clone()];

    //Make arg string to fractionate NFT
    let fractionation_msg = serde_json
        ::to_string(
            &(FractionationNftOnTransferArgs {
                fractionation_tokens: Some(vec_t),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer token to fractionation contract to create fractionation
    let res = owner
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": fractionation_contract.id(),
            "token_id": token_id_super,
            "msg": fractionation_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer to fractionate token outcome: {:#?}", res);
    assert!(res.is_success());

    //View nft fractionation
    let res: Fractionation = fractionation_contract
        .call(&worker, "nft_fractionation")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id_super,
         })
        )?
        .view().await?
        .json()?;

    println!("Token fractionation outcome: {:#?}", res);
    assert_eq!(res.token_id, token_id_super);
    assert_eq!(res.entries[0], token_id1);
    assert_eq!(res.entries[1], token_id2);

    //View fractionations  by contract
    let res: serde_json::Value = fractionation_contract
        .call(&worker, "nft_fractionations")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "from_index": U128(0),
            "limit": 100,
         })
        )?
        .view().await?
        .json()?;

    println!("Fractionations by contract outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id_super);
    assert_eq!(res[0]["entries"][0], token_id1);
    assert_eq!(res[0]["entries"][1], token_id2);

    //View fractionations supply by contract
    let res: serde_json::Value = fractionation_contract
        .call(&worker, "nft_fractionations_supply")
        .args_json(json!({
            "contract_id": nft_contract.id(),
        }))?
        .view().await?
        .json()?;

    println!("Fractionations supply by contract outcome: {:#?}", res);
    assert_eq!(res, "1");
   
    //Make arg string to transfer NFT to complete fractionation
    let fractionation_to_complete_msg = serde_json
        ::to_string(
            &(FractionationNftOnTransferArgs {
                fractionation_tokens: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer nft1 token to fractionation contract to complete fractionation
    let res = alice
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": fractionation_contract.id(),
            "token_id": token_id1,
            "msg": fractionation_to_complete_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer token to complete fractionation  outcome: {:#?}", res);
    assert!(res.is_success());

    //Transfer nft2 token to fractionation contract to complete fractionation
    let res = alice
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": fractionation_contract.id(),
            "token_id": token_id2,
            "msg": fractionation_to_complete_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_transfer token to complete fractionation  outcome: {:#?}", res);
    assert!(res.is_success());

    let res = alice
        .call(&worker, fractionation_contract.id(), "nft_fractionation_complete")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id_super
        })
        )?        
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Complete fractionation outcome: {:#?}", res);
    assert!(res.is_success());

    Ok(())
}
