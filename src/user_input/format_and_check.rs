use core::fmt;
use std::error::Error;

/// Represents the possible errors that can occur during user input.
#[derive(Debug)]
pub enum UserInputError {
    /// The input does not correspond to the defined length. (expected length, given length)
    UnexpectedLength(usize, usize),
    /// The input contains non-alphabetic characters. (not-alphabetic characters)
    NotAlphabetic(Vec<char>),
}

impl Error for UserInputError {}

impl fmt::Display for UserInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserInputError::UnexpectedLength(expected, given) => {
                write!(f, "Expected a string of {expected} characters, got {given}")
            }
            UserInputError::NotAlphabetic(not_alphabetic_characters) => write!(
                f,
                "Some of given input is not an alphabetic character: {not_alphabetic_characters:?}"
            ),
        }
    }
}

/// Formats and checks the input string for validity.
/// Returns the formatted string if it is valid, or an error otherwise.
/// # Errors
/// Returns an error if the input string is not the expected length or contains non-alphabetic characters.
pub fn input_string(input_str: &str, expected_length: usize) -> Result<String, UserInputError> {
    let trimmed_input_str = input_str.trim();
    let trimmed_input_length = trimmed_input_str.chars().count();

    let not_alphabetic_characters = trimmed_input_str
        .chars()
        .filter(|c| !c.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    if trimmed_input_length != expected_length {
        // Your guess must be exactly `expected_length` characters
        Err(UserInputError::UnexpectedLength(
            expected_length,
            trimmed_input_length,
        ))
    } else if !not_alphabetic_characters.is_empty() {
        Err(UserInputError::NotAlphabetic(not_alphabetic_characters)) // Some of your input is not an alphabetic character.
    } else {
        Ok(trimmed_input_str.to_ascii_uppercase())
    }
}
