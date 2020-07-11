pub struct Key {
    ch: char,
    state: KeyState
}

pub enum KeyState {
    Down,
    Release
}

impl Key {
    pub fn get_char(&self) -> char {
        self.ch
    }
}
