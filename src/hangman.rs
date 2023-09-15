use crate::game_state::GameState;
use crate::screen::Screen;
use crate::text::Text;
use std::rc::Rc;
use rand::Rng;

pub struct Hangman<'a> {
    lives: i32,
    word: String,
    word_pool: Vec<&'a str>,
    hidden_word: String,
    screen: Rc<Screen>,
}

impl<'a> Hangman<'a> {
    pub fn new(screen: Rc<Screen>) -> Self {
        Hangman { 
            lives: 6,
            word: "".to_string(),
            word_pool: vec!["apple", "banana", "cherry", "grape", "orange"],
            hidden_word: String::new(),
            screen
        }
    }

    pub fn init(&mut self) {
        self.word = self.choose_word().to_string();
        self.hidden_word = self.to_hidden();
        self.lives = 6;
    }

    pub fn play(&mut self) -> GameState {
        let hangman = Text::new(self.get_hangman_string().to_string());
        let word_to_guess = Text::new(self.hidden_word.to_string());
        let empty_line = Text::new("\nYour guess or special command (stop, pause, replay) : ".to_string());
        let win_line = Text::new("\nYou won! Press any character to replay".to_string());
        self.screen.clear();
        self.screen.show(&hangman);
        self.screen.show(&word_to_guess);

        if self.won_game() {
            self.screen.show(&win_line);
        } else {
            self.screen.show(&empty_line);
        }

        if self.lives == 0 {
            return GameState::Stop;
        }

        let input = self.screen.input().trim().to_lowercase(); // Normalize and trim input
        

        if self.won_game() {
            return GameState::Init;
        } else if input.is_empty() {
            // Player didn't enter anything
            println!("Please enter a valid character or a special command.");
        } else if input == "stop" {
            return GameState::Stop;
        } else if input == "pause" {
            return GameState::Pause;
        } else if input == "replay" {
            return GameState::Init;
        } else if input.len() > 1 {
            // Player entered more than one character
            println!("Please enter only one character or a special command.");
        } else if !self.found_character(&input) && !self.won_game() {
            self.lives -= 1;
        }

        GameState::Play
    }

    fn found_character(&mut self, guess: &str) -> bool {
        let guess_char = guess.chars().next().unwrap();
        let mut updated_hidden_word = self.hidden_word.clone();
        let mut found = false;
        for (index, character) in self.word.chars().enumerate() {
            if character == guess_char {
                updated_hidden_word.replace_range(index..(index) + 1, &guess_char.to_string());
                found = true;
            }
        }
        self.hidden_word = updated_hidden_word;
        found
    }

    fn won_game(&mut self) -> bool {
        self.word == self.hidden_word
    }

    fn get_hangman_string(&mut self) -> &str {
        match self.lives {
            0 => HANGMAN_COMPLETE,
            1 => HANGMAN_6,
            2 => HANGMAN_5,
            3 => HANGMAN_4,
            4 => HANGMAN_3,
            5 => HANGMAN_2,
            6 => HANGMAN_1,
            _ => "INVALID HANGMAN STATE"
        }
    }

    fn to_hidden(&mut self) -> String {
        let mut hidden_word = String::new();
        for _ in self.word.chars() {
            hidden_word.push('-');
        }
        hidden_word
    }

    fn choose_word(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.word_pool.len());
        self.word_pool[index].to_string()
    }
}

const HANGMAN_1: &str = "
  -----
 |     |
 |
 |
 |
| |
";

const HANGMAN_2: &str = "
  -----
 |     |
 |     O
 |
 |
| |
";

const HANGMAN_3: &str = "
  -----
 |     |
 |     O
 |     |
 |
| |
";

const HANGMAN_4: &str = "
  -----
 |     |
 |     O
 |    /|
 |
| |
";

const HANGMAN_5: &str = "
  -----
 |     |
 |     O
 |    /|\\
 |
| |
";

const HANGMAN_6: &str = "
  -----
 |     |
 |     O
 |    /|\\
 |    /
| |
";

const HANGMAN_COMPLETE: &str = "
  -----
 |     |
 |     O
 |    /|\\
 |    / \\
| |
";