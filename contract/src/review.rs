use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, AccountId};

use crate::key::ItemKeys::{Active, Negative};
use crate::{ItemId, YesOrNoContract};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    id: ItemId,

    initiator: AccountId,

    title: String,

    desc: Option<String>,

    link: Option<String>,

    active: UnorderedSet<AccountId>,

    negative: UnorderedSet<AccountId>,
}

impl YesOrNoContract {
    pub fn create_review(&mut self, title: String, desc: Option<String>, link: Option<String>) {
        let initiator = env::current_account_id();

        let empty = String::default();

        let desc_ref = desc.as_ref().unwrap_or(&empty);
        let link_ref = link.as_ref().unwrap_or(&empty);

        let item_id: ItemId =
            env::sha256((initiator.clone() + &title + desc_ref + link_ref).as_bytes())
                .try_into()
                .expect("get cryptoHash error");

        let review = &mut self.review;

        if review.get(&item_id).is_none() {
            panic!("exist same review")
        };

        review.insert(
            &item_id,
            &Item {
                id: item_id,
                initiator,
                title,
                desc,
                link,
                active: UnorderedSet::new(Active(item_id)),
                negative: UnorderedSet::new(Negative(item_id)),
            },
        );
    }
}

mod tests {

    #[test]
    fn test() {}
}
