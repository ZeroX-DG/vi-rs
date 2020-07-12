pub struct Key {
    ch: char,
    code: u16,
    state: KeyState
}

pub enum KeyState {
    Down,
    Release
}

impl Key {
    pub fn new(ch: char, code: u16, state: KeyState) -> Self {
        Self {
            ch,
            code,
            state
        }
    }
    pub fn get_char(&self) -> char {
        self.ch
    }
    pub fn get_state(&self) -> &KeyState {
        &self.state
    }
}
