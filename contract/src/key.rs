use crate::VoteId;
use near_primitives_core::serialize;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey, CryptoHash};

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
