use std::{collections::HashMap, hash::BuildHasher};

/// Represents the state of a character in the guess word.
#[derive(Clone, PartialEq)]
pub enum CharacterState {
    /// The character has not been tried yet.
    NotTried,
    /// The character is not in the word.
    NotPresent(char),
    /// The character is in the word but not at this place.
    Misplaced(char),
    /// The character is correctly placed.
    Good(char),
}

/// Checks for perfect characters in the guess word and updates the character counts.
///
/// This function compares each character in the `guess_word` with the corresponding character in the `guess_string`:
/// If a character in the `guess_word` matches the character in the `guess_string`, it is marked as `CharacterState::Good` and stored in the `results` vector.
/// The character count for the matched character is then decremented in the `char_counts` `HashMap`.
///
/// # Arguments
///
/// * `guess_word` - The word to be guessed.
/// * `guess_string` - The user's guess.
/// * `char_counts` - A mutable reference to a `HashMap` that stores the character counts.
///
/// # Returns
///
/// A vector of `CharacterState` representing the results.
fn mark_perfect_characters<S: BuildHasher>(
    guess_word: &str,
    guess_string: &str,
    char_counts: &mut HashMap<char, i32, S>,
) -> Vec<CharacterState> {
    let mut results = vec![CharacterState::NotTried; guess_word.len()];

    for (i, (char1, char2)) in guess_word.chars().zip(guess_string.chars()).enumerate() {
        if char1 == char2 {
            results[i] = CharacterState::Good(char1);
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
///
/// This function iterates over each character in the `results` vector.
/// If a character is not already marked as `CharacterState::Good`, it checks if the corresponding character in the `guess_string` is present in the `guess_word`.
/// If it is present and the character count for that character is greater than 0, it marks the character as `CharacterState::Misplaced` in the `results` vector.
/// The character count for the matched character is then decremented in the `char_counts` `HashMap`.
///
/// # Arguments
///
/// * `guess_word` - The word to be guessed.
/// * `guess_string` - The user's guess.
/// * `char_counts` - A `HashMap` that stores the character counts.
/// * `results` - A vector representing the current status of the characters in the `guess_word`.
///
/// # Returns
///
/// A vector of `CharacterState` representing the results:
/// - if a character is perfectly placed, it is represented by `CharacterState::Good`.
/// - if a character is misplaced, it is represented by `CharacterState::Misplaced`.
/// - otherwise, it is represented by `CharacterState::NotPresent`.
fn mark_misplaced_characters<S: BuildHasher>(
    guess_word: &str,
    guess_string: &str,
    mut char_counts: HashMap<char, i32, S>,
    mut results: Vec<CharacterState>,
) -> Vec<CharacterState> {
    for (i, result_char_i) in results.iter_mut().enumerate() {
        if std::mem::discriminant(result_char_i)
            != std::mem::discriminant(&CharacterState::Good(' '))
        {
            if let Some(input_char) = guess_string.chars().nth(i) {
                if guess_word.contains(input_char) {
                    if let Some(char_count) = char_counts.get_mut(&input_char) {
                        if *char_count > 0 {
                            *result_char_i = CharacterState::Misplaced(input_char);
                            *char_count -= 1;
                        } else {
                            *result_char_i = CharacterState::NotPresent(input_char);
                        }
                    }
                } else {
                    *result_char_i = CharacterState::NotPresent(input_char);
                }
            } else {
                eprintln!("Error reading char {i} in trimmed_uppercased_input");
            }
        }
    }

    results
}

/// Analyzes the user's guess and returns the results.
///
/// This function takes the `guess_word` and the preprocessed user's guess (`preprocessed_try`) as input.
/// By preprocessed, it is meant to be trimmed.
/// It initializes a `HashMap` (`char_counts`) to store the character counts in the `guess_word`.
/// The `mark_perfect_characters` function is called to check for perfect characters and update the character counts.
/// The `mark_misplaced_characters` function is then called to check for misplaced characters and update the character counts.
///
/// # Arguments
///
/// * `guess_word` - The word to be guessed.
/// * `preprocessed_try` - The preprocessed user's guess.
///
/// # Returns
///
/// A vector of characters representing the results. Each character is marked as:
/// - `CharacterState::Good` if the character is perfectly placed.
/// - `CharacterState::Misplaced` if the character is misplaced.
/// - `CharacterState::NotPresent` if the character is not present in the `guess_word`.
/// - `CharacterState::NotTried` if the character has not been tried yet. There should be no `CharacterState::NotTried` characters in the final result.
#[must_use]
pub fn analyze_guess(guess_word: &str, preprocessed_try: &str) -> Vec<CharacterState> {
    let mut char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    // Temporary result with perfectly placed characters
    let results = mark_perfect_characters(guess_word, preprocessed_try, &mut char_counts);

    mark_misplaced_characters(guess_word, preprocessed_try, char_counts, results)
}
