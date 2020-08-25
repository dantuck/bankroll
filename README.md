# Bankroll [![Latest Version]][crates.io] [![Conduct svg]][Code of Conduct]

[Latest Version]: https://img.shields.io/crates/v/bankroll.svg
[crates.io]: https://crates.io/crates/bankroll
[Conduct svg]: code-of-conduct.svg
[Code of Conduct]: CODE_OF_CONDUCT.md

<a href="https://www.buymeacoffee.com/dantuck" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" style="height: 51px !important;width: 217px !important;" ></a>

Ledger implementation in Rust. It takes concepts from [ledger-cli](https://www.ledger-cli.org/) but with a [TOML](https://toml.io) file type.

### Install

#### From Cargo 

`cargo install bankroll`

#### Build from Source

Alternatively, clone this repo and do the following:

- If Rust is not installed on your machine, follow the instructions on how to do that here: https://www.rust-lang.org/tools/install
- run `cargo build --release` to compile the binary
- go to `/target/release` and copy the `bankroll` binary in your path: `/usr/bin`

## Usage

### Environment variable

LEDGER_FILE - relative path to toml ledger file

```
LEDGER_FILE=~/ledger.toml bankroll balance
```

`LEDGER_FILE` can be set as a system or user environment variable.

```
export LEDGER_FILE="$HOME/ledger.toml"
```

### Example
```
$ ./bankroll --help
bankroll 0.2.0

USAGE:
    bankroll <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -r, --real       Only shows real transactions
    -V, --version    Prints version information

SUBCOMMANDS:
    balance     
    help        Prints this message or the help of the given subcommand(s)
    import      
    register    
$ ./bankroll balance

    $ 11,940.00 Assets
     $ 2,970.00  Checking
     $ 7,990.00  Savings
       $ 980.00   Fund:Auto
   $ -13,000.00 Equity
   $ -13,000.00  Opening Balance
     $ 1,075.00 Expenses
        $ 15.00  Entertainment
     $ 1,060.00  General
       $ -15.00 Liabilities
       $ -15.00  Credit
─────────────── 
              0

$ ./bankroll balance

2020-01-01 income                         Assets:Checking           $ 3,000.00      $ 3,000.00
                                          Assets:Savings           $ 10,000.00     $ 13,000.00
                                          Equi:Opening Balance    $ -13,000.00               0
2020-01-01 Sample really long that…       Expenses:General             $ 10.00         $ 10.00
                                          Expenses:General             $ 10.00         $ 20.00
                                          Assets:Savings              $ -10.00         $ 10.00
                                          Assets:Checking             $ -10.00               0
2020-01-01 Sample transaction             Expenses:General             $ 10.00         $ 10.00
                                          Expenses:General             $ 10.00         $ 20.00
                                          Assets:Checking             $ -20.00               0
2020-01-01 Movie night                    Expens:Entertainment         $ 15.00         $ 15.00
                                          Liabilities:Credit          $ -15.00               0
2020-01-01 Really big purchase            Expenses:General          $ 1,000.00      $ 1,000.00
                                          Assets:Savings           $ -1,000.00               0
2020-08-01 Car Repair                     Expenses:General             $ 20.00         $ 20.00
                                          Assets:Savings              $ -20.00               0
                                          (Fund:Auto)                 $ -20.00                
                                          
$ ./bankroll import example/sample.csv

"example/sample.csv" imported Successfully

```

## License

Licensed under GNU General Public License, Version 3, 29 June 2007 ([LICENSE-GNU](LICENSE) or <https://www.gnu.org/licenses/gpl.html>)

### Contribution
