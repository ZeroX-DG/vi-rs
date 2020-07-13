/// A key that the engine need to process
pub struct Key {
    type_: KeyType,
    state: KeyState
}

/// The state of a key, down or release
pub enum KeyState {
    Down,
    Release
}

/// A type of a key, down or release
pub enum KeyType {
    Char(char),
    Space,
    Tab,
    Arrow,
    Enter,
    Backspace
}

/// Represent a key
impl Key {
    pub fn new(type_: KeyType, state: KeyState) -> Self {
        Self {
            type_,
            state
        }
    }
    /// Returns the char of the key
    pub fn get_char(&self) -> Option<char> {
        if let KeyType::Char(ch) = self.type_ {
            return Some(ch);
        }
        None
    }
    /// Returns the state of the key
    pub fn get_state(&self) -> &KeyState {
        &self.state
    }
    /// Returns true if the key is a whitespace and vice versa.
    pub fn is_whitespace(&self) -> bool {
        match &self.type_ {
            KeyType::Space => true,
            _ => false
        }
    }
    /// Returns true if the key is an enter key and vice versa.
    pub fn is_enter(&self) -> bool {
        match &self.type_ {
            KeyType::Enter => true,
            _ => false
        }
    }
    /// Returns true if the key is a tab key and vice versa.
    pub fn is_tab(&self) -> bool {
        match &self.type_ {
            KeyType::Tab => true,
            _ => false
        }
    }
    /// Returns true if the key is an arrow key and vice versa.
    pub fn is_arrow(&self) -> bool {
        match &self.type_ {
            KeyType::Arrow => true,
            _ => false
        }
    }
    /// Returns true if the key is a backspace and vice versa.
    pub fn is_backspace(&self) -> bool {
        match &self.type_ {
            KeyType::Backspace => true,
            _ => false
        }
    }
}
