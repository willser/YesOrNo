use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId};
use serde::Serialize;

use crate::key::ItemKeys::{Active, Negative};
use crate::YesOrNoContractContract;
use crate::*;
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct Item {
    id: ItemId,

    initiator: AccountId,

    title: String,

    desc: Option<String>,

    link: Option<String>,

    #[serde(skip_serializing)]
    active: UnorderedSet<AccountId>,

    #[serde(skip_serializing)]
    negative: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl YesOrNoContract {
    pub fn create_review(
        &mut self,
        title: String,
        desc: Option<String>,
        link: Option<String>,
    ) -> ItemId {
        let initiator = env::signer_account_id();

        let empty = String::default();

        let desc_ref = desc.as_ref().unwrap_or(&empty);
        let link_ref = link.as_ref().unwrap_or(&empty);

        let item_id: ItemId =
            env::sha256((initiator.clone() + &title + desc_ref + link_ref).as_bytes())
                .try_into()
                .expect("get cryptoHash error");

        let review = &mut self.review;

        if review.get(&item_id).is_some() {
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

        item_id
    }

    /// post a review about item
    pub fn review(&mut self, item_id: ItemId, opinion: Choose) {
        let review = &mut self.review;

        let item = &mut review.get(&item_id).expect("no such item.");

        let account_id = &env::signer_account_id();

        if opinion {
            item.negative.remove(account_id);
            item.active.insert(account_id);
        } else {
            item.negative.insert(account_id);
            item.active.remove(account_id);
        }

        review.insert(&item_id, item);
    }

    /// get item by itemId
    pub fn get_item(&self, item_id: ItemId) -> Item {
        let review = &self.review;
        review.get(&item_id).expect("no such item.")
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::YesOrNoContract;
    use near_sdk::env;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    #[test]
    fn test_create_review() {
        let context = VMContextBuilder::new()
            .signer_account_id(accounts(1).try_into().unwrap())
            .is_view(false)
            .build();

        testing_env!(context);

        let mut contract = YesOrNoContract::new();

        let item_id = contract.create_review(
            String::from("title"),
            Option::Some(String::from("desc")),
            Option::Some(String::from("https://github.com")),
        );

        contract.review(item_id, true);
        let item = &contract.get_item(item_id);

        assert_eq!(item.active.len(), 1);
        assert_eq!(item.negative.len(), 0);

        contract.review(item_id, false);
        let item = &contract.get_item(item_id);
        assert_eq!(item.active.len(), 0);
        assert_eq!(item.negative.len(), 1);
    }

    #[test]
    #[should_panic(expected = "no such item.")]
    fn test_illegal_id() {
        let context = VMContextBuilder::new()
            .signer_account_id(accounts(1).try_into().unwrap())
            .is_view(false)
            .build();

        testing_env!(context);

        let mut contract = YesOrNoContract::new();
        contract.review(env::sha256("".as_bytes()).try_into().unwrap(), false);
    }
}
