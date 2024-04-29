use std::{collections::HashMap, hash::BuildHasher};

/// Checks for perfect characters in the guess word and updates the character counts.
///
/// # Returns
/// A vector of characters representing the results.
fn mark_perfect_characters<S: BuildHasher>(
    guess_word: &str,
    guess_string: &str,
    char_counts: &mut HashMap<char, i32, S>,
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
///
/// # Returns
/// A vector of characters representing the results.
fn mark_misplaced_characters<S: BuildHasher>(
    guess_word: &str,
    guess_string: &str,
    mut char_counts: HashMap<char, i32, S>,
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

#[must_use]
pub fn analyze_guess(guess_word: &str, preprocessed_try: &str) -> Vec<char> {
    let mut char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    // Temporary result with perfectly placed characters
    let results = mark_perfect_characters(guess_word, preprocessed_try, &mut char_counts);

    mark_misplaced_characters(guess_word, preprocessed_try, char_counts, results)
}
