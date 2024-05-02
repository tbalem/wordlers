use colored::Colorize;
use core::fmt;
use std::error::Error;
use wordlers::game_logic::{analyze_guess, CharacterState};
use wordlers::user_input::{format_and_check::input_string, get_user_input_stdin};

const MAX_IOERROR_TRIES: usize = 5;

/// Represents the possible errors that can occur during a guess iteration.
#[derive(Debug)]
pub enum GuessIterationError {
    /// There have been too many `IOError` (defined by `MAX_IOERROR_TRIES`) while reading user input.
    TooManyIOErrorIteration,
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

#[derive(Clone)]
struct DisplayedCharacterState {
    character_state: CharacterState,
}

impl fmt::Display for DisplayedCharacterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.character_state {
            CharacterState::NotTried => write!(f, "-"),
            CharacterState::NotPresent(character) => write!(f, "{}", String::from(character).red()),
            CharacterState::Misplaced(character) => {
                write!(f, "{}", String::from(character).yellow())
            }
            CharacterState::Good(character) => write!(f, "{}", String::from(character).green()),
        }
    }
}

/// Gives at most `n_tries` to the user to guess the `guess_word`.
/// Also prints previous attempts' result.
///
/// # Returns
/// True if the user guessed the word, false otherwise.
#[must_use]
pub fn game_iteration(guess_word: &str, n_tries: usize) -> bool {
    let mut guess_tries: Vec<Vec<DisplayedCharacterState>> = (0..n_tries)
        .map(|_| {
            vec![
                DisplayedCharacterState {
                    character_state: CharacterState::NotTried
                };
                guess_word.len()
            ]
        })
        .collect();
    for i in 0..n_tries {
        match guess_iteration(guess_word) {
            Ok(result) => {
                guess_tries[i] = result;
                println!("Current tries:");
                for guess_try in &guess_tries {
                    for displayed_guess_character in guess_try {
                        print!("{displayed_guess_character}");
                    }
                    println!();
                }
                if guess_tries[i].iter().all(|c| {
                    std::mem::discriminant(&c.character_state)
                        == std::mem::discriminant(&CharacterState::Good(' '))
                }) {
                    return true;
                }
            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
    false
}

/// Takes user input, checks for perfect and misplaced characters.
/// # Returns
/// The result char array produced by `check_perfect_characters` and `check_misplaced_characters`.
///
/// # Errors
/// Throw `GuessIterationError::TooManyIOErrorIteration` if there where more then `MAX_IOERROR_TRIES` tries
/// with error while reading user's input.
fn guess_iteration(guess_word: &str) -> Result<Vec<DisplayedCharacterState>, Box<dyn Error>> {
    let mut nb_incorrect_tries = 0;
    while nb_incorrect_tries < MAX_IOERROR_TRIES {
        println!("Please input a new guess:");
        match get_user_input_stdin() {
            Ok(user_input_str) => match input_string(&user_input_str, guess_word.len()) {
                Ok(trimmed_uppercased_input) => {
                    return Ok(analyze_guess(guess_word, &trimmed_uppercased_input)
                        .into_iter()
                        .map(|character_state| DisplayedCharacterState {
                            character_state: character_state.clone(),
                        })
                        .collect());
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
