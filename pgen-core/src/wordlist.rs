use std::convert::TryFrom;

use rand::prelude::*;
use rand::rngs::ThreadRng;

use crate::error::PgenError;

const ALL_WORDS: &str = include_str!("../../wordlists/370k_words.txt");
const SHORT_WORDS: &str = include_str!("../../wordlists/short_words.txt");
const NUMBERS: &str = include_str!("../../wordlists/numbers.txt");
const SYMBOLS: &str = include_str!("../../wordlists/symbols.txt");
const TEST_NUMBER: &str = include_str!("../../wordlists/test_number.txt");
const TEST_WORD: &str = include_str!("../../wordlists/test_word.txt");

#[derive(Debug)]
pub struct Wordlist {
    rng: ThreadRng,
    list: Vec<&'static str>,
}

impl Wordlist {
    pub fn pick(&mut self) -> &'static str {
        self.list[self.rng.gen_range(0, self.list.len())]
    }
}

impl TryFrom<char> for Wordlist {
    type Error = PgenError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let list = match value {
            'N' => NUMBERS,
            'S' => SYMBOLS,
            'W' => ALL_WORDS,
            'w' => SHORT_WORDS,
            't' => TEST_WORD,
            'T' => TEST_NUMBER,
            c => return Err(PgenError::UnknownWordlist(c.to_string())),
        }
        .split('\n')
        .collect();

        let rng = thread_rng();

        Ok(Wordlist { rng, list })
    }
}
