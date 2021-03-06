use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId, Timestamp};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};

use crate::key::VoteKeys::*;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
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
    threshold: u64,

    /// count of yes
    count: u64,

    /// map of finisher's accountId
    finish: HashMap<AccountId, (Choose, Timestamp)>,

    /// set of accountId who not vote yet
    thinking: HashSet<AccountId>,

    create_time: Timestamp,

    finish_time: Option<Timestamp>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub struct InputVote {
    title: String,

    desc: Option<String>,

    /// link of additional description or introduction
    link: Option<String>,

    /// If `finish's` length greater than `threshold`,then finish this vote,it be Completed.
    threshold: u64,

    thinking: HashSet<AccountId>,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct ActiveVote {
    vote_id: VoteId,
    title: String,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct FinishVote {
    vote_id: VoteId,
    title: String,
    choose: Choose,
    finish_time: Timestamp,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voter {
    pub thinking: UnorderedSet<(VoteId, String)>,

    pub finish: UnorderedSet<(VoteId, String, Choose, Timestamp)>,
}

/// convert InputVote to Vote
fn convert(input: InputVote, id: VoteId) -> Vote {
    Vote {
        id,
        initiator: env::signer_account_id(),
        title: input.title,
        desc: input.desc,
        link: input.link,
        active: true,
        threshold: input.threshold,
        count: 0,
        finish: HashMap::new(),
        thinking: input.thinking,
        create_time: env::block_timestamp(),
        finish_time: None,
    }
}

/// Get vote id from inputVote
fn get_vote_id(input: &InputVote) -> VoteId {
    let initiator = env::signer_account_id();
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

#[near_bindgen]
impl YesOrNoContract {
    pub fn create_vote(&mut self, input_vote: InputVote) -> VoteId {
        let id = get_vote_id(&input_vote);

        let thinking_vote = &(id, input_vote.title.clone());
        // todo check those code gas cost.
        // need to check participant's accountId is legal or not.
        for x in &input_vote.thinking {
            assert!(env::is_valid_account_id(x.as_bytes()));

            match (&self.voter).get(x) {
                Some(mut voter) => {
                    voter.thinking.insert(thinking_vote);
                }
                None => {
                    let mut set = UnorderedSet::new(VoterThinking(x.clone()));
                    set.insert(thinking_vote);
                    self.voter.borrow_mut().insert(
                        x,
                        &Voter {
                            thinking: set,
                            finish: UnorderedSet::new(VoterFinish(x.clone())),
                        },
                    );
                }
            }
        }

        let vote_map = self.vote.borrow_mut();
        if vote_map.contains_key(&id) {
            panic!("exist same vote");
        }
        if (input_vote.thinking.len() as u64) < input_vote.threshold {
            panic!("illegal threshold or participant number");
        }

        let vote = convert(input_vote, id);

        vote_map.insert(&id, &vote);
        id
    }

    /// get active vote list by accountId
    ///
    /// Ps,It's a bug to use `signer_account_id` in a view method
    pub fn get_active_vote_list(
        &self,
        index: u64,
        limit: u64,
        account_id: AccountId,
    ) -> Vec<ActiveVote> {
        let voter = &self.voter;
        let voter_list = voter.get(&account_id);

        match voter_list {
            None => {
                vec![]
            }
            Some(voter) => {
                let thinking_set = &voter.thinking;

                if index >= thinking_set.len() {
                    return vec![];
                }

                (index..std::cmp::min(index + limit, thinking_set.len()))
                    .map(|index| {
                        let (vote_id, title) = thinking_set.as_vector().get(index).unwrap();
                        ActiveVote { vote_id, title }
                    })
                    .collect()
            }
        }
    }

    pub fn get_finish_vote_list(
        &self,
        index: u64,
        limit: u64,
        account_id: AccountId,
    ) -> Vec<FinishVote> {
        let voter = &self.voter;
        let voter_option = voter.get(&account_id);

        match voter_option {
            None => {
                vec![]
            }
            Some(voter) => {
                let finish_set = &voter.finish;

                if index >= finish_set.len() {
                    return vec![];
                }

                (index..std::cmp::min(index + limit, finish_set.len()))
                    .map(|index| {
                        let (vote_id, title, choose, finish_time) =
                            finish_set.as_vector().get(index).unwrap();
                        FinishVote {
                            vote_id,
                            title,
                            choose,
                            finish_time,
                        }
                    })
                    .collect()
            }
        }
    }

    pub fn vote(&mut self, vote_id: VoteId, choose: Choose) {
        let voter_map = &mut self.voter;
        let vote_map = &mut self.vote;

        let mut vote = vote_map.get(&vote_id).expect("no such vote");
        let account_id = env::signer_account_id();

        let mut voter = voter_map
            .get(&account_id)
            .expect("no vote for this accountId");

        let title = &vote.title;

        if !(&voter.thinking).contains(&(vote_id, title.clone())) {
            panic!("finished vote")
        }

        (voter.thinking.borrow_mut()).remove(&(vote_id, title.clone()));

        let timestamp = env::block_timestamp();
        (voter.finish.borrow_mut()).insert(&(vote_id, title.clone(), choose, timestamp));

        vote.thinking.remove(account_id.as_str());
        vote.finish.insert(account_id.clone(), (choose, timestamp));

        if choose {
            vote.count += 1;
        }

        // finish by yes
        if vote.count >= vote.threshold {
            vote.active = false;
            vote.finish_time = Some(timestamp);
        }

        // finish by no
        if (vote.thinking.len() as u64) + vote.count < vote.threshold {
            vote.active = false;
            vote.finish_time = Some(timestamp);
        }

        // flush data
        voter_map.insert(
            &account_id,
            &Voter {
                thinking: voter.thinking,
                finish: voter.finish,
            },
        );

        vote_map.insert(&vote.id, &vote);
    }

    // todo need change method
    pub fn get_vote(&self, vote_id: VoteId) -> Option<Vote> {
        (&self.vote).get(&vote_id)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain, VMContext};
    use std::convert::TryInto;
    use std::ops::Range;

    fn get_context(is_view: bool, index: usize) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(accounts(index).try_into().unwrap())
            .is_view(is_view)
            .build()
    }
    const TEST_TITLE: &str = "test";

    #[test]
    fn test_crate_vote() {
        let context = get_context(false, 1);
        testing_env!(context);

        let mut contract = YesOrNoContract::new();

        let input_vote = get_legal_vote(0..3);
        let vote_id = contract.create_vote(input_vote);

        let vote = contract.get_vote(vote_id).expect("test error:no such vote");

        assert_eq!(vote.title, TEST_TITLE);
        let alice: AccountId = accounts(1).try_into().unwrap();
        assert_eq!(vote.initiator, alice);

        (0..3).for_each(|temp| -> () {
            let context = get_context(false, temp);
            testing_env!(context);

            assert_eq!(
                contract
                    .get_active_vote_list(0, 10, accounts(temp).try_into().unwrap())
                    .len(),
                1
            );
            assert_eq!(
                contract
                    .get_finish_vote_list(0, 10, accounts(temp).try_into().unwrap())
                    .len(),
                0
            );

            contract.vote(vote_id, true);

            assert_eq!(
                contract
                    .get_active_vote_list(0, 10, accounts(temp).try_into().unwrap())
                    .len(),
                0
            );
            assert_eq!(
                contract
                    .get_finish_vote_list(0, 10, accounts(temp).try_into().unwrap())
                    .len(),
                1
            );
        });

        let vote = contract.get_vote(vote_id).unwrap();

        assert_eq!(vote.count, 3);
        assert_eq!(vote.active, false);
        assert_eq!(vote.finish.len(), 3);
        assert_eq!(vote.thinking.len(), 0);
        assert!(vote.count >= vote.threshold);
        vote.finish_time.unwrap();
    }

    #[test]
    fn test_vote() {
        let context = get_context(false, 0);
        testing_env!(context);

        let mut contract = YesOrNoContract::new();

        let input_vote = get_legal_vote(0..3);
        let vote_id = contract.create_vote(input_vote);

        contract.vote(vote_id, true);

        let finish_vec = contract.get_finish_vote_list(0, 10, accounts(0).try_into().unwrap());
        let active_vec = contract.get_active_vote_list(0, 10, accounts(0).try_into().unwrap());
        assert_eq!(finish_vec.len(), 1);
        assert_eq!(active_vec.len(), 0);
    }

    #[test]
    #[should_panic(expected = "exist same vote")]
    fn test_crate_vote_repeat() {
        let context = get_context(false, 0);
        testing_env!(context);

        let mut contract = YesOrNoContract::new();

        let input_vote = get_legal_vote(0..3);
        contract.create_vote(input_vote);

        let input_vote = get_legal_vote(0..3);
        contract.create_vote(input_vote);
    }

    fn get_legal_vote(range: Range<usize>) -> InputVote {
        InputVote {
            title: TEST_TITLE.to_string(),
            desc: None,
            link: None,
            threshold: 2,
            thinking: range.map(|temp| accounts(temp).to_string()).collect(),
        }
    }
}
