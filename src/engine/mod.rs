mod key_codes;
pub use key_codes::keycodes;

pub struct Engine {
    buffer: Vec<char>
}

#[derive(Debug, Clone)]
pub struct PhysicKey {
    pub keycode: u32,
    pub state: KeyState,
    pub cap: Option<KeyCap>,
}

#[derive(Debug, Clone)]
pub enum KeyState {
    KeyPress,
    KeyRelease
}

#[derive(Debug, Clone)]
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
        match self.keycode {
            keycodes::KeyA => 'a',
            _ => '\0'
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn handle_key(&mut self, key: PhysicKey) -> Vec<Action> {
        let ch: char = key.clone().into();
        match key.state {
            KeyState::KeyPress => {
                if ch == 'a' && self.buffer.last().unwrap_or(&'\0') == &'a' {
                    self.buffer.clear();
                    return vec![
                        Action::Backspace(2),
                        Action::Insert('â')
                    ];
                }
                if ch != '\0' {
                    self.buffer.push(ch);
                }
            }
            _ => {}
        }
        println!("{:?}", self.buffer);
        vec![]
    }
}
