use std::collections::HashMap;
use crate::util::*;
use crate::cli::*;
use crate::model::{transaction, account};
use monee::*;

#[derive(Debug, StructOpt)]
pub struct BalanceOpt { }

#[derive(Debug, Clone)]
struct Balance {
    accounts: HashMap<String, account::Account>,
    check: f64
}

#[macro_export]
macro_rules! balance {
    ($x:expr) => {
        Balance::new($x)
    };
}

impl Balance {
    fn new(transactions: Option<Vec<transaction::Transaction>>) -> Balance {
        if let Some(transactions) = transactions {
            let processed = process_transactions(transactions);
            Balance {
                accounts: processed.0,
                check: processed.1
            }
        } else {
            Balance {
                accounts: HashMap::new(),
                check: 0.0
            }
        }
    }

    pub fn print(self) {
        println!();

        let mut accounts: Vec<(String, account::Account)> = self.accounts
            .into_iter()
            .collect();
        accounts.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (_, account) in accounts.iter() {
            println!("{0: >15}  {1: <}",
                format!("{: >1}", money!(account.balance, "USD")),
                account.name
            );
        }
        println!("{:->15}", "");
        println!("{:>15}", self.check);

        println!();
    }
}

pub fn process_transactions(transactions: Vec<transaction::Transaction>) -> (HashMap<String, account::Account>, f64) {
    let mut accounts: HashMap<String, account::Account> = HashMap::new();
    let mut check: f64 = 0.0;

    for transaction in transactions {
        let parsed = transaction.parse();

        for (_, account) in parsed.accounts.iter() {
            match accounts.get_mut(&account.name) {
                Some(acc) => {
                    acc.balance += &account.balance;
                },
                None => {
                    accounts.insert(
                        account.name.to_string(),
                        account::Account {
                            name: account.name.to_string(),
                            balance: account.balance,
                        }
                    );
                }
            }

            check += account.balance;
        }
    }
    
    (accounts, check)
}

pub fn eval(_cli: &Cli, _cmd: &BalanceOpt) -> Result<(), std::io::Error> {
    let ledger = file::load();

    let balances = Balance::new(ledger.transaction);
    balances.print();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_empty_transactions() {
        let balance = Balance {
            accounts: HashMap::new(),
            check: 0.0
        };
        let test_account = balance!(None);
        assert_eq!(
            test_account.accounts.is_empty(),
            balance.accounts.is_empty()
        );
    }
}
