mod key_codes;
pub use key_codes::keycodes;

pub struct Engine {
    buffer: Vec<char>
}

#[derive(Debug)]
pub struct PhysicKey {
    pub keycode: u32,
    pub state: KeyState,
    pub cap: Option<KeyCap>,
}

#[derive(Debug)]
pub enum KeyState {
    KeyPress,
    KeyRelease
}

#[derive(Debug)]
pub enum KeyCap {
    Shift,
    CapsLock
}

pub enum Action {
    Back(usize),
    Forward(usize),
    Insert(char),
    Backspace(usize)
}

impl Into<char> for PhysicKey {
    fn into(self) -> char {
        std::char::from_u32(self.keycode).unwrap_or('\0')
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn handle_key(&mut self, key: PhysicKey) -> Vec<Action> {
        let ch: char = key.into();
        if ch == 'a' && self.buffer.last().unwrap_or(&'\0') == &'a' {
            return vec![
                Action::Backspace(2),
                Action::Insert('â')
            ];
        }
        self.buffer.push(ch);
        vec![Action::Insert(ch)]
    }
}
