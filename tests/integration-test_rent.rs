use workspaces::{ Worker, Account, Contract };
use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::result::CallExecutionDetails;
use near_sdk::ONE_YOCTO;
use near_sdk::AccountId;
use std::convert::TryFrom;
use std::collections::HashMap;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;
use near_sdk::json_types::U128;
use mfight_sdk::rent::{ RentOnNftApproveArgs, RentOnFtTransferArgs, SaleConditions };
use mfight_sdk::utils::near_ft;

const NFT_RENT_WASM_FILEPATH: &str = "./out/nft_rent/nft_rent.wasm";
const FT_WASM_FILEPATH: &str = "./out/ft/ft.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";

const PRICE: U128 = U128(1_000_000_000_000);
const ATTACHED_SUPPLY: u128 = 100_000_000_000_000_000_000_000;
const TOTAL_SUPPLY: U128 = U128(1_000_000_000_000_000_000_000_000_000);
const MIN_TIME: u64 = 3700000000000;
const MAX_TIME: u64 = 8540000000000000;

static DELIMETER: &str = "||";

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsRent {
    Rent(RentOnNftApproveArgs),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsFt {
    Rent(RentOnFtTransferArgs),
}

async fn create_rent_contract(
    worker: &Worker<Sandbox>,
    rent_contract: &Contract,
    owner: &Account
) -> anyhow::Result<CallExecutionDetails> {
    let rent_outcome = rent_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
        }))?
        .transact().await?;

    assert!(rent_outcome.is_success());
    Ok(rent_outcome)
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

