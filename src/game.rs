use crate::screen::Screen;
use crate::text::Text; // Import the Text type from the appropriate module
use crate::hangman::Hangman;
use crate::game_state::GameState;
use std::rc::Rc;

pub struct Game<'a> {
    game_state: GameState,
    screen: Rc<Screen>,
    hangman: Hangman<'a>,
}

impl<'a> Game<'a> {
    pub fn new(screen: Screen) -> Self {
        let screen_rc = Rc::new(screen);

        Game {
            game_state: GameState::Init,
            screen: Rc::clone(&screen_rc),
            hangman: Hangman::new(Rc::clone(&screen_rc)),
        }
    }

    pub fn run(&mut self) {
        while self.can_run() {
            match self.game_state {
                GameState::Init => self.init_game(),
                GameState::Play => self.play_game(),
                GameState::Pause => self.pause_game(),
                GameState::Continue => self.continue_game(),
                _ => (),
            }
        }

        self.stop_game();
    }

    fn init_game(&mut self) {
        // Implement initialization logic
        self.hangman.init();
        self.game_state = GameState::Play;
    }

    fn play_game(&mut self) {
        self.game_state = self.hangman.play();
    }

    fn pause_game(&mut self) {
        // Implement pause logic
        let text = Text::new("Enter 'continue' to continue or 'stop' to stop".to_string());
        self.screen.show(&text);
        let input = self.screen.input();
        if input == "continue" {
            self.game_state = GameState::Continue;
        }
        if input == "stop" {
            self.game_state = GameState::Stop;
        }
    }

    fn continue_game(&mut self) {
        // Implement continue logic
        self.game_state = GameState::Play;
    }

    fn stop_game(&mut self) {
        // Implement stop logic
    }

    fn can_run(&self) -> bool {
        match self.game_state {
            GameState::Stop => false,
            _ => true,
        }
    }
}
