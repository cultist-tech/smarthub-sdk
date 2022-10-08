use workspaces::network::{ DevAccountDeployer, Sandbox };
use workspaces::{ Worker, Account, Contract };
use near_sdk::ONE_YOCTO;
use near_sdk::AccountId;
use std::convert::TryFrom;

use serde_json::json;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;
use near_sdk::json_types::U128;
use mfight_sdk::tournament::{ TournamentOnNftTransferArgs, TournamentOnFtTransferArgs };
use mfight_sdk::nft::{ TokenRarity, TokenCollection, TokenType };

const TOURNAMENT_WASM_FILEPATH: &str = "./out/tournament/tournament.wasm";
const NFT_WASM_FILEPATH: &str = "./out/nft/nft.wasm";
const FT_WASM_FILEPATH: &str = "./out/ft/ft.wasm";

const PRICE: &str = "10000000000000000000000";
const ATTACHED_SUPPLY: u128 = 100_000_000_000_000_000_000_000;
const TOTAL_SUPPLY: U128 = U128(1_000_000_000_000_000_000_000_000_000);

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Tournament(TournamentOnNftTransferArgs),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum ArgsFt {
    Tournament(TournamentOnFtTransferArgs),
}

async fn mint_token_to_user(
    worker: &Worker<Sandbox>,
    nft_contract: &Contract,
    token: &String,
    user: &Account
) -> anyhow::Result<()> {
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
            "rarity": TokenRarity::Common,
            "collection": TokenCollection::Fantasy,
            "token_type": TokenType::Avatar,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Nft_mint acces to Alice outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    Ok(())
}

async fn storage_deposit_for_user(
    worker: &Worker<Sandbox>,
    ft_contract: &Contract,
    user: &Account
) -> anyhow::Result<serde_json::Value> {
    let result_ft: serde_json::Value = user
        .call(&worker, ft_contract.id(), "storage_deposit")
        .args_json(json!({
            "account_id": user.id(),
        }))?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?
        .json()?;

    Ok(result_ft)
}

