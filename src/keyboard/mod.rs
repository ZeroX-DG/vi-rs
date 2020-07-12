use super::key::*;

#[cfg(target_os = "linux")]
pub mod linux;

pub trait Keyboard {
    fn backspace(&mut self, amount: usize);
    fn insert(&mut self, text: String);
    fn wait_for_key(&mut self) -> Key;
}
