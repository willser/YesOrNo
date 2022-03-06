use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{init, PanicOnDefault};
use near_sdk::{near_bindgen, AccountId, CryptoHash};

use vote::*;

use crate::key::ContractKeys;

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
