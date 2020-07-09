use serde::Deserialize;

use super::default;
use super::transaction;

#[derive(Deserialize, Debug, Clone)]
pub struct Ledger {
    pub default: Option<default::Default>,
    pub transaction: Option<Vec<transaction::Transaction>>
}

impl Ledger {
    pub fn parse_transactions(self) -> Vec<transaction::TransactionMeta> {
        let mut transactions_vec: Vec<transaction::TransactionMeta> = Vec::new();

        if let Some(transactions) = self.transaction {
            for transaction in transactions {
                let test = transaction.parse();
                transactions_vec.push(test);
            }
        }

        transactions_vec
    }
}