async fn initialize_rent (
    worker: &Worker<Sandbox>,
    nft_rent_contract: &Contract,
    nft_contract: &Contract,
    sale_conditions: &SaleConditions,
    token: &String,
    owner: &Account,
    alice: &Account,
) -> anyhow::Result<()> {

    //Create rent contract
    let res = create_rent_contract(&worker, &nft_rent_contract, &owner).await?;
    println!("Rent contract create new_default_meta outcome: {:#?}", res);

    //Create NFT contract
    let res = create_nft_contract(&worker, &nft_contract).await?;
    println!("NFT create new_default_meta outcome: {:#?}", res);

    //Mint nft token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token, &alice).await?;
    println!("Nft_mint NFT1 to Alice outcome: {:#?}", res);

    //Make arg string to rent NFT1 to rent contract
    let nft_rent_msg = serde_json
        ::to_string(
            &ArgsRent::Rent(RentOnNftApproveArgs {
                sale_conditions: sale_conditions.clone(),
                min_time: MIN_TIME,
                max_time: MAX_TIME,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Approve NFT token to rent contract
    let res = alice
        .call(&worker, nft_contract.id(), "nft_approve")
        .args_json(
            json!({
            "token_id": token,
            "account_id": nft_rent_contract.id(),
            "msg": nft_rent_msg
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_approve NFT to rent contract outcome: {:#?}", res);
    assert!(res.is_success());

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_for_near() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(),PRICE);

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    let alice_balance_before = worker.view_account(alice.id()).await?.balance;

    let time = MIN_TIME + 10000;

    let price_for_time = ((PRICE.0 * (time/60000) as u128) / 60 / 1_000_000) as u128;

    //Pay rent for NFT
    let res = bob
        .call(&worker, nft_rent_contract.id(), "rent_pay")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
            "time": time,
            "receiver_id": bob.id(),
        })
        )?
        .deposit(price_for_time)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Rent pay outcome: {:#?}", res);
    assert!(res.is_success());

    //Check rent contract have nft1
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": nft_rent_contract.id(),
        }))?
        .view().await?
        .json()?;

    println!("Rent contract nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    let alice_balance_after = worker.view_account(alice.id()).await?.balance;

    assert_eq!(alice_balance_after, alice_balance_before + price_for_time);

    worker.fast_forward(15*60*60).await?;

    //Claim the rent
    let res = alice
        .call(&worker, nft_rent_contract.id(), "rent_claim")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Rent claim outcome: {:#?}", res);
    assert!(res.is_success());

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_for_ft() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    //Create FT contract
    let res = create_ft_contract(&worker, &ft_contract, &owner).await?;
    println!("FT create new_default_meta outcome: {:#?}", res);

    //FT storage deposit for rent contract
    let res = storage_deposit_for_user(&worker, &ft_contract, &nft_rent_contract.as_account()).await?;
    println!("Storage deposit FT for rent contract result: {:#?}", res);

    //FT storage deposit for Alice
    let res = storage_deposit_for_user(&worker, &ft_contract, &alice).await?;
    println!("Storage deposit FT for Alice result: {:#?}", res);

    //FT storage deposit for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract, &bob).await?;
    println!("Storage deposit FT for Bob result: {:#?}", res);

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(AccountId::try_from(ft_contract.id().clone()).unwrap(),PRICE);

    let time = MIN_TIME + 10000;

    let price_for_time = ((PRICE.0 * (time/60000) as u128) / 60 / 1_000_000) as u128;

    //Send Bob some FT
    let res = owner
        .call(&worker, ft_contract.id(), "ft_transfer")
        .args_json(
            json!({
            "receiver_id": bob.id(),
            "amount": U128(price_for_time),
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft transfer to Bob result: {:#?}", res);
    assert!(res.is_success());

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    //Make arg string to transfer FT to escrow to make an offer
    let rent_msg = serde_json
        ::to_string(
            &ArgsFt::Rent(RentOnFtTransferArgs {
                token_id: token_id.to_string(),
                contract_id: AccountId::try_from(nft_contract.id().clone()).unwrap(),
                receiver_id: AccountId::try_from(bob.id().clone()).unwrap(),
                time: time,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT amount to rent contract to pay for rent
    let res = bob
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": nft_rent_contract.id(),
            "amount": U128(price_for_time),
            "msg": rent_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer offer to escrow outcome: {:#?}", res);
    assert!(res.is_success());

    //Check rent contract have nft1
    let res: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": nft_rent_contract.id(),
        }))?
        .view().await?
        .json()?;

    println!("Rent contract nft_view outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    //Check Alice have FT
    let res: U128 = alice
        .call(&worker, ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice FT balance outcome: {:#?}", res);
    assert_eq!(u128::from(res), price_for_time);

    worker.fast_forward(15*60*60).await?;

    let res = alice
         .call(&worker, nft_rent_contract.id(), "rent_claim")
         .args_json(
             json!({
             "contract_id": nft_contract.id(),
             "token_id": token_id,
         })
         )?
         .gas(near_units::parse_gas!("300 T") as u64)
         .transact().await?;

    println!("Rent claim outcome: {:#?}", res);
    assert!(res.is_success());

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(),PRICE);

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    let new_min_time = MIN_TIME + 10000;
    let new_max_time = MAX_TIME - 10000;
    let new_price: U128 = U128(2_000_000_000_000_000_000_000_000);

    //Update rent contract
    let res = alice
        .call(&worker, nft_rent_contract.id(), "rent_update")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
            "ft_token_id": ft_contract.id(),
            "price_per_hour": new_price,
            "min_time": new_min_time,
            "max_time": new_max_time,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_approve NFT to rent contract outcome: {:#?}", res);
    assert!(res.is_success());

    //View rent
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rent")
         .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents for account outcome: {:#?}", res);
    assert_eq!(res["token_id"], token_id);
    assert_eq!(res["min_time"], new_min_time);
    assert_eq!(res["max_time"], new_max_time);
    assert_eq!(res["sale_conditions"][ft_contract.id().to_string()], json!(new_price));

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_remove() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(),PRICE);

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    //Remove rent contract
    let res = alice
        .call(&worker, nft_rent_contract.id(), "rent_remove")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Rent remove contract outcome: {:#?}", res);
    assert!(res.is_success());

    //View rents supply by contract
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_supply_by_contract")
         .args_json(
            json!({
            "contract_id": nft_contract.id(),
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents for contract outcome: {:#?}", res);
    assert_eq!(res, "0");

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_utils() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(),PRICE);

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    //Is rent approved
    let res: serde_json::Value = alice
        .call(&worker, nft_rent_contract.id(), "rent_is_approved")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
            "account_id": alice.id(),
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?.json()?;

    println!("Rent is approved contract outcome: {:#?}", res);
    assert_eq!(res, true);

    //Rent total supply
    let res: serde_json::Value = alice
        .call(&worker, nft_rent_contract.id(), "rent_total_supply")
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?.json()?;

    println!("Rent total supply contract outcome: {:#?}", res);
    assert_eq!(res, 1);

    let time = MIN_TIME + 10000;

    let price_for_time = ((PRICE.0 * (time/60000) as u128) / 60 / 1_000_000) as u128;

    //Pay rent for NFT
    let res = bob
        .call(&worker, nft_rent_contract.id(), "rent_pay")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
            "time": time,
            "receiver_id": bob.id(),
        })
        )?
        .deposit(price_for_time)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Rent pay outcome: {:#?}", res);
    assert!(res.is_success());

    //Is rent locked
    let res: serde_json::Value = alice
        .call(&worker, nft_rent_contract.id(), "rent_token_is_locked")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?.json()?;

    println!("Rent is locked contract outcome: {:#?}", res);
    assert_eq!(res, true);

    worker.fast_forward(15*60*60).await?;

    //Is rent ended
    let res: serde_json::Value = alice
        .call(&worker, nft_rent_contract.id(), "rent_is_ended")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?.json()?;

    println!("Rent is ended contract outcome: {:#?}", res);
    assert_eq!(res, true);

    Ok(())
}

#[tokio::test]
async fn test_nft_rent_enumeration() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let nft_rent_wasm = std::fs::read(NFT_RENT_WASM_FILEPATH)?;
    let nft_rent_contract = worker.dev_deploy(&nft_rent_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let owner = worker.dev_create_account().await?;

    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    let token_id = "nft1";

    let mut sale_conditions: SaleConditions = HashMap::new();
    sale_conditions.insert(near_ft(),PRICE);

    initialize_rent(&worker, &nft_rent_contract, &nft_contract, &sale_conditions, &token_id.to_string(), &owner, &alice).await?;

    let token_id2 = "nft2";

    //Mint nft2 token to Alice
    let res = mint_token_to_user(&worker, &nft_contract, &token_id2.to_string(), &alice).await?;
    println!("Nft_mint NFT2 to Alice outcome: {:#?}", res);

    //Make arg string to rent nft2 to rent contract
    let nft_rent_msg = serde_json
        ::to_string(
            &ArgsRent::Rent(RentOnNftApproveArgs {
                sale_conditions: sale_conditions.clone(),
                min_time: MIN_TIME,
                max_time: MAX_TIME,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Approve nft2 token to rent contract
    let res = alice
        .call(&worker, nft_contract.id(), "nft_approve")
        .args_json(
            json!({
            "token_id": token_id2,
            "account_id": nft_rent_contract.id(),
            "msg": nft_rent_msg
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_approve NFT to rent contract outcome: {:#?}", res);
    assert!(res.is_success());

    //View rents
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents")
         .args_json(
            json!({
            "from_index": U128(0),
            "limit": 100,
        })
        )?
        .view()
        .await?
        .json()?;

     println!("Rents enumeration outcome: {:#?}", res);
     assert_eq!(res[0]["token_id"], token_id);
     assert_eq!(res[1]["token_id"], token_id2);

    //View rents for account
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_for_account")
         .args_json(
            json!({
            "account_id": alice.id(),
            "from_index": U128(0),
            "limit": 100,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents for account outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);
    assert_eq!(res[1]["token_id"], token_id2);


    let token1 = format!("{}{}{}", nft_contract.id().to_string(), DELIMETER, token_id);
    let token2 = format!("{}{}{}", nft_contract.id().to_string(), DELIMETER, token_id2);
    let vec_t = vec![token1, token2];

    //View rents by id
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_by_ids")
         .args_json(
            json!({
            "ids": vec_t,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents by ids outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);
    assert_eq!(res[1]["token_id"], token_id2);

    //View rents supply for account
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_supply_for_account")
         .args_json(
            json!({
            "account_id": alice.id(),
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents supply for account outcome: {:#?}", res);
    assert_eq!(res, "2");

    //View rent
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rent")
         .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rent outcome: {:#?}", res);
    assert_eq!(res["token_id"], token_id);

    let time = MIN_TIME + 10000;

    let price_for_time = ((PRICE.0 * (time/60000) as u128) / 60 / 1_000_000) as u128;

    //Pay rent for NFT
    let res = bob
        .call(&worker, nft_rent_contract.id(), "rent_pay")
        .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "token_id": token_id,
            "time": time,
            "receiver_id": bob.id(),
        })
        )?
        .deposit(price_for_time)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Rent pay outcome: {:#?}", res);
    assert!(res.is_success());

    //View rented tokens for account
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rented_tokens_for_account")
         .args_json(
            json!({
            "account_id": bob.id(),
            "from_index": U128(0),
            "limit": 100,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rented tokens for account outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);

    //View rented tokens supply for account
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rented_tokens_supply_for_account")
         .args_json(
            json!({
            "account_id": bob.id(),
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rented tokens supply for account outcome: {:#?}", res);
    assert_eq!(res, "1");

    //View rents by contract
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_by_contract")
         .args_json(
            json!({
            "contract_id": nft_contract.id(),
            "from_index": U128(0),
            "limit": 100,
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents by contract outcome: {:#?}", res);
    assert_eq!(res[0]["token_id"], token_id);
    assert_eq!(res[1]["token_id"], token_id2);

    //View rents supply by contract
    let res: serde_json::Value = nft_rent_contract
        .call(&worker, "rents_supply_by_contract")
         .args_json(
            json!({
            "contract_id": nft_contract.id(),
        })
        )?
        .view()
        .await?
        .json()?;

    println!("Rents supply by contract outcome: {:#?}", res);
    assert_eq!(res, "2");

    Ok(())
}
