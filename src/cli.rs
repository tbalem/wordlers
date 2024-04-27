use std::collections::HashMap;
use wordlers::game_logic::{check_misplaced_characters, check_perfect_characters};
use wordlers::input::{format_and_check::*, get_user_input_stdin};

/// The main game loop that takes user input, checks for perfect and misplaced characters,
/// and prints the results.
pub fn cli_game_loop(guess_word: &str) {
    let char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut edited_char_counts = char_counts.clone();

    println!("Please input a new guess:");
    match get_user_input_stdin() {
        Ok(user_input_str) => {
            match format_and_check_input_string(user_input_str, guess_word.len()) {
                Ok(trimmed_uppercased_input) => {
                    let results_with_perfect_characters = check_perfect_characters(
                        guess_word,
                        &trimmed_uppercased_input,
                        &mut edited_char_counts,
                    );

                    let results = check_misplaced_characters(
                        guess_word,
                        &trimmed_uppercased_input,
                        edited_char_counts,
                        results_with_perfect_characters,
                    );

                    println!("{}", results.into_iter().collect::<String>());
                }
                Err(UserInputError::UnexpectedLength) => {
                    println!("Your guess must be exactly 5 characters.");
                }
                Err(UserInputError::NotAlphabetic) => {
                    println!("Some of your input is not an alphabetic character.");
                }
            }
        }
        Err(io_error) => eprintln!("Error while reading user input: {}", io_error),
    }
}
