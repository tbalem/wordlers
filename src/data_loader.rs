use core::fmt;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::BuildHasher;

/// Custom error type for data loading errors.
#[derive(Debug)]
pub enum Error {
    /// Error indicating that a file contains non-alphabetic characters.
    NotAlphabetic(String),
    /// Error indicating that no word of a specific length was found.
    NoWordThisLength(usize),
    /// Error indicating that a vector of words is empty for a specific length.
    EmptyWordVec(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAlphabetic(path) => {
                write!(f, "File {path} contains non-alphabetic characters.")
            }
            Error::NoWordThisLength(word_size) => {
                write!(f, "No word of length {word_size} found.")
            }
            Error::EmptyWordVec(word_size) => {
                write!(f, "Vector is empty for length {word_size}.")
            }
        }
    }
}

/// Loads words from a file and organizes them into a hashmap based on their length.
///
/// # Arguments
///
/// * `data_file_path` - The path to the file containing the words.
///
/// # Returns
///
/// Returns a `Result` containing a hashmap where the keys are word lengths and the values are sets of words.
///
/// # Errors
///
/// - `wordlers::data_loader::Error` if the file contains non-alphabetic characters.
/// - `std::io::Error` if the file cannot be read.
///
/// # Examples
///
/// ```no_run
/// use wordlers::data_loader::load_words_file;
/// use std::collections::HashSet;
/// use std::io::Write;
///
/// // Initialize a temporary file with words
/// let mut file = tempfile::NamedTempFile::new().unwrap();
/// // 2 words of length 5, 1 word of length 9, 1 words of length 6
/// write!(file, "apple\npeach\npineapple\ncherry").unwrap();
///
/// let result = load_words_file(file.path().to_str().unwrap());
/// assert!(result.is_ok());
/// let word_hashmap = result.unwrap();
/// assert_eq!(word_hashmap.get(&5).unwrap(), &HashSet::from_iter(vec!["APPLE".to_string(), "PEACH".to_string()]));
/// assert_eq!(word_hashmap.get(&6).unwrap(), &HashSet::from_iter(vec!["CHERRY".to_string()]));
/// assert_eq!(word_hashmap.get(&9).unwrap(), &HashSet::from_iter(vec!["PINEAPPLE".to_string()]));
/// ```
pub fn load_words_file(
    data_file_path: &str,
) -> Result<HashMap<usize, HashSet<String>>, Box<dyn std::error::Error>> {
    let raw_file = std::fs::read_to_string(data_file_path)?;
    let lines_with_len: Result<HashSet<(usize, String)>, Box<Error>> = raw_file
        .lines()
        .map(|line| {
            if line.chars().all(char::is_alphabetic) {
                Ok((line.len(), line.to_uppercase()))
            } else {
                Err(Box::new(Error::NotAlphabetic(String::from(data_file_path))))
            }
        })
        .collect();

    match lines_with_len {
        Ok(lines_with_len) => {
            Ok(lines_with_len
                .into_iter()
                .fold(HashMap::new(), |mut acc, (len, line)| {
                    acc.entry(len).or_default().insert(line);
                    acc
                }))
        }
        Err(err) => Err(err),
    }
}

