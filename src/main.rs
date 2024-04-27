pub mod cli;

use cli::game_loop;

fn main() {
    let guess_word = String::from("TESTS");

    loop {
        game_loop(&guess_word);
    }
}
