use serde::Deserialize;

use super::default;
use super::transaction::transaction::{
    Transaction,
    TransactionMeta
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ledger {
    pub default: Option<default::Default>,
    pub transaction: Option<Vec<Transaction>>
}

impl Ledger {
    pub fn parse_transactions(self) -> Vec<TransactionMeta> {
        let mut transactions_vec: Vec<TransactionMeta> = Vec::new();

        if let Some(transactions) = self.transaction {
            for transaction in transactions {
                let test = transaction.parse();
                transactions_vec.push(test);
            }
        }

        transactions_vec
    }
}
