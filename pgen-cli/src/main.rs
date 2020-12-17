//! A command-line password generator.

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,

    // Restriction lints
    clippy::as_conversions,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::else_if_without_else,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wrong_pub_self_convention,
)]

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
