use crate::cli::*;

use crate::model::bank;
use crate::error::Result;

#[derive(Debug, StructOpt)]
pub struct ImportOpt {
    #[structopt(name = "file", aliases = &["f"])]
    file: String,

    #[structopt(name = "for-account", aliases = &["a"])]
    for_account: Option<String>,
}

#[derive(Debug)]
pub struct Import { }

pub fn eval(_cli: &Cli, cmd: &ImportOpt) -> Result<()> {
    println!();

    if let Err(err) = bank::import(&cmd.file, &cmd.for_account) {
        println!("Oops, unable to write to \"{}\" [{}]", cmd.file, err);
    } else {
        println!("\"{}\" {}", cmd.file, "imported Successfully")
    }

    println!();

    Ok(())
}