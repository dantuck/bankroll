use serde::Deserialize;
// use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use crate::model::{account};

#[derive(Deserialize, Clone)]
pub struct Entry {
    pub amount: f64,
    pub account: String,
}

#[derive(Deserialize, Clone)]
pub struct Transaction {
    pub description: String,
    pub account: Option<String>,
    pub account_offset: Option<String>,
    pub amount: Option<f64>,
    pub entry: Option<Vec<Entry>>,
}

pub struct TransactionMeta {
    pub accounts: HashMap<String, account::Account>
}

impl Transaction {
    pub fn parse(mut self) -> TransactionMeta {
        let mut check: f64 = 0.0;
        let mut accounts: HashMap<String, account::Account> = HashMap::new();

        if let Some(entries) = &self.entry {
            for entry in entries {
                check += entry.amount;
            }
        }
        else if let Some(_t) = self.amount {
            let offset_account = &self.account_offset;
            let entry = Entry {
                amount: self.amount.unwrap(),
                account: offset_account.as_ref().unwrap().to_string()
            };

            check += entry.amount;
            self.entry = Some(
                vec![
                    entry
                ]
            ); 
        }

        if check != 0.0 {
            if let Some(account) = self.account {
                let check_inverse = if check > 0.0 { -check } else { check.abs() };
                let mut entries = self.entry.unwrap();
                entries.push(Entry {
                    amount: check_inverse,
                    account: account,
                });

                self.entry = Some(entries);
            }
        }

        for entry in self.entry.unwrap().iter() {
            match accounts.get_mut(&entry.account) {
                Some(account) => {
                    account.balance += &entry.amount;
                },
                None => {
                    accounts.insert(
                        entry.account.to_string(),
                        account::Account {
                            name: entry.account.to_string(),
                            balance: entry.amount,
                        }
                    );
                }
            }
        }

        TransactionMeta {
            accounts: accounts,
        }
    }
}
