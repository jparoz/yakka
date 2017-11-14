pub struct Settings {
    pub complete_char: char,
    pub incomplete_char: char,

    pub width: usize,
    pub height: usize,
}

impl Settings {
    pub fn complete_char(&self, complete: bool) -> char {
        if complete {
            self.complete_char
        } else {
            self.incomplete_char
        }
    }
}
