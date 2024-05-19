use std::fmt::Display;

use crate::{
    editing::{add_modification_char, add_tone_char, get_tone_mark_placement, replace_nth_char},
    parsing::{extract_letter_modifications, extract_tone, parse_word},
    processor::{modify_letter, LetterModification, ToneMark},
    util::clean_char,
};

pub struct Word {
    pub initial_consonant: String,
    pub vowel: String,
    pub final_consonant: String,
    pub tone_mark: Option<ToneMark>,
    pub letter_modifications: Vec<(usize, LetterModification)>,
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
        let clean_word = format!(
            "{}{}{}{}",
            self.initial_consonant, self.vowel, self.final_consonant, ch
        );
        let (_, word) = parse_word(&clean_word).unwrap();
        self.initial_consonant = word.initial_consonant.chars().map(clean_char).collect();
        self.vowel = word.vowel.chars().map(clean_char).collect();
        self.final_consonant = word.final_consonant.to_string();

        self.recalculate_modifications();
    }

    pub fn recalculate_modifications(&mut self) {
        // consonants are required to recalculate, unless it's the word uoi
        if self.initial_consonant.is_empty()
            && self.final_consonant.is_empty()
            && !self.vowel.eq_ignore_ascii_case("uoi")
        {
            return;
        }

        // Special case for uo where the reposition can only be decided when the final consonant is present
        if self.vowel.eq_ignore_ascii_case("uo")
            && !self.initial_consonant.is_empty()
            && self.final_consonant.is_empty()
        {
            return;
        }

        let mut modifications = std::mem::take(&mut self.letter_modifications);
        modifications.dedup_by_key(|(_, modifcation)| modifcation.clone());

        for (_, modification) in modifications {
            modify_letter(self, &modification);
        }
    }

    pub fn set(&mut self, raw: String) {
        let (_, word) = parse_word(&raw).unwrap();
        self.initial_consonant = word.initial_consonant.chars().map(clean_char).collect();
        self.vowel = word.vowel.chars().map(clean_char).collect();
        self.final_consonant = word.final_consonant.to_string();

        self.letter_modifications = extract_letter_modifications(&raw);
        self.tone_mark = extract_tone(&raw);
    }

    pub fn replace_last_char(&mut self, ch: char) {
        let mut raw = self.to_string();
        let last_index = raw.chars().count() - 1;
        replace_nth_char(&mut raw, last_index, ch);
        self.set(raw);
    }

    pub fn contains_modification(&self, modification: &LetterModification) -> bool {
        self.letter_modifications
            .iter()
            .any(|(_, m)| m == modification)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!(
            "{}{}{}",
            self.initial_consonant, self.vowel, self.final_consonant
        );

        for (position, modification) in &self.letter_modifications {
            let ch = result.chars().nth(*position).unwrap();
            let replace_char = add_modification_char(ch, modification);

            replace_nth_char(&mut result, *position, replace_char);
        }

        if let Some(tone_mark) = &self.tone_mark {
            let tone_mark_position = get_tone_mark_placement(&result);
            let ch = result.chars().nth(tone_mark_position).unwrap();
            let replace_char = add_tone_char(ch, tone_mark);
            replace_nth_char(&mut result, tone_mark_position, replace_char);
        }

        write!(f, "{}", result)
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
