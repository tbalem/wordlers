use core::fmt;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::BuildHasher;

#[derive(Debug)]
enum DataLoaderError {
    NotAlphabetic(String),
    NoWordThisLength(usize),
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

/// # Errors
pub fn load_words_file(
    data_file_path: &str,
) -> Result<HashMap<usize, Vec<String>>, Box<dyn Error>> {
    let raw_file = std::fs::read_to_string(data_file_path)?;
    let lines_with_len: Result<Vec<(usize, String)>, Box<DataLoaderError>> = raw_file
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
                    acc.entry(len).or_default().push(line);
                    acc
                }))
        }
        Err(err) => Err(err),
    }
}

/// # Errors
pub fn choose_random_word<S: BuildHasher>(
    word_hashmap: &HashMap<usize, Vec<String>, S>,
    word_length: usize,
) -> Result<String, Box<dyn Error>> {
    match word_hashmap.get(&word_length) {
        None => Err(Box::new(DataLoaderError::NoWordThisLength(word_length))),
        Some(words_vec) => match words_vec[..].choose(&mut rand::thread_rng()) {
            None => Err(Box::new(DataLoaderError::EmptyWordVec(word_length))),
            Some(word) => Ok(word.clone()),
        },
    }
}