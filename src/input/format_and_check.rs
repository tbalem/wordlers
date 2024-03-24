/// Represents the possible errors that can occur during user input.
pub enum UserInputError {
    UnexpectedLength, // The input does not correspond to the defined length.
    NotAlphabetic,    // The input contains non-alphabetic characters.
}

/// Formats and checks the input string for validity.
/// Returns the formatted string if it is valid, or an error otherwise.
pub fn format_and_check_input_string(
    input_str: String,
    expected_length: usize,
) -> Result<String, UserInputError> {
    let trimmed_input_str = input_str.trim();

    if trimmed_input_str.chars().count() != expected_length {
        Err(UserInputError::UnexpectedLength) // Your guess must be exactly 5 characters.
    } else if !trimmed_input_str.chars().all(|c| c.is_ascii_alphabetic()) {
        Err(UserInputError::NotAlphabetic) // Some of your input is not an alphabetic character.
    } else {
        Ok(trimmed_input_str.to_ascii_uppercase())
    }
}
