use crate::cli::*;
use crate::util::*;
use crate::model::{ledger, transaction, account};
use monee::*;
use transaction::*;
use account::*;

#[derive(Debug, StructOpt)]
pub struct RegisterOpt { }

#[derive(Debug, Clone)]
struct Register {
    transactions: Vec<transaction::TransactionMeta>
}

#[macro_export]
macro_rules! register {
    ($x: expr) => {
        Register::new($x)
    };
}

impl Register {
    fn new(ledger: ledger::Ledger) -> Register {
        let transactions = ledger.parse_transactions();

        Register {
            transactions: transactions
        }
    }

    fn print(self) {
        println!();

        for transaction_meta in self.transactions {
            let mut balance: f64 = 0.0;
            if let Some((first, entries)) = transaction_meta.entries.split_first() {
                balance += first.amount;

                println!("{0: <10} {1: <30} {2: <20} {3: >15} {4: >15}", 
                    transaction_meta.date,
                    transaction_meta.description.fit_to(30),
                    fitaccount(&first.account, 20),
                    format!("{: >1}", money!(first.amount, "USD")),
                    format!("{: >1}", money!(balance, "USD"))
                );

                for entry in entries {
                    balance += entry.amount;

                    println!("{0: <41} {1: <20} {2: >15} {3: >15}",
                        "",
                        fitaccount(&entry.account, 20),
                        format!("{: >1}", money!(entry.amount, "USD")),
                        format!("{: >1}", money!(balance, "USD"))
                    );
                }
            }
        }

        println!();
    }
}

pub fn eval(_cli: &Cli, _cmd: &RegisterOpt) -> Result<(), std::io::Error> {
    let register = register!(file::load());
    register.print();

    Ok(())
}