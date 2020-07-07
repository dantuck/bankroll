mod cli;
mod util;
mod model;

fn main() -> Result<(), std::io::Error> {
    cli::run()
}