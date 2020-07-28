use serde::{Serialize, Deserialize};
use csv::Reader;

use crate::model::transaction::transaction::Transaction;
use crate::util::file;
use crate::error::{Result, Error, ErrorKind};

#[derive(Debug, Deserialize)]
struct Record {
    status: Option<String>,
    date: String,
    description: String,
    amount: f64
}

#[derive(Serialize)]
pub struct Bank {
    pub transaction: Vec<Transaction>,
}

pub fn import(file: &String, for_account: &Option<String>) -> Result<()> {
    let defaults = file::get_ledger_defaults()?;

    let account: String;
    let account_offset: String;

    if let Some(from_for_account) = for_account {
        account = from_for_account.to_string()
    } else if let Some(default_account) = defaults.account {
        account = default_account
    } else {
        return Err(Error::new(ErrorKind::InvalidInput("account missing".to_string()), None))
    }

    if let Some(default_account_offset) = defaults.account_offset {
        account_offset = default_account_offset
    } else {
        return Err(Error::new(ErrorKind::InvalidInput("account_offset missing".to_string()), None))
    }

    let transactions = parse_import(file, &account, &account_offset)?;
    if let Err(_error) = write_to_ledger(transactions) {
        return Err(Error::new(ErrorKind::Parsing("transactions".to_string()), None))
    }

    Ok(())
}

fn write_to_ledger(transactions: Vec<Transaction>) -> Result<()> {
    let toml = toml::to_string(&Bank {
        transaction: transactions
    }).unwrap();
    
    if let Err(err) = file::write_to_ledger(toml.as_bytes()) {
        match err.into_kind() {
            ErrorKind::Io(err, None) => {
                return Err(Error::new(ErrorKind::Io(err, Some("Unable to write".to_string())), None))
            },
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn parse_import(file: &String, account: &String, account_offset: &String) -> Result<Vec<Transaction>> {
    let mut transactions: Vec<Transaction> = Vec::new();

    let mut rdr = Reader::from_path(file)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        
        transactions.push(Transaction {
            date: record.date,
            description: record.description,
            account: Some(account.to_string()),
            amount: Some(record.amount),
            account_offset: Some(account_offset.to_string()),
            post: None,
            fund: None
        });
    }

    Ok(transactions)
}