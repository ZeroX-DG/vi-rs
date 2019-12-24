mod key_codes;
mod vni;
mod util;
mod character_map;

pub use key_codes::keycodes;
use vni::Vni;

pub struct Engine {
    vni: Vni
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

#[derive(Debug, Clone)]
pub enum Action {
    Insert(char),
    Backspace(usize)
}

impl Into<char> for PhysicKey {
    fn into(self) -> char {
        match self.keycode {
            keycodes::KEY_A => 'a',
            keycodes::KEY_B => 'b',
            keycodes::KEY_C => 'c',
            keycodes::KEY_D => 'd',
            keycodes::KEY_E => 'e',
            keycodes::KEY_F => 'f',
            keycodes::KEY_G => 'g',
            keycodes::KEY_H => 'h',
            keycodes::KEY_I => 'i',
            keycodes::KEY_J => 'j',
            keycodes::KEY_K => 'k',
            keycodes::KEY_L => 'l',
            keycodes::KEY_M => 'm',
            keycodes::KEY_N => 'n',
            keycodes::KEY_O => 'o',
            keycodes::KEY_P => 'p',
            keycodes::KEY_Q => 'q',
            keycodes::KEY_R => 'r',
            keycodes::KEY_S => 's',
            keycodes::KEY_T => 't',
            keycodes::KEY_U => 'u',
            keycodes::KEY_V => 'v',
            keycodes::KEY_W => 'w',
            keycodes::KEY_X => 'x',
            keycodes::KEY_Y => 'y',
            keycodes::KEY_Z => 'z',
            // numbers
            keycodes::KEY_1 => '1',
            keycodes::KEY_2 => '2',
            keycodes::KEY_3 => '3',
            keycodes::KEY_4 => '4',
            keycodes::KEY_5 => '5',
            keycodes::KEY_6 => '6',
            keycodes::KEY_7 => '7',
            keycodes::KEY_8 => '8',
            keycodes::KEY_9 => '9',
            keycodes::KEY_0 => '0',
            _ => '\0'
        }
    }
}

impl PhysicKey {
    pub fn is_whitespace(&self) -> bool {
        match self.keycode {
            keycodes::KEY_SPACE => true,
            keycodes::KEY_TAB => true,
            keycodes::KEY_ENTER => true,
            _ => false
        }
    }

    pub fn is_arrow(&self) -> bool {
        match self.keycode {
            keycodes::KEY_LEFT => true,
            keycodes::KEY_RIGHT => true,
            keycodes::KEY_UP => true,
            keycodes::KEY_DOWN => true,
            _ => false
        }
    }

    pub fn is_backspace(&self) -> bool {
        self.keycode == keycodes::KEY_BACKSPACE
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            vni: Vni::new()
        }
    }

    pub fn handle_key(&mut self, key: PhysicKey) -> Vec<Action> {
        self.vni.handle_key(key)
    }
}
