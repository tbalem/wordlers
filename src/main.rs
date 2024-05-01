/// This module contains the command-line interface (CLI) functionality.
pub mod cli;

/// This module handles loading data.
pub mod data_loader;

use clap::Parser;
use cli::game_iteration;
use data_loader::{choose_random_word, load_words_file};

/// A word guessing game.
///
/// The program loads a list of words from a file and chooses a random word of a specified length.
/// If the user guesses the word correctly, they win. Otherwise, they lose.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the file containing the list of words.
    #[arg(long, short = 'f')]
    words_file: String,
    /// Length of the guess.
    #[arg(long, short = 'n', default_value_t = 5)]
    guess_length: usize,
}

fn main() {
    let args = Args::parse();

    println!("Loading words.");
    let all_words = load_words_file(&args.words_file).unwrap();
    println!("Choosing random word.");
    let guess_word = choose_random_word(&all_words, args.guess_length).unwrap();

    if game_iteration(&guess_word, 5) {
        println!("Congratulations, the word was {guess_word}, you won!");
    } else {
        println!("You lost, the word was {guess_word}.");
    }
}
