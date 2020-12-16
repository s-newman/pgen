//! Creating and using patterns that describe password generation.

use std::convert::TryFrom;
use std::str::FromStr;

use crate::wordlist::Wordlist;
use crate::PgenError;

/// A description of how a password should be generated.
#[derive(Debug)]
pub struct Pattern(Vec<Wordlist>);

impl Pattern {
    /// Generate a new password based on the pattern.
    ///
    /// This generator will iterate through each wordlist referenced by the pattern and select a
    /// random entry from each one. The random selections will then be concatenated together in
    /// order.
    ///
    /// # Examples
    ///
    /// Generating a single password using the test-only wordlists.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use pgen_core::pattern::Pattern;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut pat = Pattern::from_str("tTT")?;
    /// let password: String = pat.generate();
    /// assert_eq!(&password, "password11");
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate(&mut self) -> String {
        let mut password = String::default();

        for list in &mut self.0 {
            password.push_str(list.pick());
        }

        password
    }
}

impl FromStr for Pattern {
    type Err = PgenError;

    /// Create a new pattern using a string representation.
    ///
    /// Pattern strings are sequences of characters that define which wordlists to use, and in what
    /// order. Each character in the string refers to a specific wordlist. Wordlists can be used
    /// multiple times in a pattern string.
    ///
    /// Note that characters in pattern strings are case-sensitive. The pattern string `WWW` will
    /// use different wordlists than `www` will.
    ///
    /// # Examples
    ///
    /// This example creates a pattern that will generate a password containing one short word
    /// followed by a symbol and then a number.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use pgen_core::pattern::Pattern;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut pat = Pattern::from_str("wSN")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// An error will be returned if you try to parse a string containing a character that doesn't
    /// match any known wordlists.
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use pgen_core::pattern::Pattern;
    /// let mut pat = Pattern::from_str("!?!?");
    /// assert!(pat.is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lists = Vec::new();

        for chr in s.chars() {
            lists.push(Wordlist::try_from(chr)?);
        }

        Ok(Pattern(lists))
    }
}
