use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
use tea_sdk::tapp::{Account, Balance};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    pub id: String,
    pub title: String,
    pub description: String,
    pub owner: Account,
    pub create_at: u64,
    pub total_contribution: String,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, AsRefStr, EnumString, Display,
)]
pub enum Status {
    New,
    InProgress,
    WaitForVerification,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsRefStr, Display)]
pub enum Txns {
    Init {},
    CreateIdea {
        id: String,
        title: String,
        description: String,
        owner: Account,
        auth_b64: String,
        unit: Balance,
    },
    VoteIdea {
        id: String,
        user: Account,
        auth_b64: String,
        price: Balance,
    },
}
