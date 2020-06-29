use crate::cli::*;

#[derive(Debug, StructOpt)]
pub struct RegisterOpt { }

pub fn eval(_cli: &Cli, _cmd: &RegisterOpt) -> Result<(), std::io::Error> {
    println!("{}", "Register");
    
    Ok(())
}