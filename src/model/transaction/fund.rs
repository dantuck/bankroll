use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::model::account::Account;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fund {
    pub account: String,
    pub amount: f64
}

impl Fund {
    pub fn parse_to_accounts(funds: &Vec<Fund>) -> HashMap<String, Account> {
        let mut accounts: HashMap<String, Account> = HashMap::new();

        for fund in funds {
            match accounts.get_mut(&fund.account) {
                Some(account) => {
                    account.balance += &fund.amount;
                },
                None => {
                    accounts.insert(
                        fund.account.to_string(),
                        Account {
                            name: fund.account.to_string(),
                            balance: fund.amount,
                        }
                    );
                }
            }
        }

        accounts
    }
}