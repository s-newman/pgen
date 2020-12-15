#![warn(clippy::all, clippy::pedantic)]
use structopt::StructOpt;

use pgen_core::pattern::Pattern;

#[derive(Debug, StructOpt)]
#[structopt(name = "pgen", about = "A wordlist-based password generator.")]
struct Arguments {
    #[structopt(short, long, default_value = "1")]
    count: usize,

    #[structopt(short, long, default_value = "wNwSWNSw")]
    pattern: Pattern,
}

fn main() {
    // Parse arguments
    let mut args = Arguments::from_args();

    for _ in 0..args.count {
        println!("{}", args.pattern.generate());
    }
}
