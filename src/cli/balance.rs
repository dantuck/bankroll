use crate::cli::*;

#[derive(Debug, StructOpt)]
pub struct BalanceOpt { }

pub fn eval(_cli: &Cli, _cmd: &BalanceOpt) -> Result<(), std::io::Error> {
    println!("{}", "Balance");
    
    Ok(())
}