use crate::{
    editing::{
        add_modification_char, add_tone_char, get_modification_positions, get_tone_mark_placement,
        replace_nth_char,
    },
    parsing::parse_word,
    processor::{LetterModification, ToneMark},
    util::clean_char,
};

pub struct Word {
    pub initial_consonant: String,
    pub vowel: String,
    pub final_consonant: String,
    pub tone_mark: Option<ToneMark>,
    pub letter_modifications: Vec<LetterModification>,
}

impl Word {
    pub fn empty() -> Self {
        Self {
            initial_consonant: String::new(),
            vowel: String::new(),
            final_consonant: String::new(),
            tone_mark: None,
            letter_modifications: Vec::new(),
        }
    }

    pub fn is_emtpy(&self) -> bool {
        self.initial_consonant.is_empty()
            && self.vowel.is_empty()
            && self.final_consonant.is_empty()
    }

    pub fn len(&self) -> u8 {
        self.initial_consonant.chars().count() as u8
            + self.vowel.chars().count() as u8
            + self.final_consonant.chars().count() as u8
    }

    pub fn push(&mut self, ch: char) {
        let mut raw = self.to_string();
        raw.push(ch);
        self.set(raw);
    }

    pub fn set(&mut self, raw: String) {
        let (_, word) = parse_word(&raw).unwrap();
        self.initial_consonant = word
            .initial_consonant
            .chars()
            .map(|c| clean_char(c))
            .collect();
        self.vowel = word.vowel.chars().map(|c| clean_char(c)).collect();
        self.final_consonant = word.final_consonant.to_string();
    }

    pub fn replace_last_char(&mut self, ch: char) {
        let mut raw = self.to_string();
        let last_index = raw.chars().count() - 1;
        replace_nth_char(&mut raw, last_index, ch);
        self.set(raw);
    }

    pub fn to_string(&self) -> String {
        let mut result = format!(
            "{}{}{}",
            self.initial_consonant, self.vowel, self.final_consonant
        );
        if let Some(tone_mark) = &self.tone_mark {
            let tone_mark_position = get_tone_mark_placement(&self);
            let ch = result.chars().nth(tone_mark_position).unwrap();
            let replace_char = add_tone_char(ch, tone_mark);
            replace_nth_char(&mut result, tone_mark_position, replace_char);
        }

        for modification in &self.letter_modifications {
            let modification_positions = get_modification_positions(&self, modification);

            for position in modification_positions {
                let ch = result.chars().nth(position).unwrap();
                let replace_char = add_modification_char(ch, modification);
                replace_nth_char(&mut result, position, replace_char);
            }
        }
        result
    }
}

#[cfg(test)]
impl From<&str> for Word {
    fn from(value: &str) -> Self {
        let mut word = Word::empty();
        word.set(value.to_string());
        word
    }
}
