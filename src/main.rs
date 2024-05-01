pub mod cli;
pub mod data_loader;

use cli::game_iteration;
use data_loader::{choose_random_word, load_words_file};

const WORDS_FILE: &str = "./data/words.txt";

fn main() {
    println!("Loading words.");
    let all_words = load_words_file(WORDS_FILE).unwrap();
    println!("Choosing random word.");
    let guess_word = choose_random_word(&all_words, 5).unwrap();

    if game_iteration(&guess_word, 5) {
        println!("Congratulations, you won!");
    } else {
        println!("You lost, the word was {guess_word}.");
    }
}
