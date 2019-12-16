use super::{Keyboard};
use crate::engine::{keycodes, PhysicKey, KeyState};

pub struct KeyboardHandler {
}

impl KeyboardHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Keyboard for KeyboardHandler {
    fn forward(&self, amount: usize) {}
    fn back(&self, amount: usize) {}
    fn backspace(&self, amount: usize) {
    }
    fn insert(&self, ch: char) {}
    fn wait_for_key(&self) -> Option<PhysicKey> {
        None
    }
}

