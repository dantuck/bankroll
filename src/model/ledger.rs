use serde::Deserialize;

use super::default;
use super::transaction;

#[derive(Deserialize)]
pub struct Ledger {
    pub default: Option<default::Default>,
    pub transaction: Option<Vec<transaction::Transaction>>
}