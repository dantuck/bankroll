mod cli;
mod error;
mod model;
mod util;

fn main() -> Result<(), std::io::Error> {
    cli::run()
}
