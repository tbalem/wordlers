use std::collections::HashMap;
use std::io;

fn main() {
    let guess_word = String::from("TESTS");
    let char_counts = guess_word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    loop {
        let mut cloned_char_counts = char_counts.clone();
        let mut input = String::new();

        println!("Please input a new guess: ");
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error while reading input.");
            continue;
        }

        let trimmed_uppercased_input = input.trim().to_uppercase();

        if trimmed_uppercased_input.chars().count() != 5 {
            println!("Your guess must be exactly 5 characters.");
        } else if !trimmed_uppercased_input
            .chars()
            .all(|c| c.is_ascii_alphabetic())
        {
            println!("Some of your input is not an alphabetic character.");
        } else {
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
    }
}
