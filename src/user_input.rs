use std::io;

pub mod format_and_check;

/// Reads user input from stdin.
/// # Errors
/// Returns an error if the input cannot be read
/// # Examples
/// ```
/// use wordlers::input::get_user_input_stdin;
///
/// let user_input = get_user_input_stdin().unwrap();
/// ```
pub fn get_user_input_stdin() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
