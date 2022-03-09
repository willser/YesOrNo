use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{init, PanicOnDefault};
use near_sdk::{near_bindgen, AccountId};

use crate::key::ContractKeys;
use crate::review::Item;
use crate::types::{Choose, ItemId, VoteId};
use crate::vote::{Vote, Voter};

mod key;
mod review;
mod types;
mod vote;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct YesOrNoContract {
    vote: LookupMap<VoteId, Vote>,

    voter: LookupMap<AccountId, Voter>,

    review: UnorderedMap<ItemId, Item>,
}

impl YesOrNoContract {
    #[init]
    pub fn new() -> Self {
        Self {
            vote: LookupMap::new(ContractKeys::ContractVote),
            voter: LookupMap::new(ContractKeys::ContractVoter),
            review: UnorderedMap::new(ContractKeys::Review),
        }
    }
}
