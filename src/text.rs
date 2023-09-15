use crate::displayable::Displayable;

pub struct Text {
    text : String,
}

impl Text {
    pub fn new(initial_text: String) -> Self {
        Text { text: initial_text }
    }
}

impl Displayable for Text {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}
