use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{env, log, near_bindgen, AccountId, CryptoHash};
use near_sdk::{init, BorshStorageKey, PanicOnDefault};

use crate::key::ContractKeys;
use vote::*;

near_sdk::setup_alloc!();

mod key;
mod vote;

type VoteId = CryptoHash;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct YesOrNoContract {
    vote: LookupMap<VoteId, Vote>,

    voter: LookupMap<AccountId, Voter>,
}

impl YesOrNoContract {
    #[init]
    pub fn new() -> Self {
        Self {
            vote: LookupMap::new(ContractKeys::ContractVote),
            voter: LookupMap::new(ContractKeys::ContractVoter),
        }
    }
}
