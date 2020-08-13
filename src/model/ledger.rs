use serde::Deserialize;
use std::collections::HashMap;

use super::account::Account;
use super::default;
use super::transaction::{
    fund::Fund,
    transaction::{Transaction, TransactionMeta},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ledger {
    pub default: Option<default::Default>,
    pub transaction: Option<Vec<Transaction>>,
}

/// Type alias for `HashMap<String, (Account, HashMap<String, Account>)>`
pub type Accounts = HashMap<String, (Account, HashMap<String, Account>)>;

/// Type alias for `HashMap<String, Account>,`
pub type Funds = HashMap<String, Account>;

impl Ledger {
    pub fn parse_transactions(self) -> Vec<TransactionMeta> {
        let mut transactions_vec: Vec<TransactionMeta> = Vec::new();

        if let Some(transactions) = self.transaction {
            for transaction in transactions {
                transactions_vec.push(transaction.parse());
            }
        }

        transactions_vec
    }

    pub fn parse(self) -> (Accounts, Option<default::Default>) {
        // HashMap by account_name with (Account, FundRolls)
        let mut accounts: Accounts = HashMap::new();
        let mut funds: Funds = HashMap::new();

        if let Some(transactions) = self.transaction {
            for transaction in transactions {
                let parsed = transaction.parse();

                for (account_name, account) in parsed.accounts {
                    match accounts.get_mut(&account_name) {
                        Some(found) => {
                            found.0.balance += &account.balance;
                        }
                        None => {
                            accounts.insert(
                                account_name,
                                (
                                    Account {
                                        name: account.name.to_string(),
                                        balance: account.balance,
                                    },
                                    HashMap::new(),
                                ),
                            );
                        }
                    }
                }

                if let Some(fund) = parsed.funds {
                    let parsed_fund = Fund::parse_to_rolls(&fund);
                    for fund in parsed_fund.0 {
                        match funds.get_mut(&fund.0) {
                            Some(fund_roll) => {
                                fund_roll.balance += &fund.1.balance;
                            }
                            None => {
                                funds.insert(fund.0, fund.1);
                            }
                        }
                    }

                    for fund_account in parsed_fund.1 {
                        match accounts.get_mut(&fund_account.0) {
                            Some(account) => match account.1.get_mut(&fund_account.1.name) {
                                Some(account_fund) => {
                                    account_fund.balance += fund_account.1.balance;
                                }
                                None => {
                                    let mut something: HashMap<String, Account> = HashMap::new();
                                    something
                                        .insert(fund_account.1.name.to_string(), fund_account.1);
                                    account.1 = something
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        (accounts, self.default)
    }
}
