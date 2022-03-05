use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::BorshStorageKey;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum ContractKeys {
    ContractVote,
    ContractVoter,
}
#[derive(BorshStorageKey, BorshSerialize)]
pub enum VoteKeys {
    VoteFinish,
    VoteThinking,
    VoterFinish,
    VoterThinking,
}
