use crate::cli::*;
use crate::model::{account, ledger, transaction};
use crate::util::*;
use account::*;
use monee::*;
use transaction::transaction::*;

use crate::error::Result;

#[derive(Debug, StructOpt)]
pub struct RegisterOpt {
    /// Only shows real transactions
    #[structopt(short, long)]
    real: bool,
}

#[derive(Debug, Clone)]
struct Register {
    transactions: Vec<TransactionMeta>,
}

#[macro_export]
macro_rules! register {
    ($x: expr) => {
        Register::new($x)
    };
}

fn print_transactions(transactions: Vec<TransactionMeta>, opts: &RegisterOpt) {
    for transaction_meta in transactions {
        let mut balance: f64 = 0.0;
        if let Some((first, posts)) = transaction_meta.posts.split_first() {
            balance += first.amount;

            println!(
                "{0: <10} {1: <30} {2: <20} {3: >15} {4: >15}",
                transaction_meta.date,
                transaction_meta.description.fit_to(30),
                fitaccount(&first.account, 20),
                format!("{: >1}", money!(first.amount, "USD")),
                format!("{: >1}", money!(balance, "USD"))
            );

            for post in posts {
                balance += post.amount;

                println!(
                    "{0: <41} {1: <20} {2: >15} {3: >15}",
                    "",
                    fitaccount(&post.account, 20),
                    format!("{: >1}", money!(post.amount, "USD")),
                    format!("{: >1}", money!(balance, "USD"))
                );
            }

            if !opts.real {
                if let Some(funds) = transaction_meta.funds {
                    for fund in funds {
                        println!(
                            "{0: <41} {1: <20} {2: >15} {3: >15}",
                            "",
                            fitaccount(&format!("({})", &fund.name), 20),
                            format!("{: >1}", money!(fund.amount, "USD")),
                            "".to_string()
                        );
                    }
                }
            }
        }
    }
}

impl Register {
    fn new(ledger: ledger::Ledger) -> Register {
        let transactions = ledger.parse_transactions();

        Register {
            transactions: transactions,
        }
    }

    fn print(self, opts: &RegisterOpt) {
        println!();

        print_transactions(self.transactions, opts);

        println!();
    }
}

pub fn eval(_cli: &Cli, opts: &RegisterOpt) -> Result<()> {
    let ledger_file = file::load()?;

    register!(ledger_file).print(opts);

    Ok(())
}
