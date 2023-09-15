mod screen;
mod text;
mod displayable;
mod game;
mod hangman;
mod game_state;

use screen::Screen;
use game::Game;

fn main() {
    // Create instances without using mut unnecessarily
    let screen = Screen::new();
    let mut game = Game::new(screen);

    game.run();
}
