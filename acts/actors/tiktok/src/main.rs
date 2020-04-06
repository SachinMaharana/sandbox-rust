mod game;

use game::Game;
fn main() {
    // assert_eq!(!Answer::Yes, Answer::No);
    // assert_eq!(!Answer::No, Answer::Yes);

    let mut game = Game::new();

    while !game.is_finished() {
        print_tiles(game.tiles());
    }
}
