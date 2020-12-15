use std::convert::TryFrom;
use std::str::FromStr;

use crate::error::PgenError;
use crate::wordlist::Wordlist;

#[derive(Debug)]
pub struct Pattern(Vec<Wordlist>);

impl Pattern {
    pub fn generate(&mut self) -> String {
        let mut password = String::default();

        for list in self.0.iter_mut() {
            password.push_str(list.pick());
        }

        password
    }
}

impl FromStr for Pattern {
    type Err = PgenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lists = Vec::new();

        for chr in s.chars() {
            lists.push(Wordlist::try_from(chr)?);
        }

        Ok(Pattern(lists))
    }
}
