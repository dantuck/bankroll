use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::account::Account;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fund {
    pub account: Option<String>,
    pub name: String,
    pub amount: f64,
}

impl Fund {
    pub fn parse_to_rolls(
        funds: &Vec<Fund>,
    ) -> (HashMap<String, Account>, HashMap<String, Account>) {
        let mut fund_rolls: HashMap<String, Account> = HashMap::new();
        let mut funds_by_accounts: HashMap<String, Account> = HashMap::new();

        for fund in funds {
            match fund_rolls.get_mut(&fund.name) {
                Some(fund_roll) => {
                    fund_roll.balance += &fund.amount;
                }
                None => {
                    fund_rolls.insert(
                        fund.name.to_string(),
                        Account {
                            name: fund.name.to_string(),
                            balance: fund.amount,
                        },
                    );
                }
            }

            if let Some(account) = &fund.account {
                match funds_by_accounts.get_mut(account) {
                    Some(account) => {
                        account.balance += &fund.amount;
                    }
                    None => {
                        funds_by_accounts.insert(
                            account.to_string(),
                            Account {
                                name: fund.name.to_string(),
                                balance: fund.amount,
                            },
                        );
                    }
                }
            }
        }

        (fund_rolls, funds_by_accounts)
    }
}
