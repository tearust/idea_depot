#![feature(min_specialization)]

use serde::{Deserialize, Serialize};
use tea_sdk::{serde::TypeId, tapp::Account};

pub mod error;
pub mod txn;

pub const NAME: &[u8] = b"com.developer.idea-vote-state-actor";

#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct TaskQueryRequest {
    pub creator: Option<Account>,
    pub worker: Option<Account>,
    pub status: Option<txn::Status>,
    pub subject: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct TaskQueryResponse(pub Vec<txn::Task>);