/// Chooses a random word of a specific length from a hashmap of words.
///
/// # Arguments
///
/// * `word_hashmap` - The hashmap containing words organized by their length.
/// * `word_length` - The length of the word to choose.
///
/// # Returns
///
/// Returns a `Result` containing the chosen word as a `String`.
/// If no word of the specified length is found or the vector of words is empty, a `Box<dyn Error>` is returned.
///
/// # Errors
///
/// - `wordlers::data_loader::Error::NoWordThisLength` if no word of the specified length is found.
/// - `wordlers::data_loader::Error::EmptyWordVec` if the vector of words is empty.
///
/// # Examples
///
/// ```
/// use std::collections::{HashMap, HashSet};
/// use wordlers::data_loader::choose_random_word;
///
/// let mut word_hashmap = HashMap::new();
/// let words_set_5: HashSet<String> = vec![
///     String::from("APPLE"),
///     String::from("PEACH"),
/// ].into_iter().collect();
///
/// let words_set_6: HashSet<String> = vec![
///    String::from("BANANA"),
/// ].into_iter().collect();
/// word_hashmap.insert(5, words_set_5.clone());
/// word_hashmap.insert(6, words_set_6.clone());
///
/// let result = choose_random_word(&word_hashmap, 5);
/// assert!(result.is_ok());
/// let chosen_word = result.unwrap();
/// assert!(words_set_5.contains(&chosen_word));
/// assert!(chosen_word.len() == 5);
/// assert!(!words_set_6.contains(&chosen_word));
/// ```
pub fn choose_random_word<S: BuildHasher>(
    word_hashmap: &HashMap<usize, HashSet<String, S>, S>,
    word_length: usize,
) -> Result<String, Error> {
    match word_hashmap.get(&word_length) {
        None => Err(Error::NoWordThisLength(word_length)),
        Some(words_set) => {
            match words_set.iter().collect::<Vec<&String>>()[..].choose(&mut rand::thread_rng()) {
                None => Err(Error::EmptyWordVec(word_length)),
                Some(word) => Ok((*word).clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_words_file() {
        // Init files for testing
        let mut valid_file = tempfile::NamedTempFile::new().unwrap();

        // 2 words of length 5, 1 word of length 9, 1 words of length 6
        write!(valid_file, "apple\npeach\npineapple\ncherry").unwrap();

        let mut non_alphabetic_file = tempfile::NamedTempFile::new().unwrap();
        write!(non_alphabetic_file, "apple\nbanana-cherry\n123").unwrap();

        let empty_file = tempfile::NamedTempFile::new().unwrap();

        // Test case 1: Valid file with alphabetic characters
        let result = load_words_file(valid_file.path().to_str().unwrap());
        assert!(result.is_ok());
        let word_hashmap = result.unwrap();
        assert_eq!(
            word_hashmap,
            HashMap::from_iter(vec![
                (
                    5,
                    vec!["APPLE".to_string(), "PEACH".to_string()]
                        .into_iter()
                        .collect()
                ),
                (6, vec!["CHERRY".to_string()].into_iter().collect()),
                (9, vec!["PINEAPPLE".to_string()].into_iter().collect()),
            ])
        );

        // Test case 2: File with non-alphabetic characters
        let non_alphabetic_file_path_as_str = non_alphabetic_file.path().to_str().unwrap();
        let result = load_words_file(non_alphabetic_file_path_as_str);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("File {non_alphabetic_file_path_as_str} contains non-alphabetic characters.")
        );

        // Test case 3: Empty file
        let result = load_words_file(empty_file.path().to_str().unwrap());
        assert!(result.is_ok());
        let word_hashmap = result.unwrap();
        assert_eq!(word_hashmap.len(), 0);
    }

    #[test]
    fn test_choose_random_word() {
        let mut word_hashmap = HashMap::new();
        let words_set: HashSet<String> = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ]
        .into_iter()
        .collect();
        word_hashmap.insert(5, words_set.clone());

        // Test case 1: Word of specified length exists
        let result = choose_random_word(&word_hashmap, 5);
        assert!(result.is_ok());
        let chosen_word = result.unwrap();
        assert!(words_set.contains(&chosen_word));

        // Test case 2: Word of specified length does not exist
        let result = choose_random_word(&word_hashmap, 6);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "No word of length 6 found.");

        // Test case 3: Word vector is empty
        let mut empty_word_hashmap = HashMap::new();
        let empty_words_set: HashSet<String> = HashSet::new();
        empty_word_hashmap.insert(5, empty_words_set);
        let result = choose_random_word(&empty_word_hashmap, 5);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Vector is empty for length 5.");
    }
}
