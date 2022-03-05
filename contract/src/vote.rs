use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::env::panic;
use near_sdk::{env, AccountId, Timestamp};
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::ops::Add;

use crate::key::VoteKeys::*;
use crate::{VoteId, YesOrNoContract};

pub enum Choose {
    YES,
    NO,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Vote {
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
    finish: UnorderedMap<AccountId, (Choose, Timestamp)>,

    /// set of accountId who not vote yet
    thinking: UnorderedSet<AccountId>,

    create_time: Timestamp,

    finish_time: Option<Timestamp>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InputVote {
    title: String,

    desc: Option<String>,

    /// link of additional description or introduction
    link: Option<String>,

    /// `vote` status
    active: bool,

    /// If `finish's` length greater than `threshold`,then finish this vote,it be Completed.
    threshold: u32,

    thinking: HashSet<AccountId>,
}

pub struct Voter {
    pub thinking: UnorderedSet<VoteId>,

    pub finish: UnorderedSet<(VoteId, Choose, Timestamp)>,
}

/// convert InputVote to Vote
fn convert(input: InputVote, id: VoteId) -> Vote {
    Vote {
        id,
        initiator: string,
        title,
        desc: input.desc,
        link: input.link,
        active: false,
        threshold: input.threshold,
        finish: UnorderedMap::new(VoteFinish),
        thinking: UnorderedSet::new(VoteThinking),
        create_time: env::block_timestamp(),
        finish_time: None,
    }
}

/// Get vote id from inputVote
fn get_vote_id(input: &InputVote) -> VoteId {
    let initiator = env::current_account_id();
    let title = &input.title;

    let desc = match &input.desc {
        None => "",
        Some(desc) => desc,
    };

    let link = match &input.link {
        None => "",
        Some(link) => link,
    };

    let vec = env::sha256(&initiator + title + desc + link);
    vec.as_slice().try_into().expect("error about get voteId")
}

impl YesOrNoContract {
    pub fn create_vote(&mut self, input_vote: InputVote) {
        let id = get_vote_id(&vote);

        let vote_map = self.vote.borrow_mut();
        if vote_map.contains_key(&id) {
            panic!("exist same vote");
        }
        if vote_map.thinking.len() as u32 >= vote_map.threshold {
            panic!("illegal threshold or participant number");
        }

        let vote = convert(input_vote, id);

        vote_map.insert(&id, &vote);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, CryptoHash, MockedBlockchain, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn my_test() {
        let context = get_context(false);
        testing_env!(context);

        let sha2561 = env::sha256(b"asdasda");
        // let cow = String::from_utf8(sha2561).unwrap();

        println!("{:?}", sha2561);
    }
}
