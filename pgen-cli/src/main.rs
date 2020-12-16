#![warn(clippy::all, clippy::pedantic)]
use structopt::StructOpt;

use pgen_core::pattern::Pattern;

/// A wordlist-based password generator.
#[derive(Debug, StructOpt)]
#[structopt(author = "Sean Newman <sean@snewman.co>")]
struct Arguments {
    /// The number of passwords to generate.
    ///
    /// If multiple passwords are to be generated, all passwords will use the same pattern. Each
    /// password will be printed on a separate line.
    #[structopt(short, long, default_value = "1")]
    count: usize,

    /// A string of characters describing how the password should be generated.
    ///
    /// Each of the characters in the pattern identifies a wordlist that should be used to generate
    /// part of the password. To generate a password, a random entry will be selected from each of
    /// the wordlists and concatenated together, in order.
    ///
    /// For example, if the pattern string for a password is "wSN", then the password will consist
    /// of a short word followed by a symbol, then a number.
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
