mod balance;
mod register;

use balance::*;
use register::*;

pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bankroll")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Opt,
}

#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "balance", aliases = &["b", "bal"])]
    Balance(BalanceOpt),
    #[structopt(name = "register", aliases = &["r", "reg"])]
    Register(RegisterOpt),
}

pub fn run() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();
    
    match &cli.cmd {
        Opt::Balance(opt) => balance::eval(&cli, &opt)?,
        Opt::Register(opt) => register::eval(&cli, &opt)?,
    }

    Ok(())
}