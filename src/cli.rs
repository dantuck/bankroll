mod balance;
mod register;
mod import;

use balance::*;
use register::*;
use import::*;

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
    #[structopt(name = "import", aliases = &["i"])]
    Import(ImportOpt),
}

pub fn run() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();
    
    if let Err(err) = 
        match &cli.cmd {
            Opt::Balance(opt) => balance::eval(&cli, &opt),
            Opt::Register(opt) => register::eval(&cli, &opt),
            Opt::Import(opt) => import::eval(&cli, &opt),
        } {
            println!("\n{}\n", err.to_string())
        }

    Ok(())
}