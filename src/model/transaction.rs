use serde::Deserialize;
use std::collections::HashMap;

use crate::model::{account};

#[derive(Deserialize, Clone, Debug)]
pub struct Entry {
    pub amount: f64,
    pub account: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub account: Option<String>,
    pub account_offset: Option<String>,
    pub amount: Option<f64>,
    pub entry: Option<Vec<Entry>>,
}

#[derive(Clone, Debug)]
pub struct TransactionMeta {
    pub date: String,
    pub description: String,
    pub entries: Vec<Entry>,
    pub accounts: HashMap<String, account::Account>,
}

impl Transaction {
    pub fn parse(self) -> TransactionMeta {
        let mut check: f64 = 0.0;
        let mut accounts: HashMap<String, account::Account> = HashMap::new();
        let mut entries_parsed: Vec<Entry> = Vec::new();

        if let Some(entries) = self.entry {
            for entry in entries {
                check += entry.amount;
                entries_parsed.push(entry);
            }
        }
        else if let Some(_t) = self.amount {
            let offset_account = &self.account_offset;
            let entry = Entry {
                amount: self.amount.unwrap(),
                account: offset_account.as_ref().unwrap().to_string()
            };

            check += entry.amount;
            entries_parsed.push(entry);
        }

        if check != 0.0 {
            if let Some(account) = self.account {
                let check_inverse = if check > 0.0 { -check } else { check.abs() };
                
                entries_parsed.push(Entry {
                    amount: check_inverse,
                    account: account,
                });
            }
        }

        for entry in &entries_parsed {
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
            date: self.date,
            description: self.description,
            entries: entries_parsed,
            accounts: accounts,
        }
    }
}
