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
        initiator: env::current_account_id(),
        title: input.title,
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
    let title = input.title.as_str();

    let desc = match &input.desc {
        None => "",
        Some(desc) => desc,
    };

    let link = match &input.link {
        None => "",
        Some(link) => link,
    };

    let vec = env::sha256((initiator + title + desc + link).as_bytes());
    vec.as_slice().try_into().expect("error about get voteId")
}

impl YesOrNoContract {
    pub fn create_vote(&mut self, input_vote: InputVote) -> VoteId {
        let id = get_vote_id(&input_vote);

        // todo check those code gas cost.
        // need to check participant's accountId is legal or not.
        for x in input_vote.thinking {
            assert!(env::is_valid_account_id(x.as_bytes()))
        }

        let vote_map = self.vote.borrow_mut();
        if vote_map.contains_key(&id) {
            panic!("exist same vote");
        }
        if (input_vote.thinking.len() as u32) < input_vote.threshold {
            panic!("illegal threshold or participant number");
        }

        let vote = convert(input_vote, id);

        vote_map.insert(&id, &vote);
        id
    }

    pub fn get_vote(&self, vote_id: VoteId) -> Option<Vote> {
        (&self.vote).get(&vote_id)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, utils, CryptoHash, MockedBlockchain, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(accounts(1).try_into().unwrap())
            .current_account_id(accounts(1).try_into().unwrap())
            .is_view(is_view)
            .build()
    }
    const TEST_TITLE: &str = "test";

    #[test]
    fn test_crate_vote() {
        let mut contract = create_vm_and_get_contract();

        let input_vote = get_vote();
        let vote_id = contract.create_vote(input_vote);

        let option = contract.get_vote(vote_id);

        let vote = option.unwrap();
        assert_eq!(vote.title, TEST_TITLE);
        let alice: AccountId = accounts(1).try_into().unwrap();
        assert_eq!(vote.initiator, alice);
    }

    #[test]
    #[should_panic(expected = "exist same vote")]
    fn test_crate_vote_repeat() {
        let mut contract = create_vm_and_get_contract();
        let input_vote = get_vote();
        contract.create_vote(input_vote);

        let input_vote = get_vote();
        contract.create_vote(input_vote);
    }

    fn create_vm_and_get_contract() -> YesOrNoContract {
        let context = get_context(false);
        testing_env!(context);

        YesOrNoContract::new()
    }

    fn get_vote() -> InputVote {
        InputVote {
            title: TEST_TITLE.to_string(),
            desc: None,
            link: None,
            active: false,
            threshold: 10,
            thinking: (0..20).map(|temp| -> String { temp.to_string() }).collect(),
        }
    }
}
