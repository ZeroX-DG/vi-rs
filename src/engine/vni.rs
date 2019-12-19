use super::{PhysicKey, Action, KeyState};
use super::util;

pub struct Vni {
    buffer: Vec<char>
}

const TRIGGER_CIRCUMFLEX: char = '6';
const TRIGGER_HORN: char = '7';
const TRIGGER_BREVE: char = '8';
const TRIGGER_CROSSED_D: char = '9';

struct DiacriticMatch {
    pub ch: char,
    pub pair_with: Vec<char>,
    pub replace_with: (char, char) // lowercase && uppercase
}

impl Vni {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    fn replace_char_at(&mut self, index: usize, ch: char, is_first_edit: bool)
        -> Vec<Action> {
        let buffer_len = self.buffer.len();
        let mut backspace_amount = buffer_len - index;
        if is_first_edit {
            backspace_amount += 1;
        }
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

    fn add_diacritic(&mut self, matches: Vec<DiacriticMatch>) -> Vec<Action> {
        let buffer_len = self.buffer.len();
        let mut steps: Vec<Action> = Vec::new();
        let mut is_first_match = true;
        for i in 0..buffer_len {
            let ch = self.buffer[i];
            let next_ch = if i == buffer_len - 1 {
                self.buffer[i]
            } else {
                self.buffer[i + 1]
            };
            let clean_ch = util::remove_accents(ch);
            for diacritic_match in &matches {
                if diacritic_match.ch == clean_ch.to_ascii_lowercase() {
                    let next_ch_lower = &util::remove_accents(
                        next_ch.to_ascii_lowercase()
                    );
                    if diacritic_match.pair_with.contains(next_ch_lower)
                        || i == buffer_len - 1 {
                        
                        let replace_char = if ch.is_ascii_uppercase() {
                            diacritic_match.replace_with.1
                        } else {
                            diacritic_match.replace_with.0
                        };
                        steps = [
                            steps,
                            self.replace_char_at(i, replace_char, is_first_match)
                        ].concat();
                        self.buffer[i] = replace_char;
                        if is_first_match {
                            is_first_match = false;
                        }
                    }
                }
            }
        }
        steps
    }

    fn handle_normal_char(&mut self, ch: char) -> Vec<Action> {
        match ch {
            TRIGGER_CIRCUMFLEX => self.add_diacritic(vec![
                DiacriticMatch {
                    ch: 'a',
                    pair_with: vec!['u', 'n', 'm', 'p', 't', 'c', 'y'],
                    replace_with: ('â', 'Â')
                },
                DiacriticMatch {
                    ch: 'e',
                    pair_with: vec!['u', 'n', 'm', 'p', 't', 'c', 'y'],
                    replace_with: ('ê', 'Ê')
                },
                DiacriticMatch {
                    ch: 'o',
                    pair_with: vec!['i', 'n', 'm', 'p', 't', 'c', 'y'],
                    replace_with: ('ô', 'Ô')
                }
            ]),
            TRIGGER_HORN => self.add_diacritic(vec![
                DiacriticMatch {
                    ch: 'u',
                    pair_with: vec!['o', 'i', 'n', 'm', 'a', 'p', 't', 'c'],
                    replace_with: ('ư', 'Ư')
                },
                DiacriticMatch {
                    ch: 'o',
                    pair_with: vec!['i', 'n', 'm', 'p', 't', 'c', 'y'],
                    replace_with: ('ơ', 'Ơ')
                }
            ]),
            TRIGGER_BREVE => self.add_diacritic(vec![
                DiacriticMatch {
                    ch: 'a',
                    pair_with: vec!['p', 'n', 'm', 't', 'c'],
                    replace_with: ('ă', 'Ă')
                }
            ]),
            TRIGGER_CROSSED_D => self.add_diacritic(vec![
                DiacriticMatch {
                    ch: 'd',
                    pair_with: vec!['a', 'c', 'e', 'i', 'm', 'n', 'o', 'p', 't', 'u', 'y'],
                    replace_with: ('đ', 'Đ')
                }
            ]),
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
