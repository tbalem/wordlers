use std::collections::HashMap;
use std::io::{self, Write};

/// Represents the possible errors that can occur during user input.
enum UserInputError {
    IoError(io::Error), // Error while performing I/O operations.
    UnexpectedLength,   // The input does not correspond to the defined length.
    NotAlphabetic,      // The input contains non-alphabetic characters.
}

/// Reads a line of user input from the standard input and performs validation.
/// Returns the trimmed and uppercased input if it is valid, or an error otherwise.
fn get_user_input(expected_length: usize) -> Result<String, UserInputError> {
    let mut input = String::new();
    println!("Please input a new guess:");
    io::stdout().flush().map_err(UserInputError::IoError)?;

    io::stdin()
        .read_line(&mut input)
        .map_err(UserInputError::IoError)?; // Error while reading user input.

    let trimmed_uppercased_input = input.trim();

    if trimmed_uppercased_input.chars().count() != expected_length {
        Err(UserInputError::UnexpectedLength) // Your guess must be exactly 5 characters.
    } else if !trimmed_uppercased_input
        .chars()
        .all(|c| c.is_ascii_alphabetic())
    {
        Err(UserInputError::NotAlphabetic) // Some of your input is not an alphabetic character.
    } else {
        Ok(trimmed_uppercased_input.to_ascii_uppercase())
    }
}

/// Checks for perfect characters in the guess word and updates the character counts.
/// Returns a vector of characters representing the results.
fn check_perfect_characters(
    guess_word: &String,
    guess_string: &String,
    char_counts: &mut HashMap<char, i32>,
) -> Vec<char> {
    let mut results = vec!['-'; guess_word.len()];

    for (i, (char1, char2)) in guess_word.chars().zip(guess_string.chars()).enumerate() {
        if char1 == char2 {
            results[i] = 'X';
            if let Some(char_count) = char_counts.get_mut(&char1) {
                *char_count -= 1;
            } else {
                eprintln!("Error reading char {i}: {char1} in char_counts HashMap");
                continue;
            }
        }
    }

    results
}

/// Checks for misplaced characters in the guess word and updates the character counts.
/// Returns a vector of characters representing the results.
fn check_misplaced_characters(
    guess_word: &String,
    guess_string: &String,
    mut char_counts: HashMap<char, i32>,
    mut results: Vec<char>,
) -> Vec<char> {
    for (i, result_char_i) in results.iter_mut().enumerate() {
        if *result_char_i != 'X' {
            if let Some(input_char) = guess_string.chars().nth(i) {
                if guess_word.contains(input_char) {
                    if let Some(char_count) = char_counts.get_mut(&input_char) {
                        if *char_count > 0 {
                            *result_char_i = 'O';
                            *char_count -= 1;
                        }
                    }
                }
            } else {
                eprintln!("Error reading char {i} in trimmed_uppercased_input");
            }
        }
    }

    results
}

/// The main game loop that takes user input, checks for perfect and misplaced characters,
/// and prints the results.
fn game_loop(guess_word: &String) {
    let char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut edited_char_counts = char_counts.clone();

    match get_user_input(guess_word.len()) {
        Ok(trimmed_uppercased_input) => {
            let results_with_perfect_characters = check_perfect_characters(
                &guess_word,
                &trimmed_uppercased_input,
                &mut edited_char_counts,
            );

            let results = check_misplaced_characters(
                &guess_word,
                &trimmed_uppercased_input,
                edited_char_counts,
                results_with_perfect_characters,
            );

            println!("{}", results.into_iter().collect::<String>());
        }
        Err(UserInputError::IoError(err)) => {
            eprintln!("Error while reading input: {}", err);
        }
        Err(UserInputError::UnexpectedLength) => {
            println!("Your guess must be exactly 5 characters.");
        }
        Err(UserInputError::NotAlphabetic) => {
            println!("Some of your input is not an alphabetic character.");
        }
    }
}

fn main() {
    let guess_word = String::from("TESTS");

    loop {
        game_loop(&guess_word);
    }
}
