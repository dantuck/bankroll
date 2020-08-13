use ansi_term::Colour::{Green, Red, White, RGB};
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;

use crate::cli::*;
use crate::model::{
    account::Account,
    default::{AccountSetting, AccountThreshold, Default, ThresholdOperator},
    ledger::Accounts,
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
    accounts: Accounts,
    defaults: Option<Default>,
}

#[macro_export]
macro_rules! balance {
    ($x:expr) => {
        Balance::new($x.0, $x.1)
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

fn print_account_ln(
    account: &Account,
    settings: Option<&AccountSetting>,
    indent_by: Option<usize>,
) {
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

    let account_name: String = account.name.to_string();

    let money_formatted = ANSIString::from(ansi_color.paint(format!(
        "{: >15}",
        format!("{: >1}", money!(account.balance, "USD"))
    )));

    let strings: &[ANSIString<'static>] = &[
        money_formatted,
        ANSIString::from(" ".repeat(indent_by.unwrap_or(1))),
        ANSIString::from(format!("{: <}", account_name)),
    ];

    println!("{}", ANSIStrings(strings));
}

impl Balance {
    fn new(accounts: Accounts, defaults: Option<Default>) -> Balance {
        Balance { accounts, defaults }
    }

    fn get_by_account_type(
        self,
        opts: &BalanceOpt,
    ) -> (HashMap<String, (Account, usize)>, Default) {
        let mut map: HashMap<String, (Account, usize)> = HashMap::new();

        let accounts: &Vec<(String, (Account, HashMap<String, Account>))> =
            &self.accounts.into_iter().collect();

        for (account_key, element) in accounts.into_iter() {
            let mut indented_by: usize = 1;
            let account = &element.0;
            let funds = &element.1;
            let account_name_vec: Vec<&str> = account_key.split_terminator(':').collect();

            // get the account type
            if let Some(account_type) = account_name_vec.first() {
                // check if account_type is already in the result
                if let Some(parent) = map.get_mut(&account_type.to_string()) {
                    // found parent
                    // add balance to parent
                    parent.0.balance += account.balance;
                    indented_by += parent.1;

                    map.insert(
                        account_name_vec.split_at(2).0.join(":"),
                        (
                            Account {
                                name: account_name_vec.split_at(1).1.join(":"),
                                balance: account.balance,
                            },
                            indented_by,
                        ),
                    );
                } else {
                    map.insert(
                        account_type.to_string(),
                        (
                            Account {
                                name: account_type.to_string(),
                                balance: account.balance,
                            },
                            indented_by,
                        ),
                    );

                    indented_by += 1;
                    map.insert(
                        account_name_vec.split_at(2).0.join(":"),
                        (
                            Account {
                                name: account_name_vec.split_at(1).1.join(":"),
                                balance: account.balance,
                            },
                            indented_by,
                        ),
                    );
                }

                if !opts.real {
                    for (_, fund) in funds.into_iter() {
                        if let Some(account) =
                            map.get_mut(&account_name_vec.split_at(2).0.join(":"))
                        {
                            account.0.balance -= fund.balance;
                        }
                        indented_by += 1;
                        map.insert(
                            account_name_vec.split_at(2).0.join(":") + &fund.name.to_owned(),
                            (
                                Account {
                                    name: fund.name.to_string(),
                                    balance: fund.balance,
                                },
                                indented_by,
                            ),
                        );
                    }
                }
            }
        }

        (map, self.defaults.unwrap_or_default())
    }

    pub fn print(self, opts: &BalanceOpt) {
        println!();

        let accounts_by_type = self.get_by_account_type(opts);
        let account_settings = accounts_by_type.1.get_account_settings();

        let mut accounts: Vec<(String, (Account, usize))> =
            accounts_by_type.0.into_iter().collect();
        accounts.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut balance_check: f64 = 0.0;
        for (_, account) in accounts {
            balance_check += &account.0.balance;
            let account_setting = account_settings.get(&account.0.name);
            print_account_ln(&account.0, account_setting, Some(account.1));
        }

        print_horizontal_line(15, None, None, None);
        println!("{:>15}", balance_check);

        println!();
    }
}

pub fn eval(_cli: &Cli, opts: &BalanceOpt) -> Result<()> {
    let ledger_file = file::load()?.parse();

    balance!(ledger_file).print(opts);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_balance_empty_transactions() {
//         let balance = Balance {
//             accounts: HashMap::new(),
//             defaults: None,
//         };

//         let test_account = balance!((None, None));
//         assert_eq!(
//             test_account.accounts.is_empty(),
//             balance.accounts.is_empty()
//         );
//     }
// }
