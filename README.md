# Bankroll [![Latest Version]][crates.io] [![Docs]][docs.rs] [![Conduct svg]][Code of Conduct]


[Latest Version]: https://img.shields.io/crates/v/bankroll.svg
[crates.io]: https://crates.io/crates/bankroll
[Docs]: https://docs.rs/bankroll/badge.svg
[docs.rs]: https://docs.rs/bankroll
[Conduct svg]: code-of-conduct.svg
[Code of Conduct]: CODE_OF_CONDUCT.md

Ledger implementation in Rust. It takes concepts from [ledger-cli](https://www.ledger-cli.org/) but with a [TOML](https://toml.io) file type.

## Documentation

Find it on [Docs.rs](https://docs.rs/bankroll)

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
bankroll 0.0.1

USAGE:
    bankroll <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    balance     
    help        Prints this message or the help of the given subcommand(s)
    import      
    register    
$ ./bankroll balance

      $ 2970.00 Assets:checking
      $ 9990.00 Assets:savings
    $ -13000.00 Equity:opening balance
        $ 15.00 Expenses:entertainment
        $ 40.00 Expenses:general
       $ -15.00 Liabilities:credit
───────────────
              0

$ ./bankroll balance

2020-01-01 income                         Assets:checking            $ 3000.00       $ 3000.00
                                          Assets:savings            $ 10000.00      $ 13000.00
                                          Equi:opening balance     $ -13000.00               0
2020-01-01 Sample really long that…       Expenses:general             $ 10.00         $ 10.00
                                          Expenses:general             $ 10.00         $ 20.00
                                          Assets:savings              $ -10.00         $ 10.00
                                          Assets:checking             $ -10.00               0
2020-01-01 Sample transaction             Expenses:general             $ 10.00         $ 10.00
                                          Expenses:general             $ 10.00         $ 20.00
                                          Assets:checking             $ -20.00               0
2020-01-01 Movie night                    Expens:entertainment         $ 15.00         $ 15.00
                                          Liabilities:credit          $ -15.00               0

$ ./bankroll import example/sample.csv

"example/sample.csv" imported Successfully

```

## License

Licensed under GNU General Public License, Version 3, 29 June 2007 ([LICENSE-GNU](LICENSE) or <https://www.gnu.org/licenses/gpl.html>)

### Contribution
