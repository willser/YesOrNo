use crate::VoteId;
use near_primitives_core::types::AccountId;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::BorshStorageKey;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum ContractKeys {
    ContractVote,
    ContractVoter,
}
#[derive(BorshStorageKey, BorshSerialize)]
pub enum VoteKeys {
    VoteFinish(VoteId),
    VoteThinking(VoteId),
    VoterFinish(AccountId),
    VoterThinking(AccountId),
}
