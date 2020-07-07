use std::fs;
use crate::model::ledger;

pub fn parse_ledger(filename: &String) -> ledger::Ledger {
    let file_path = fs::read_to_string(filename).unwrap();
    toml::from_str(&file_path).unwrap()
}