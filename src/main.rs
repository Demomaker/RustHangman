mod screen;
mod text;
mod displayable;
mod game;

use screen::Screen;
use text::Text;
use game::Game;

fn main() {
    // Create instances without using mut unnecessarily
    let screen = Screen::new();
    let mut game = Game::new(screen);

    game.run();
}
