use std::{fs, fs::OpenOptions};
use std::{io::Write};
use crate::error::{
    Result,
    Error,
    ErrorKind
};

use crate::model::{
    ledger,
    default,
};

pub fn get_file_name() -> String {
    match std::env::var("LEDGER_FILE") {
        Ok(p) => format!("{}", p),
        Err(_) => format!("{}", ""),
    }
}

pub fn load() -> Result<ledger::Ledger> {
    let ledger_file_env = get_file_name();

    parse_ledger(&ledger_file_env.to_string())
}

pub fn parse_ledger(filename: &String) -> Result<ledger::Ledger> {
    let file_path = fs::read_to_string(filename);

    if let Err(err) = file_path {
        return Err(
            Error::new(
                ErrorKind::Io(err, Some(filename.to_string())
            ), None)
        );
    }

    Ok(toml::from_str(&file_path.unwrap()).unwrap())
}

pub fn write_to_ledger(bytes_to_write: &[u8]) -> Result<()> {
    let ledger_file = get_file_name();
    let mut ledger = OpenOptions::new()
        .append(true)
        .create(true)
        .open(ledger_file)
        .unwrap();

    ledger.write_all(b"\n\n")?;
    ledger.write_all(bytes_to_write)?;

    Ok(())
}

pub fn get_ledger_defaults() -> Result<default::Default> {
    let ledger_file = load()?;

    Ok(ledger_file.default.unwrap())
}