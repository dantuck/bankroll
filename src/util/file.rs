use std::fs;
use crate::model::ledger;

pub fn load() -> ledger::Ledger {
    let ledger_file_env = match std::env::var("LEDGER_FILE") {
        Ok(p) => format!("{}", p),
        Err(_) => format!("{}", ""),
    };

    parse_ledger(&ledger_file_env.to_string())
}

pub fn parse_ledger(filename: &String) -> ledger::Ledger {
    let file_path = fs::read_to_string(filename).unwrap();
    toml::from_str(&file_path).unwrap()
}