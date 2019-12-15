use crate::engine::PhysicKey;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::KeyboardHandler;

pub trait Keyboard {
    fn init(&mut self);
    fn back(&self, amount: usize);
    fn forward(&self, amount: usize);
    fn backspace(&self, amount: usize);
    fn insert(&self, ch: char);
    fn wait_for_key(&self) -> Option<PhysicKey>;
}

pub fn get_keyboard() -> Box<Keyboard> {
    Box::new(KeyboardHandler::new())
}
