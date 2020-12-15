#![warn(clippy::all, clippy::pedantic)]
use rand::prelude::*;
use structopt::StructOpt;

const WORDS: &str = include_str!("../wordlists/370k_words.txt");
const NUMBERS: &str = include_str!("../wordlists/numbers.txt");
const SYMBOLS: &str = include_str!("../wordlists/symbols.txt");

#[derive(Debug, StructOpt)]
#[structopt(name = "pgen", about = "A wordlist-based password generator.")]
struct Arguments {
    #[structopt(short, long, default_value = "1")]
    count: usize,
}

fn main() {
    // Parse arguments
    let args = Arguments::from_args();

    // Parse wordlists
    let words: Vec<&str> = WORDS.split('\n').collect();
    let numbers: Vec<&str> = NUMBERS.split('\n').collect();
    let symbols: Vec<&str> = SYMBOLS.split('\n').collect();

    let mut rng = rand::thread_rng();

    for _ in 0..args.count {
        let mut pw = String::default();

        pw.push_str(words[rng.gen_range(0, words.len())]);
        pw.push_str(numbers[rng.gen_range(0, numbers.len())]);
        pw.push_str(symbols[rng.gen_range(0, numbers.len())]);
        pw.push_str(words[rng.gen_range(0, words.len())]);
        pw.push_str(symbols[rng.gen_range(0, numbers.len())]);
        pw.push_str(words[rng.gen_range(0, words.len())]);
        pw.push_str(numbers[rng.gen_range(0, numbers.len())]);
        pw.push_str(words[rng.gen_range(0, words.len())]);

        println!("{}", pw);
    }
}
