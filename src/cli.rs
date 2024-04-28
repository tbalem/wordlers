use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use wordlers::game_logic::{check_misplaced_characters, check_perfect_characters};
use wordlers::input::{format_and_check::input_string, get_user_input_stdin};

const MAX_IOERROR_TRIES: usize = 5;

/// Represents the possible errors that can occur during a guess iteration.
#[derive(Debug)]
pub enum GuessIterationError {
    TooManyIOErrorIteration, // There have been
}

impl Error for GuessIterationError {}

impl fmt::Display for GuessIterationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuessIterationError::TooManyIOErrorIteration => {
                write!(f, "More than {MAX_IOERROR_TRIES} occurred.")
            }
        }
    }
}

/// Takes user input, checks for perfect and misplaced characters.
/// # Returns
/// The result char array produced by `check_perfect_characters` and `check_misplaced_characters`.
///
/// # Errors
/// Throw `GuessIterationError::TooManyIOErrorIteration` if there where more then `MAX_IOERROR_TRIES` tries
/// with error while reading user's input.
pub fn game_loop(guess_word: &str) -> Result<Vec<char>, Box<dyn Error>> {
    let char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut edited_char_counts = char_counts.clone();

    let mut nb_incorrect_tries = 0;
    while nb_incorrect_tries < MAX_IOERROR_TRIES {
        println!("Please input a new guess:");
        match get_user_input_stdin() {
            Ok(user_input_str) => match input_string(&user_input_str, guess_word.len()) {
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

                    return Ok(results);
                }
                Err(err) => eprintln!("{err}"),
            },
            Err(err) => {
                eprintln!("Error while reading user input: {err}");
                nb_incorrect_tries += 1;
            }
        }
    }

    Err(Box::new(GuessIterationError::TooManyIOErrorIteration))
}
