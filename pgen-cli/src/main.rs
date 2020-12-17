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

    /// Optional subcommands that can be run.
    ///
    /// The default behavior if no subcommands are passed is to run the password generation routine.
    /// If a subcommand is passed, then the subcommand will be run instead of generating a password.
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

/// The available optional subcommands.
#[derive(Debug, StructOpt)]
enum Command {
    /// Calculate and display the number of bits of entropy in the given password pattern string.
    ///
    /// Bits of entropy can be useful when comparing the strength of a password to the security of a
    /// symmetrical key. For example, a password with 128 bits of entropy should be about as
    /// difficult to guess as an AES-128 key.
    ///
    /// The recommended amount of entropy depends on your use case and threat model. More entropy is
    /// recommended for passwords that will be in use for longer, are used to secure privileged
    /// access or sensitive information, or are hashed with relatively weak hashing algorithms. On
    /// the other hand, high-entropy passwords can be difficult to remember even when generated as
    /// "passphrases".
    Entropy,
}

fn main() {
    // Parse arguments
    let mut args = Arguments::from_args();

    // Run the correct routine
    if let Some(cmd) = args.cmd {
        match cmd {
            Command::Entropy => {
                println!(
                    "This pattern has {} bits of entropy.",
                    args.pattern.entropy_bits()
                );
            }
        }
    } else {
        for _ in 0..args.count {
            println!("{}", args.pattern.generate());
        }
    }
}
