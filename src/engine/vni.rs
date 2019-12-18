use super::{PhysicKey, Action, KeyState};

pub struct Vni {
    buffer: Vec<char>
}

const TRIGGER_CIRCUMFLEX: char = '6';

impl Vni {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    fn replace_char_at(&mut self, index: usize, ch: char) -> Vec<Action> {
        let buffer_len = self.buffer.len();
        let backspace_amount = buffer_len - index + 1;
        let deleted_chars = self.buffer
            .iter()
            .skip(index + 1)
            .take(backspace_amount)
            .map(|c| c.clone())
            .collect::<Vec<char>>();
        let mut steps: Vec<Action> = vec![
            Action::Backspace(backspace_amount),
            Action::Insert(ch),
        ];
        for deleted_char in deleted_chars {
            steps.push(Action::Insert(deleted_char));
        }
        steps
    }

    fn add_circumflex(&mut self) -> Vec<Action> {
        let buffer_len = self.buffer.len();
        for i in 0..buffer_len {
            let ch = self.buffer[i];
            let next_ch = if i == buffer_len - 1 {
                self.buffer[i]
            } else {
                self.buffer[i + 1]
            };
            match ch {
                'a' | 'A' => {
                    let pair = ['u', 'p', 'n', 'm', 't', 'c'];
                    let replace_char = if ch == 'A' {
                        'Â'
                    } else {
                        'â'
                    };
                    let next_ch_lower = &next_ch.to_ascii_lowercase();
                    if pair.contains(next_ch_lower) || i == buffer_len - 1 {
                        let steps = self.replace_char_at(i, replace_char);
                        self.buffer[i] = replace_char;
                        return steps;
                    }
                },
                'o' | 'O' => {
                    let pair = ['i', 'p', 'n', 'm', 'p', 't', 'c'];
                    let replace_char = if ch == 'O' {
                        'Ô'
                    } else {
                        'ô'
                    };
                    let next_ch_lower = &next_ch.to_ascii_lowercase();
                    if pair.contains(next_ch_lower) || i == buffer_len - 1 {
                        let steps = self.replace_char_at(i, replace_char);
                        self.buffer[i] = replace_char;
                        return steps;
                    }
                },
                'e' | 'E' => {
                    let pair = ['u', 'n', 'm', 'p', 't', 'c'];
                    let replace_char = if ch == 'E' {
                        'Ê'
                    } else {
                        'ê'
                    };
                    let next_ch_lower = &next_ch.to_ascii_lowercase();
                    if pair.contains(next_ch_lower) || i == buffer_len - 1 {
                        let steps = self.replace_char_at(i, replace_char);
                        self.buffer[i] = replace_char;
                        return steps;
                    }
                }
                _ => {}
            }
        }
        vec![]
    }

    fn handle_normal_char(&mut self, ch: char) -> Vec<Action> {
        match ch {
            TRIGGER_CIRCUMFLEX => self.add_circumflex(),
            _ => Vec::new()
        }
    }

    pub fn handle_key(&mut self, key: PhysicKey) -> Vec<Action> {
        let mut ch: char = key.clone().into();
        let mut actions: Vec<Action> = Vec::new();
        match key.state {
            KeyState::KeyPress => {
                let mut clear_buffer = false;
                if key.is_arrow() || key.is_whitespace() {
                    clear_buffer = true;
                } else if key.is_backspace() {
                    self.buffer.pop();
                } else {
                    ch = match key.cap {
                        Some(_) => ch.to_ascii_uppercase(),
                        None => ch
                    };
                    actions = self.handle_normal_char(ch);
                }
                if clear_buffer {
                    self.buffer.clear();
                } else {
                    if ch != '\0' && actions.is_empty() {
                        self.buffer.push(ch);
                    }
                }
                println!("{:?}", self.buffer);
            }
            _ => {}
        }
        actions
    }
}
