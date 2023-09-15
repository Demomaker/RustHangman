use std::io::{self, Write};
use crate::displayable::Displayable;

pub struct Screen {
}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn show(&self, displayable: &dyn Displayable) {
        print!("{}", displayable.to_string());
        io::stdout().flush().unwrap();
    }

    pub fn input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn clear(&self) {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}