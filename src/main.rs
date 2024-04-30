use clap::Parser;
use dotenv::dotenv;

use crate::args::Args;
use crate::cli::Cli;

mod args;
mod ast;
mod cli;
mod error;
mod friggen;
mod ioutil;
mod logging;

mod friggenfile;
mod fs_context;
mod parser;
mod print;
mod shell;

fn main() {
    dotenv().ok();
    logging::init();

    let args = Args::parse();
    let cli = Cli::new(args);
    cli.run();
}
