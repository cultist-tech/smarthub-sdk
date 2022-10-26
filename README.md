<div align="center">

  <h1><code>mfight_sdk</code></h1>

  <p>
    <strong>Rust library for writing NEAR smart contracts.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/mfight_sdk"><img src="https://img.shields.io/crates/v/mfight_sdk.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/mfight_sdk"><img src="https://img.shields.io/crates/d/mfight_sdk.svg?style=flat-square" alt="Download" /></a>
    <a href="https://buildkite.com/nearprotocol/mfight_sdk-rs"><img src="https://badge.buildkite.com/3bdfe06edbbfe67700833f865fe573b9ac6db517392bfc97dc.svg" alt="Buildkite Build" /></a>
  </p>
</div>

## Online Constructor

Online constructor to select the functionality you need

[https://near-constructor.vercel.app/](https://near-constructor.vercel.app/)

## The-Graph

The-graph will give you a ready-made api for reading data from the blockchain, forming analytics, etc. 

Online Demo:
[https://thegraph.com/hosted-service/subgraph/muzikanto/cultist](https://thegraph.com/hosted-service/subgraph/muzikanto/cultist)

SubGraph Repository: 
[cultist-tech/cultist-graph](https://github.com/cultist-tech/cultist-graph)

GQL queries Repository:
[cultist-tech/cultist-gql](https://github.com/cultist-tech/cultist-gql)

## NFT

NFT standard with new functionality (nep-171)

<b>NFT Upgrade</b>:                                                                
Upgrade an NFT by transferring a certain amount of FT or NEAR. Each NFT will have a pumping threshold, above which you can no longer pump. The price for each level is specified by the developers with the necessary functionality. This will allow the games to implement the functionality to improve any items, as well as increase the use of their FT.

<b>NFT Reveal</b>:                                                 
Immediately after mint users get NFT with the same media (hiding metadata fields) and there will be 2 options to open it:
1. The user himself can open his NFT, thanks to this mechanics it is possible to implement the mechanics of "boxes", like on Binance
2. The timing of disclosure is set by the creator, and, for example, disclosure occurs 24 hours after the full sale of the NFT collection, which allows you to expand the number of options for interacting with the community, thus heating up interest

<b>NFT Bind</b>:                                                                
Binding nft to an account (transfer ban)

Example Repository:
[cultist-tech/near-nft](https://github.com/cultist-tech/near-nft)

API:

- [x] core [nep-171](https://github.com/near/NEPs/blob/master/neps/nep-0171.md)
- [x] approval [nep-178](https://github.com/near/NEPs/blob/master/neps/nep-0178.md)
- [x] enumeration [nep-181](https://github.com/near/NEPs/blob/master/neps/nep-0181.md)
- [x] payout [nep-199](https://github.com/near/NEPs/blob/master/neps/nep-199.md)
- [x] mint
- [x] burn
- [x] bind_to_owner
- [x] royalty
- [x] reveal
- [ ] upgrade

## FT

FT standard (nep-141)

Example Repository:
[cultist-tech/near-ft](https://github.com/cultist-tech/near-ft)

- [x] core [nep-141](https://github.com/near/NEPs/blob/master/neps/nep-141.md)
- [x] storage_management [nep-145](https://github.com/near/NEPs/blob/master/neps/nep-145.md)

## MT

MT standard (nep-245)

Example Repository:
[cultist-tech/near-mt](https://github.com/cultist-tech/near-mt)

- [ ] core [nep-245](https://github.com/near/NEPs/blob/master/neps/nep-245.md)
- [ ] storage_management

## Market

Marketplace for nft trading

Example Repository:
[cultist-tech/near-market](https://github.com/cultist-tech/near-market)

- [x] core
- [x] enumeration
- [ ] storage_management

## Rent

It is possible to transfer an NFT for a certain period of time. At the end of the lease term, the NFT is returned to its holder. It is also possible to pay the rent for any FT or NEAR. 

Example Repository:
[cultist-tech/near-rent](https://github.com/cultist-tech/near-rent)

API:

- [x] core
- [x] enumeration
- [ ] storage_management

## NFT IDO

Implemented functionality for simplified initial sale of nft (like NFT sale on Binance). Basic functionality: random NFT from the list, purchase restrictions, time-delayed mint, mint for FT or NEAR. Problem: not many NFT platforms offer such functionality, that's why many developers create such functionality themselves.

Example Repository:
[cultist-tech/near-nft-ido](https://github.com/cultist-tech/near-nft-ido)

API:

- [x] core
- [x] enumeration
- [ ] storage_management

## Tournament

Necessary for various tournaments on blockchain. A tournament is created (access via NFT as an option) with certain conditions (number of participants, entry price, distribution of funds between winners). Each participant contributes a predetermined amount of money, after the end of the tournament, all collected funds are distributed according to the terms of the tournament. Also anyone can contribute FT or NFT to the tournament prize fund.
              
Example Repository:
[cultist-tech/near-tournament](https://github.com/cultist-tech/near-tournament)

API:

- [x] core
- [x] enumeration
- [x] nft_access

## NFT Fractionation

You collect certain NFTs to exchange them for another certain NFT from any collection. For example: a unique sword is created in the game, but it can only be obtained by collecting all its parts (blade, hilt). This would motivate users to search for the necessary NFT parts, offer to redeem them to the holders of the parts themselves. And the user who collected all the parts and exchanged them for the same unique item can get special opportunities. 

Example Repository:
[cultist-tech/near-fractionation](https://github.com/cultist-tech/near-fractionation)

API:

- [ ] core
- [ ] enumeration

## Escrow

The ability to trade NFT to FT, NFT to NFT, FT to FT outside of a specialized site with a specific user. This will give you the ability to do things with NFT/FT even if they are not on the site (not cast), or if you don't want the NFT/FT to fall into other hands.

Example Repository:
[cultist-tech/near-escrow](https://github.com/cultist-tech/near-escrow)

API:

- [x] core
- [x] enumeration

## Reputation

Reputation parameter for the participants of the project, will give certain bonuses for people with a good reputation. Reputation will be accrued from other users, by spending their daily accrued "votes" (the number of "votes" depends on the value of reputation), as well as for some activity in the project. Examples of uses: reduced commission on the marketplace, increased influence in the DAO, a pass to the tournament).

Example Repository:
[cultist-tech/near-reputation](https://github.com/cultist-tech/near-reputation)

- [ ] core

## Referral

Influencer advertises the project by providing a referral link. And in the referral contract each user who followed the link is recorded, as well as if necessary transferred to another contract (for example in the NFT contract to prescribe royalties)

Example Repository:
[cultist-tech/near-referral](https://github.com/cultist-tech/near-referral)

- [ ] core

## Tools

- [x] owner
- [x] contract pause
- [x] blacklist
- [x] whitelist


