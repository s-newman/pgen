//! Implementation of the core pgen functionality.
//!
//! This library is written to provide simple APIs for end-user applications to use.
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
// Turn off some pedantic warnings
#![allow(clippy::must_use_candidate)]

pub mod pattern;
pub mod wordlist;

use thiserror::Error;

/// All possible errors that could be returned from pgen-core.
#[derive(Debug, Error)]
pub enum PgenError {
    /// Represents a failure to parse part of a pattern string due to an unknown pattern character.
    #[error("Pattern character `{0}` does not match any known wordlists")]
    UnknownWordlist(String),

    /// Represents a failure to convert a number of combinations to bits of entropy.
    #[error("Could not determine the bits of entropy for `{0}` combinations")]
    BitsOfEntropyConversion(usize),
}
