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
    pub fn is_whitespace(&self) -> bool {
        if cfg!(target_os = "linux") {
            return self.code == input_event_codes::KEY_SPACE
        }
        false
    }
    pub fn is_backspace(&self) -> bool {
        if cfg!(target_os = "linux") {
            return self.code == input_event_codes::KEY_BACKSPACE
        }
        false
    }
    pub fn is_recognized_char(&self) -> bool {
        self.get_char() != '\0'
    }
}
