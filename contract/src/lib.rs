use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId, CryptoHash};

use model::*;

mod model;

near_sdk::setup_alloc!();

type VoteId = CryptoHash;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    vote: LookupMap<VoteId, Vote>,

    voter: LookupMap<AccountId, Voter>,
}

impl Contract {
    #[init]
    pub fn new() -> Self {
        Self
    }
}
