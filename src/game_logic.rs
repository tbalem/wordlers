use std::collections::HashMap;

/// Checks for perfect characters in the guess word and updates the character counts.
/// Returns a vector of characters representing the results.
pub fn check_perfect_characters(
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
pub fn check_misplaced_characters(
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
