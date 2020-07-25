use crate::cli::*;
use crate::util::*;
use crate::model::{ledger, transaction, account};
use monee::*;
use transaction::*;
use account::*;

use crate::error::Result;

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
            if let Some((first, posts)) = transaction_meta.posts.split_first() {
                balance += first.amount;

                println!("{0: <10} {1: <30} {2: <20} {3: >15} {4: >15}", 
                    transaction_meta.date,
                    transaction_meta.description.fit_to(30),
                    fitaccount(&first.account, 20),
                    format!("{: >1}", money!(first.amount, "USD")),
                    format!("{: >1}", money!(balance, "USD"))
                );

                for post in posts {
                    balance += post.amount;

                    println!("{0: <41} {1: <20} {2: >15} {3: >15}",
                        "",
                        fitaccount(&post.account, 20),
                        format!("{: >1}", money!(post.amount, "USD")),
                        format!("{: >1}", money!(balance, "USD"))
                    );
                }
            }
        }

        println!();
    }
}

pub fn eval(_cli: &Cli, _cmd: &RegisterOpt) -> Result<()> {
    let ledger_file = file::load()?;

    register!(ledger_file).print();

    Ok(())
}