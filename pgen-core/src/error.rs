use thiserror::Error;

/// All possible errors that could be returned from pgen-core.
#[derive(Debug, Error)]
pub enum PgenError {
    /// Represents a failure to parse part of a pattern string due to an unknown pattern character.
    #[error("Pattern character `{0}` does not match any known wordlists")]
    UnknownWordlist(String),
}
