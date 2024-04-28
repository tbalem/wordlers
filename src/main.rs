pub mod cli;

use cli::game_loop;

fn main() {
    let guess_word = String::from("TESTS");

    loop {
        match game_loop(&guess_word) {
            Ok(results) => println!("{}", results.into_iter().collect::<String>()),
            Err(err) => eprint!("{err}"),
        }
    }
}
