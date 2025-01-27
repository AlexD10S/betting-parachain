# Betting pallet

## Overview

This pallet implements a basic protocol for decentralized betting.

Every account can go an create a match and everyone can bet in that match, for a basic result: victory team 1, draw or victory team 2.
For now only the one with SUDO priviliges can set result.

:warning: It is **not a production-ready paller**, but a sample built for learning purposes. It is discouraged to use this code 'as-is' in a production runtime.

## Configuration

### Types
* `RuntimeEvent` – The overarching event type.
* `Currency` – The currency type.

### Constants
* `PalletId` – Pallet ID. Used for account derivation.
* `MaxTeamNameLength` – Maximum length for team names.
* `MaxBetsPerMatch` – Maximum number of bets per match.

## Extrinsics

<details>
<summary><h3>create_match_to_bet</h3></summary>

Creates a match to bet on. This function must be dispatched by a signed extrinsic.
Emit an event on success: `MatchCreated`.

#### Parameters:
* `origin` – Origin for the call. Must be signed.
* `team1` – Name of the first team.
* `team2` – Name of the second team.
* `start` – Time when the match starts and a bet can not be placed (in blocks).
* `lenght` – Duration of the match (in blocks).

#### Errors:
* `MatchAlreadyExists` – A match for the specified values already exists.
* `OriginHasAlreadyOpenMatch` – An origin can only have one match open.
* `TimeMatchOver` – The match is created when the match time is over.
</details>

<details>
<summary><h3>bet</h3></summary>

Create bet for a match.
Emit an event on success: `BetPlaced`.

#### Parameters:
* `origin` – Origin for the call. Must be signed.
* `match_id` – Id of the match, in our case the creator of the bet accountId.
* `amount_to_bet` – Amount placed for the bet.
* `result` – The result for the bet.

#### Errors:
* `MatchDoesNotExist` – A match selected for the bet doesn't exist.
* `OriginHasAlreadyOpenMatch` – If the match has started, betting is not allowed.
* `TimeMatchOver` – The match is created when the match time is over.
* `MaxBets` – The match has reach its betting limit.
* `AlreadyBet` – You already place the same bet in that match.
</details>

<details>
<summary><h3>set_result</h3></summary>

Notify the result of an existing match.
The dispatch origin for this call must be _Root_.
Emit an event on success: `MatchResult`.

#### Parameters:
* `origin` – Origin for the call. Must be signed.
* `match_id` – Id of the match, in our case the creator of the bet accountId.
* `result` – The result of the match.

#### Errors:
* `MatchDoesNotExist` – A match selected for the bet doesn't exist.
* `TimeMatchNotOver` –  If the match is not over, set the result is not allowed.
</details>

<details>
<summary><h3>distribute_winnings</h3></summary>

When a match ends someone the owner of the match can distribute the money from the winers and delete the match.

#### Parameters:
* `origin` – Origin for the call. Must be signed.

#### Errors:
* `MatchDoesNotExist` – A match selected for the bet doesn't exist.
* `MatchNotResult` –  The match still has not a result.
</details>

## RPC

<details>
<summary><h3>betting_getMatch</h3></summary>

Get a match stored.

#### Parameters:
* `match_id` – ID of the match to retrieve (accountId of the creator).
</details>

## How to add `pallet-betting` to a node

:information_source: The pallet is compatible with Substrate version
[polkadot-v0.9.32](https://github.com/paritytech/substrate/tree/polkadot-v0.9.32).

:information_source: This section is based on
[Substrate node template](https://github.com/substrate-developer-hub/substrate-node-template/tree/main).
Integrating `pallet-betting` with another node might look slightly different.

### Runtime's `Cargo.toml`

Add `pallet-betting`, and the RPC runtime API, to dependencies.
```toml

[dependencies.pallet-betting]
version = "0.0.3"
default-features = false
git = "https://github.com/AlexD10S/substrate-betting.git"
branch = "main"

[dependencies.pallet-betting-rpc-runtime-api]
version = "0.0.1"
default-features = false
git = "https://github.com/AlexD10S/substrate-betting.git"
branch = "main"
```

Update the runtime's `std` feature:
```toml
std = [
    # --snip--
    "pallet-betting/std",
    "pallet-betting-rpc-runtime-api/std",
    # --snip--
]
```

### Node's `Cargo.toml`

Add `pallet-betting-rpc` to dependencies.
```toml

[dependencies.pallet-betting-rpc]
version = "0.0.1"
default-features = false
git = "https://github.com/AlexD10S/substrate-betting.git"
branch = "main"
```

### Runtime's `lib.rs`


Configure the betting pallet.
```rust

parameter_types! {
    pub const BettingPalletId: PalletId = PalletId(*b"py/betts");
}

impl pallet_betting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = BettingPalletId;
    type Currency = Balances;
    type MaxBetsPerMatch = ConstU32<10>;
}
```

Add configured pallets to the `construct_runtime` macro call.
```rust
construct_runtime!(
    pub enum Runtime where
        // --snip--
    {
        // --snip---
        Betting: pallet_betting,
        // --snip---
    }
);
```

Add the RPC implementation.
```rust
impl_runtime_apis! {
    // --snip--
    impl pallet_betting_rpc_runtime_api::BettingApi<Block> for Runtime {
      fn get_matches() ->  pallet_betting::Matches {
        Betting::get_matches().unwrap_or({})
      }
    }
}
``` 


### Node's `rpc.rs`

Instantiate the RPC extension and merge it into the RPC module.
```rust
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    // --snip--
    C::Api: pallet_betting_rpc::BettingRuntimeApi<Block, Balance>,
{
    use pallet_betting_rpc::{Betting, BettingApiServer};
    // --snip--
    module.merge(Betting::new(client).into_rpc())?;
    Ok(module)
}
``` 
