use std::convert::TryFrom;
use std::str::FromStr;

use crate::error::PgenError;

#[derive(Debug)]
pub enum Wordlist {
    Numbers,
    Symbols,
    AllWords,
    ShortWords,
}

impl TryFrom<char> for Wordlist {
    type Error = PgenError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(Wordlist::Numbers),
            'S' => Ok(Wordlist::Symbols),
            'W' => Ok(Wordlist::AllWords),
            'w' => Ok(Wordlist::ShortWords),
            c => Err(PgenError::UnknownWordlist(c.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Pattern(pub Vec<Wordlist>);

impl FromStr for Pattern {
    type Err = PgenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lists: Vec<Wordlist> = Vec::new();

        for chr in s.chars() {
            lists.push(Wordlist::try_from(chr)?);
        }

        Ok(Pattern(lists))
    }
}
