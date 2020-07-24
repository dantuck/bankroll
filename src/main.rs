mod cli;
mod util;
mod error;
mod model;

fn main() -> Result<(), std::io::Error> {
    cli::run()
}