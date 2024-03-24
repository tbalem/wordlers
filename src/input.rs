use std::io;

pub mod format_and_check;

/// Reads user input from stdin.
pub fn get_user_input_stdin() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
