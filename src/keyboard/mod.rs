use crate::engine::PhysicKey;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::KeyboardHandler;

pub trait Keyboard {
    fn backspace(&self, amount: usize);
    fn insert(&self, ch: char);
    fn wait_for_key(&mut self) -> PhysicKey;
}

pub fn get_keyboard() -> Box<dyn Keyboard> {
    Box::new(KeyboardHandler::new())
}
