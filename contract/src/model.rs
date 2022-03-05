use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{AccountId, Timestamp};

use crate::VoteId;

enum Choose {
    YES,
    NO,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Vote {
    id: VoteId,

    initiator: AccountId,

    title: String,

    desc: Option<String>,

    /// link of additional description or introduction
    link: Option<String>,

    /// `vote` status
    active: bool,

    /// If `finish's` length greater than `threshold`,then finish this vote,it be Completed.
    threshold: u32,

    /// set of finisher's accountId
    finish: UnorderedSet<(AccountId, Choose, Timestamp)>,

    /// set of accountId who not vote yet
    thinking: UnorderedSet<AccountId>,

    create_time: Timestamp,

    finish_time: Option<Timestamp>,
}

struct Voter {
    thinking: UnorderedSet<VoteId>,

    finish: UnorderedSet<(VoteId, Choose, Timestamp)>,
}
