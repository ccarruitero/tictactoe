mod game;
mod board;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
