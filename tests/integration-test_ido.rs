use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;
use near_sdk::ONE_YOCTO;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use std::convert::TryFrom;
use schemars::JsonSchema;
use mfight_sdk::nft_ido::{ NftIdoOnNftTransferArgs, NftIdoOnFtTransferArgs };

const NFT_IDO_WASM_FILEPATH: &str = "./out/nft_ido/nft_ido.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";
const FT_WASM_FILEPATH: &str = "./out/ft/ft.wasm";

const VALID_DATE: u64 = 1663513608939000001;
const PRICE: u128 = 10_000_000_000_000_000_000_000;
const ATTACHED_SUPPLY: u128 = 100_000_000_000_000_000_000_000;
const TOTAL_SUPPLY: U128 = U128(1_000_000_000_000_000_000_000_000_000);

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsNft {
    NftIdo(NftIdoOnNftTransferArgs),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsFt {
    NftIdo(NftIdoOnFtTransferArgs),
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

async fn mint_token(
    worker: &Worker<Sandbox>,
    nft_contract: &Contract,
    token: &String
) -> anyhow::Result<CallExecutionDetails> {
    let nft_outcome = nft_contract
        .call(&worker, "nft_mint")
        .args_json(
            json!({
            "token_id": token,            
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
async fn test_nft_ido_near() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let ido_wasm = std::fs::read(NFT_IDO_WASM_FILEPATH)?;
    let ido_contract = worker.dev_deploy(&ido_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let alice = worker.dev_create_account().await?;

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id = "nft".to_string();

    //Mint nft token
    let res = mint_token(&worker, &nft_contract, &token_id).await?;
    println!("Nft_mint NFT outcome: {:#?}", res);

    //Create an IDO contract
    let res = ido_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": ido_contract.id(),
        }))?
        .transact().await?;

    println!("IDO new_default_meta outcome: {:#?}", res);
    assert!(res.is_success());

    let ido_id = "ido0";
    let ido_name = "ido0_name";

    //Create new IDO
    let res = nft_contract
        .as_account()
        .call(&worker, ido_contract.id(), "nft_ido_add")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "ido_id": ido_id,
            "name": ido_name,
            "amount": 1,
            "price": U128(PRICE),
            "per_transaction_min": 1,
            "per_transaction_max": 1,
            "buy_max": 1
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_ido_add outcome: {:#?}", res);
    assert!(res.is_success());

    let ido_msg = serde_json
        ::to_string(&ArgsNft::NftIdo(NftIdoOnNftTransferArgs { ido_id: ido_id.to_string() }))
        .ok()
        .expect("Wrong struct to stringify");

    // Transfer minted NFT to IDO contract
    let res = nft_contract
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

    println!("Nft_transfer outcome: {:#?}", res);
    assert!(res.is_success());

    // Start IDO sale
    let res = nft_contract
        .as_account()
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

    println!("Nft_ido_start outcome: {:#?}", res);
    assert!(res.is_success());

    // Buy the sale by Alice
    let res = alice
        .call(&worker, ido_contract.id(), "nft_ido_buy")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "receiver_id": alice.id(),
            "ido_id": ido_id,
            "amount": 1,
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_ido_buy outcome: {:#?}", res);
    assert!(res.is_success());

    // Check Alice bought token from IDO
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    Ok(())
}

#[tokio::test]
async fn test_nft_ido_ft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let ido_wasm = std::fs::read(NFT_IDO_WASM_FILEPATH)?;
    let ido_contract = worker.dev_deploy(&ido_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let owner = worker.dev_create_account().await?;
    let alice = worker.dev_create_account().await?;

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    let token_id = "nft".to_string();

    //Mint nft token
    let res = mint_token(&worker, &nft_contract, &token_id).await?;
    println!("Nft_mint NFT outcome: {:#?}", res);

    //Create an IDO contract
    let res = ido_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": ido_contract.id(),
        }))?
        .transact().await?;

    println!("IDO new_default_meta outcome: {:#?}", res);
    assert!(res.is_success());

    let ido_id = "ido0".to_string();
    let ido_name = "ido0_name";

    //Create new IDO
    let res = nft_contract
        .as_account()
        .call(&worker, ido_contract.id(), "nft_ido_add")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "ido_id": ido_id,
            "name": ido_name,
            "amount": 1,
            "price": U128(PRICE),
            "per_transaction_min": 1,
            "per_transaction_max": 1,
            "buy_max": 1,
            "ft_token": ft_contract.id()
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_ido_add outcome: {:#?}", res);
    assert!(res.is_success());

    let ido_msg = serde_json
        ::to_string(
            &ArgsNft::NftIdo(NftIdoOnNftTransferArgs {
                ido_id: ido_id.clone(),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    // Transfer minted NFT to IDO contract
    let res = nft_contract
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

    println!("Nft_transfer outcome: {:#?}", res);
    assert!(res.is_success());

    // Start IDO sale
    let res = nft_contract
        .as_account()
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

    println!("Nft_ido_start outcome: {:#?}", res);
    assert!(res.is_success());

    //Create FT contract
    let res = create_ft_contract(&worker, &ft_contract, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT storage deposit for ido contract
    let res = storage_deposit_for_user(&worker, &ft_contract, &ido_contract.as_account()).await?;
    println!("Storage deposit FT for ido contract result: {:#?}", res);

    //FT storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    //FT storage deposit for nft owner
    let res = storage_deposit_for_user(&worker, &ft_contract, &nft_contract.as_account()).await?;
    println!("Storage deposit FT for nft owner result: {:#?}", res);

    //Send Alice some FT
    let res = owner
        .call(&worker, ft_contract.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": alice.id(),
            "amount": U128(PRICE)
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT transfer to Alice result: {:#?}", res);
    assert!(res.is_success());

    //Make arg string to buy ido
    let ido_msg = serde_json
        ::to_string(
            &ArgsFt::NftIdo(NftIdoOnFtTransferArgs {
                contract_id: AccountId::try_from(nft_contract.id().clone()).unwrap(),
                ido_id: ido_id,
                receiver_id: AccountId::try_from(alice.id().clone()).unwrap(),
                mint_amount: 1,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    // Buy the sale by Alice for FT
    let res = alice
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": ido_contract.id(),
            "amount": U128(PRICE),
            "msg": ido_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("FT_transfer by Alice to ido contract outcome: {:#?}", res);
    assert!(res.is_success());

    // Check Alice bought token from IDO
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    //Check NFT owner have FT
    let res: U128 = nft_contract
        .as_account()
        .call(&worker, ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": nft_contract.id(),
        }))?
        .view().await?
        .json()?;

    println!("NFT owner FT balance outcome: {:#?}", res);
    assert_eq!(res, U128(PRICE));

    Ok(())
}