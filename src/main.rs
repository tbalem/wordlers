pub mod cli;

use cli::cli_game_loop;

fn main() {
    let guess_word = String::from("TESTS");

    loop {
        cli_game_loop(&guess_word);
    }
}
