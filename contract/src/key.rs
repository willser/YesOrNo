use crate::ItemId;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum ContractKeys {
    ContractVote,
    ContractVoter,
    Review,
}
#[derive(BorshStorageKey, BorshSerialize)]
pub enum VoteKeys {
    // VoteFinish(VoteId),
    // VoteThinking(VoteId),
    VoterFinish(AccountId),
    VoterThinking(AccountId),
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum ItemKeys {
    Active(ItemId),
    Negative(ItemId),
}
