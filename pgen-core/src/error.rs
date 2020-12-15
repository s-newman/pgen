use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgenError {
    #[error("No wordlist found for `{0}` wordlist specifier")]
    UnknownWordlist(String),
}
