use core::fmt;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::BuildHasher;

/// Custom error type for data loading errors.
#[derive(Debug)]
enum DataLoaderError {
    /// Error indicating that a file contains non-alphabetic characters.
    NotAlphabetic(String),
    /// Error indicating that no word of a specific length was found.
    NoWordThisLength(usize),
    /// Error indicating that a vector of words is empty for a specific length.
    EmptyWordVec(usize),
}

impl Error for DataLoaderError {}

impl Display for DataLoaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataLoaderError::NotAlphabetic(path) => write!(f, "File {path}."),
            DataLoaderError::NoWordThisLength(word_size) => {
                write!(f, "No word of length {word_size} found.")
            }
            DataLoaderError::EmptyWordVec(word_size) => {
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
/// - `DataLoaderError` if the file contains non-alphabetic characters.
/// - `std::io::Error` if the file cannot be read.
pub fn load_words_file(
    data_file_path: &str,
) -> Result<HashMap<usize, HashSet<String>>, Box<dyn Error>> {
    let raw_file = std::fs::read_to_string(data_file_path)?;
    let lines_with_len: Result<HashSet<(usize, String)>, Box<DataLoaderError>> = raw_file
        .lines()
        .map(|line| {
            if line.chars().all(char::is_alphabetic) {
                Ok((line.len(), line.to_uppercase()))
            } else {
                Err(Box::new(DataLoaderError::NotAlphabetic(String::from(
                    data_file_path,
                ))))
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
/// - `DataLoaderError::NoWordThisLength` if no word of the specified length is found.
/// - `DataLoaderError::EmptyWordVec` if the vector of words is empty.
pub fn choose_random_word<S: BuildHasher>(
    word_hashmap: &HashMap<usize, HashSet<String, S>, S>,
    word_length: usize,
) -> Result<String, Box<dyn Error>> {
    match word_hashmap.get(&word_length) {
        None => Err(Box::new(DataLoaderError::NoWordThisLength(word_length))),
        Some(words_set) => {
            match words_set.iter().collect::<Vec<&String>>()[..].choose(&mut rand::thread_rng()) {
                None => Err(Box::new(DataLoaderError::EmptyWordVec(word_length))),
                Some(word) => Ok((*word).clone()),
            }
        }
    }
}