#[tokio::test]
async fn test_tournament() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let tournament_wasm = std::fs::read(TOURNAMENT_WASM_FILEPATH)?;
    let tournament_contract = worker.dev_deploy(&tournament_wasm).await?;

    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    //Create NFT contract
    let mut nft_outcome = nft_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": nft_contract.id(),
        }))?
        .transact().await?;

    println!("NFT create new_default_meta outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    let token_access_id = "access";
    let owner = worker.dev_create_account().await?;
    let alice = worker.dev_create_account().await?;

    //Mint access token to Alice
    mint_token_to_user(&worker, &nft_contract, &token_access_id.to_string(), &alice).await?;

    //Check Alice have access token
    let result: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice has access nft_view outcome: {:#?}", result);
    assert_eq!(result[0]["token_id"], token_access_id);

    let token_prize_nft_id = "prize";

    //Mint prize token to owner
    mint_token_to_user(&worker, &nft_contract, &token_prize_nft_id.to_string(), &owner).await?;

    //Create FT contract
    let mut ft_outcome = ft_contract
        .call(&worker, "new_default_meta")
        .args_json(
            json!({
            "owner_id": owner.id(),
            "total_supply": TOTAL_SUPPLY,
        })
        )?
        .transact().await?;

    println!("FT create new_default_meta outcome: {:#?}", ft_outcome);
    assert!(nft_outcome.is_success());

    //Create Tournament contract
    let mut tournament_outcome = tournament_contract
        .call(&worker, "new_with_default_meta")
        .args_json(json!({
            "owner_id": tournament_contract.id(),
        }))?
        .transact().await?;

    println!("Tournament contract create new_default_meta outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    let tournament_id = "Tournament1";

    //Create new tournament
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_create")
        .args_json(
            json!({
            "tournament_id": tournament_id,
            "players_number": 4,
            "price": PRICE,
            "name": "Tournament_name",
            "nft_access_contract": nft_contract.id(),
        })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_create by owner outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    let token_vec = vec![token_access_id];

    //Add NFT access to tournament
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_add_nft_access")
        .args_json(
            json!({
            "tournament_id": tournament_id,
            "token_ids": token_vec,
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_add_nft_access outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    let place_1: u8 = 1;

    //Make arg string to transfer NFT as a prize for tournament
    let tournament_nft_msg = serde_json
        ::to_string(
            &Args::Tournament(TournamentOnNftTransferArgs {
                tournament_id: tournament_id.to_string(),
                place: Some(place_1),
                owner_id: AccountId::try_from(owner.id().clone()).unwrap(),
                prize_id: Some(token_prize_nft_id.to_string()),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer minted NFT to tournament contract
    nft_outcome = owner
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": tournament_contract.id(),
            "token_id": token_prize_nft_id,
            "msg": tournament_nft_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("nft_transfer prize to tournnament outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    //Alice joins the tournament by NFT
    //Make arg string to transfer NFT to join the tournament
    let tournament_join_msg = serde_json
        ::to_string(
            &Args::Tournament(TournamentOnNftTransferArgs {
                tournament_id: tournament_id.to_string(),
                place: None,
                owner_id: AccountId::try_from(owner.id().clone()).unwrap(),
                prize_id: None,
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer access NFT to tournament contract
    nft_outcome = alice
        .call(&worker, nft_contract.id(), "nft_transfer_call")
        .args_json(
            json!({
            "receiver_id": tournament_contract.id(),
            "token_id": token_access_id,
            "msg": tournament_join_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Alice nft_tournament_join outcome: {:#?}", nft_outcome);
    assert!(nft_outcome.is_success());

    // Check tournament have access token
    let result: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": tournament_contract.id(),
        }))?
        .view().await?
        .json()?;

    println!("Owner recieved access nft_view outcome: {:#?}", result);
    assert_eq!(result[0]["token_id"], token_prize_nft_id);
    assert_eq!(result[1]["token_id"], token_access_id);

    let place_2: u8 = 2;
    let prize_2_amount = U128(10_000_000_000_000_000_000_000);
    let token_prize_ft_id = "FT_prize";

    // Storage deposit for FT prizes for tournament
    let res = storage_deposit_for_user(
        &worker,
        &ft_contract,
        tournament_contract.as_account()
    ).await?;
    println!("Storage deposit for tournament result: {:#?}", res);

    //Make arg string to transfer FT as a prize for tournament
    let tournament_ft_msg = serde_json
        ::to_string(
            &ArgsFt::Tournament(TournamentOnFtTransferArgs {
                tournament_id: tournament_id.to_string(),
                place: Some(place_2),
                owner_id: AccountId::try_from(owner.id().clone()).unwrap(),
                prize_id: Some(token_prize_ft_id.to_string()),
            })
        )
        .ok()
        .expect("Wrong struct to stringify");

    //Transfer FT amount as a prize to tournament contract
    ft_outcome = owner
        .call(&worker, ft_contract.id(), "ft_transfer_call")
        .args_json(
            json!({
            "receiver_id": tournament_contract.id(),
            "amount": prize_2_amount,
            "msg": tournament_ft_msg
        })
        )?
        .deposit(ONE_YOCTO)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Ft_transfer prize to tournnament outcome: {:#?}", ft_outcome);
    assert!(ft_outcome.is_success());

    let bob = worker.dev_create_account().await?;
    let charlie = worker.dev_create_account().await?;
    let danny = worker.dev_create_account().await?;

    //Bob join the tournament
    tournament_outcome = bob
        .call(&worker, tournament_contract.id(), "tournament_join")
        .args_json(
            json!({
                "tournament_id": tournament_id,
                "owner_id": owner.id(),
            })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_join bob outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    //Charlie join the tournament
    tournament_outcome = charlie
        .call(&worker, tournament_contract.id(), "tournament_join")
        .args_json(
            json!({
                "tournament_id": tournament_id,
                "owner_id": owner.id(),
            })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_join charlie outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    //Danny join the tournament
    tournament_outcome = danny
        .call(&worker, tournament_contract.id(), "tournament_join")
        .args_json(
            json!({
                "tournament_id": tournament_id,
                "owner_id": owner.id(),
            })
        )?
        .deposit(ATTACHED_SUPPLY)
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_join danny outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    //Start the tournament
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_start")
        .args_json(json!({
            "tournament_id": tournament_id,
        }))?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_start outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    let winner = alice.clone();
    let winner_place = place_1;

    //Reward first prize
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_execute_reward")
        .args_json(
            json!({
            "tournament_id": tournament_id,
            "winner_place": winner_place,
            "account_id": winner.id(),
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_execute_reward outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    //Check alice have prize token
    let result: serde_json::Value = nft_contract
        .call(&worker, "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": alice.id(),
        }))?
        .view().await?
        .json()?;

    println!("Alice prize nft_view outcome: {:#?}", result);
    assert_eq!(result[0]["token_id"], token_prize_nft_id);

    // Storage deposit for FT prizes for Bob
    let res = storage_deposit_for_user(&worker, &ft_contract, &bob).await?;
    println!("Storage deposit for Bob result: {:#?}", res);

    //Reward second prize
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_execute_reward")
        .args_json(
            json!({
            "tournament_id": tournament_id,
            "winner_place": place_2,
            "account_id": bob.id(),
        })
        )?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_execute_reward outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    //Check bob have ft prize
    let res: U128 = bob
        .call(&worker, ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))?
        .view().await?
        .json()?;

    println!("Bob prize ft_balance outcome: {:#?}", res);
    assert_eq!(res, prize_2_amount);

    //End the tournament
    tournament_outcome = owner
        .call(&worker, tournament_contract.id(), "tournament_end")
        .args_json(json!({
            "tournament_id": tournament_id,
        }))?
        .gas(near_units::parse_gas!("300 T") as u64)
        .transact().await?;

    println!("Tournament_end outcome: {:#?}", tournament_outcome);
    assert!(tournament_outcome.is_success());

    Ok(())
}