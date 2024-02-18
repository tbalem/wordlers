use std::collections::HashMap;
use std::io::{self, Write};

enum UserInputError {
    IoError(io::Error),
    Not5Chars,
    NotAlphabetic,
}

fn get_user_input() -> Result<String, UserInputError> {
    let mut input = String::new();
    println!("Please input a new guess: ");
    io::stdout().flush().map_err(UserInputError::IoError)?;

    io::stdin()
        .read_line(&mut input)
        .map_err(UserInputError::IoError)?; // Error while reading user input.

    let trimmed_uppercased_input = input.trim();

    if trimmed_uppercased_input.chars().count() != 5 {
        Err(UserInputError::Not5Chars) // Your guess must be exactly 5 characters.
    } else if !trimmed_uppercased_input
        .chars()
        .all(|c| c.is_ascii_alphabetic())
    {
        Err(UserInputError::NotAlphabetic) // Some of your input is not an alphabetic character.
    } else {
        Ok(trimmed_uppercased_input.to_ascii_uppercase())
    }
}

fn main() {
    let guess_word = String::from("TESTS");
    let char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    loop {
        let mut cloned_char_counts = char_counts.clone();

        match get_user_input() {
            Ok(trimmed_uppercased_input) => {
                let mut result_string = ['-'; 5];
                for (i, (char1, char2)) in guess_word
                    .chars()
                    .zip(trimmed_uppercased_input.chars())
                    .enumerate()
                {
                    if char1 == char2 {
                        result_string[i] = 'X';
                        if let Some(char_count) = cloned_char_counts.get_mut(&char1) {
                            *char_count -= 1;
                        } else {
                            println!("Error reading char {i}: {char1} in char_counts HashMap");
                            continue;
                        }
                    }
                }

                for (i, result_char_i) in result_string.iter_mut().enumerate() {
                    if *result_char_i != 'X' {
                        if let Some(input_char) = trimmed_uppercased_input.chars().nth(i) {
                            if guess_word.contains([input_char]) {
                                if let Some(char_count) = cloned_char_counts.get_mut(&input_char) {
                                    if *char_count > 0 {
                                        *result_char_i = 'O';
                                        *char_count -= 1;
                                    }
                                }
                            }
                        } else {
                            println!("Error reading char {i} in trimmed_uppercased_input");
                        }
                    }
                }

                println!("{}", String::from_iter(result_string.iter()));
            }
            Err(UserInputError::IoError(err)) => {
                eprintln!("Error while reading input: {}", err);
            }
            Err(UserInputError::Not5Chars) => {
                eprintln!("Your guess must be exactly 5 characters.");
            }
            Err(UserInputError::NotAlphabetic) => {
                eprintln!("Some of your input is not an alphabetic character.");
            }
        }
    }
}
