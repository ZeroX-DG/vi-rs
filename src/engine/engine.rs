use super::vni;
use super::super::key::{Key, KeyState};

pub struct Engine {
    input_method: InputMethod,
    buffer: Vec<char>
}

pub enum InputMethod {
    Vni
}

#[derive(Debug)]
pub enum Action {
    Insert(String),
    Backspace(usize)
}

impl Engine {
    pub fn new() -> Self {
        Self {
            input_method: InputMethod::Vni,
            buffer: Vec::new()
        }
    }

    pub fn handle_key(&mut self, key: Key) -> Vec<Action> {
        if let KeyState::Down = key.get_state() {
            if key.is_whitespace() || key.is_enter() || key.is_tab() || key.is_arrow() {
                self.buffer.clear();
                return Vec::new();
            }

            if key.is_backspace() {
                self.buffer.pop();
                return Vec::new();
            }
            if key.is_recognized_char() {
                self.buffer.push(key.get_char());
            }

            let (has_action, transform_result) = match self.input_method {
                InputMethod::Vni => vni::transform_buffer(&self.buffer)
            };

            if !has_action {
                return Vec::new()
            }

            self.buffer.clear();
            self.buffer = transform_result.chars().collect();

            return vec![
                Action::Backspace(self.buffer.len() + 1),
                Action::Insert(transform_result)
            ];
        }
        Vec::new()
    }
}
