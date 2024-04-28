pub mod cli;

use cli::game_iteration;

fn main() {
    let guess_word = String::from("TESTS");

    if game_iteration(&guess_word, 5) {
        println!("Congratulations, you won!");
    } else {
        println!("You lost.");
    }
}
