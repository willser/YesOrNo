use crate::{Choose, ThingsId};
use near_sdk::AccountId;

struct Things {
    id: ThingsId,

    initiator: AccountId,

    title: String,

    desc: Option<String>,

    link: Option<String>,

    active: u64,

    negative: u64,
}

struct Review {
    account_id: AccountId,
    reviews: Option<String>,
    attitude: Choose,
    posted_time: u64,
}
