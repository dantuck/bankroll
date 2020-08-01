use ansi_term::Colour::{Green, Red, White, RGB};
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;

use crate::cli::*;
use crate::model::transaction::{fund::Fund, transaction::Transaction};
use crate::model::{
    account,
    default::{AccountSetting, AccountThreshold, Default, ThresholdOperator},
    ledger::Ledger,
};
use crate::util::*;

use monee::{self, money, Money};

use crate::error::Result;

#[derive(Debug, StructOpt)]
pub struct BalanceOpt {
    /// Only shows real transactions
    #[structopt(short, long)]
    real: bool,
}

#[derive(Debug, Clone)]
struct Balance {
    accounts: HashMap<String, account::Account>,
    fund_accounts: Option<HashMap<String, account::Account>>,
    check: f64,
    defaults: Option<Default>,
}

#[macro_export]
macro_rules! balance {
    ($x:expr, $y:expr) => {
        Balance::new($x, $y)
    };
}

/// Prints a horizontal line
///
/// @param width (required)
///
/// @param color (Option or None) default = `RGB(255, 140, 0)`
///
/// @param line_char (Option or None) default `─`
fn print_horizontal_line(
    width: usize,
    color: Option<ansi_term::Colour>,
    line_char: Option<char>,
    text: Option<String>,
) {
    let color = color.unwrap_or(RGB(255, 140, 0));
    let hline = line_char.unwrap_or('─').to_string().repeat(width);

    let output = color.normal();

    println!("{} {}", output.paint(hline), text.unwrap_or_default());
}

fn print_account_ln(account: &account::Account, settings: Option<&AccountSetting>) {
    let mut ansi_color: ansi_term::Colour = White;

    if account.balance >= 0.0 {
        ansi_color = Green
    } else if account.balance < 0.0 {
        ansi_color = Red
    }

    if let Some(settings) = settings {
        for threshold in &settings.account_thresholds {
            match threshold {
                AccountThreshold::Limit {
                    limit,
                    color,
                    operator,
                } => match operator {
                    ThresholdOperator::LessThan => {
                        if account.balance < *limit {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                    ThresholdOperator::LessThanOrEqual => {
                        if account.balance <= *limit {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                    ThresholdOperator::Equal => {
                        if account.balance.eq(limit) {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                    ThresholdOperator::MoreThanOrEqual => {
                        if account.balance >= *limit {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                    ThresholdOperator::MoreThan => {
                        if account.balance > *limit {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                    ThresholdOperator::Between(lower, upper) => {
                        if account.balance > *lower && account.balance < *upper {
                            ansi_color = color.to_ansi_color()
                        }
                    }
                },
            }
        }
    }

    let money_formatted = ANSIString::from(ansi_color.paint(format!(
        "{: >15}",
        format!("{: >1}", money!(account.balance, "USD"))
    )));

    let strings: &[ANSIString<'static>] = &[
        money_formatted,
        ANSIString::from(" "),
        ANSIString::from(format!("{: <}", account.name)),
    ];

    println!("{}", ANSIStrings(strings))
}

impl Balance {
    fn new(transactions: Option<Vec<Transaction>>, defaults: Option<Default>) -> Balance {
        if let Some(transactions) = transactions {
            let processed = process_transactions(transactions);

            Balance {
                accounts: processed.0,
                fund_accounts: Some(processed.1),
                check: processed.2,
                defaults,
            }
        } else {
            Balance {
                accounts: HashMap::new(),
                fund_accounts: None,
                check: 0.0,
                defaults,
            }
        }
    }

    pub fn print(self, opts: &BalanceOpt) {
        println!();

        let mut accounts: Vec<(String, account::Account)> = self.accounts.into_iter().collect();
        accounts.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let defaults = &self.defaults.unwrap_or(Default {
            account: None,
            account_offset: None,
            setting: None,
        });

        let account_settings = defaults.get_account_settings();

        for (_, account) in accounts.iter() {
            let account_setting = account_settings.get(&account.name);
            print_account_ln(account, account_setting);
        }

        print_horizontal_line(15, None, None, None);
        println!("{:>15}", format!("{: >1}", money!(self.check, "USD")));

        if !opts.real {
            if let Some(fund_account) = self.fund_accounts {
                if !fund_account.is_empty() {
                    println!();
                    print_horizontal_line(15, None, None, Some("Funds".to_string()));
                    let mut fund_accounts: Vec<(String, account::Account)> =
                        fund_account.into_iter().collect();
                    fund_accounts.sort_unstable_by(|a, b| a.0.cmp(&b.0));

                    for (_, fund_account) in fund_accounts.iter() {
                        print_account_ln(fund_account, None);
                    }
                }
            }
        }

        println!();
    }
}

pub fn process_transactions(
    transactions: Vec<Transaction>,
) -> (
    HashMap<String, account::Account>,
    HashMap<String, account::Account>,
    f64,
) {
    let mut accounts: HashMap<String, account::Account> = HashMap::new();
    let mut fund_accounts: HashMap<String, account::Account> = HashMap::new();
    let mut check: f64 = 0.0;

    for transaction in transactions {
        let parsed = transaction.parse();

        for (_, account) in parsed.accounts.iter() {
            match accounts.get_mut(&account.name) {
                Some(acc) => {
                    acc.balance += &account.balance;
                }
                None => {
                    accounts.insert(
                        account.name.to_string(),
                        account::Account {
                            name: account.name.to_string(),
                            balance: account.balance,
                        },
                    );
                }
            }

            check += account.balance;
        }

        if let Some(fund) = parsed.funds {
            for fund in Fund::parse_to_accounts(&fund) {
                match fund_accounts.get_mut(&fund.0) {
                    Some(account) => {
                        account.balance += &fund.1.balance;
                    }
                    None => {
                        fund_accounts.insert(fund.0, fund.1);
                    }
                }
            }
        }
    }

    (accounts, fund_accounts, check)
}

pub fn eval(_cli: &Cli, opts: &BalanceOpt) -> Result<()> {
    let ledger_file: Ledger = file::load()?;

    balance!(ledger_file.transaction, ledger_file.default).print(opts);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_empty_transactions() {
        let balance = Balance {
            accounts: HashMap::new(),
            fund_accounts: None,
            check: 0.0,
            defaults: None,
        };

        let test_account = balance!(None, None);
        assert_eq!(
            test_account.accounts.is_empty(),
            balance.accounts.is_empty()
        );
    }
}
