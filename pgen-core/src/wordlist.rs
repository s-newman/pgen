//! A set of wordlists that are included with the binary, and ergonomic APIs to work with them.

use std::convert::TryFrom;

use rand::prelude::*;
use rand::rngs::ThreadRng;

use crate::PgenError;

/// A long wordlist containing many words in the english language, including long and rare words.
const ALL_WORDS: &str = include_str!("../../wordlists/370k_words.txt");

/// A subset of the [`ALL_WORDS`] list containing only words up to 7 characters long.
const SHORT_WORDS: &str = include_str!("../../wordlists/short_words.txt");

/// A wordlist containing the numbers 0-9.
const NUMBERS: &str = include_str!("../../wordlists/numbers.txt");

/// A wordlist containing most of the symbols that can be typed on a typical US English keyboard.
const SYMBOLS: &str = include_str!("../../wordlists/symbols.txt");

/// A wordlist containing only `password`. Used for some tests that require known output from
/// [`Pattern::generate`](crate::pattern::Pattern::generate).
const TEST_NUMBER: &str = include_str!("../../wordlists/test_number.txt");

/// A wordlist containing only `1`. Used for the same purpose as [`TEST_NUMBER`].
const TEST_WORD: &str = include_str!("../../wordlists/test_word.txt");

/// A wrapper around a wordlist included with the library.
///
/// Each instance of a wordlist refers to (instead of copying) a wordlist constant, so multiple
/// instances of the same wordlist can be created without ballooning the memory usage much.
#[derive(Debug)]
pub struct Wordlist {
    /// A random number generator to use when generating a password with this wordlist.
    rng: ThreadRng,

    /// The list of words in the wordlist.
    list: Vec<&'static str>,
}

impl Wordlist {
    /// Select a random word from the wordlist.
    pub fn pick(&mut self) -> &'static str {
        self.list[self.rng.gen_range(0, self.list.len())]
    }

    /// Return the number of words available in the wordlist.
    pub fn words(&self) -> usize {
        self.list.len()
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
        .filter(|elem| !elem.trim().is_empty())
        .collect();

        let rng = thread_rng();

        Ok(Wordlist { rng, list })
    }
}
