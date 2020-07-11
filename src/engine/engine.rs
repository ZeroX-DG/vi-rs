use super::vni;
use super::key::Key;

pub struct Engine {
    input_method: InputMethod,
    buffer: Vec<char>
}

pub enum InputMethod {
    Vni
}

impl Engine {
    pub fn new() -> Self {
        Self {
            input_method: InputMethod::Vni,
            buffer: Vec::new()
        }
    }

    pub fn handle_key(&mut self, key: Key) -> String {
        self.buffer.push(key.get_char());
        match self.input_method {
            InputMethod::Vni => vni::transform_buffer(&self.buffer)
        }
    }
}
