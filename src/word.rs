//! The cache for word transformation.
//! 
//! Since vi-rs work by looping through a list of character & apply transformation on a word,
//! it's much more beneficial to store the current state of the word as a struct rather than
//! a string so it doesn't need to be parsed everytime a transformation is applied. 
//! 
//! Normally you'd start by constructing an empty word at the start of the process,
//! and then perform various manipulations on the words. Afterwards, you can call `to_string()`
//! to retrieve a String value representing the final state of the word.
//! 
//! ## Example:
//! 
//! ```
//! use vi::word::Word;
//! use vi::processor::{modify_letter, add_tone, LetterModification, ToneMark};
//! 
//! let mut word = Word::empty();
//! word.push('t');
//! word.push('u');
//! word.push('y');
//! word.push('e');
//! word.push('t');
//! 
//! modify_letter(&mut word, &LetterModification::Circumflex);
//! add_tone(&mut word, &ToneMark::Acute);
//! 
//! println!("{}", word); // tuyết
//! 
//! ```
use std::fmt::Display;

use crate::{
    editing::{add_modification_char, add_tone_char, get_tone_mark_placement, replace_nth_char},
    parsing::{extract_letter_modifications, extract_tone, parse_word},
    processor::{modify_letter, LetterModification, ToneMark},
    util::clean_char,
};

/// Represent a word that is being transformed. This is so the word doesn't need to be re-parsed
/// during transformation stage. After all transformation is applied, the final state of the word
/// can be retreieved via the `to_string` method.
pub struct Word {
    /// The initial consonant of the word. This is always a clean text with no transformation applied.
    pub initial_consonant: String,
    /// The vowel of the word. This is always a clean text with no transformation applied.
    pub vowel: String,
    /// The final consonant of the word. This is always a clean text with no transformation applied.
    pub final_consonant: String,
    /// The tone mark of the word. This could be empty for word with no tone mark or "thanh ngang".
    pub tone_mark: Option<ToneMark>,
    /// Letter modifications on the word, along with the index that the modification is applying to.
    pub letter_modifications: Vec<(usize, LetterModification)>,
}

impl Word {
    /// Construct an empty word
    pub fn empty() -> Self {
        Self {
            initial_consonant: String::new(),
            vowel: String::new(),
            final_consonant: String::new(),
            tone_mark: None,
            letter_modifications: Vec::new(),
        }
    }

    /// The length of the word in characters instead of bytes.
    pub fn len(&self) -> usize {
        self.initial_consonant.chars().count()
            + self.vowel.chars().count()
            + self.final_consonant.chars().count()
    }

    /// Indicate whether the word have no initial consonant, vowel & final consonant.
    pub fn is_empty(&self) -> bool {
        self.initial_consonant.is_empty()
            && self.vowel.is_empty()
            && self.final_consonant.is_empty()
    }

    /// Push a character to the word. This will also trigger modification recalculation for the word.
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

    /// Recalculate the position of the modification for the current word.
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

    /// Set a new value for the current word. This will parse the value into consonants, vowel, tonemark & modifications.
    pub fn set(&mut self, raw: String) {
        let (_, word) = parse_word(&raw).unwrap();
        self.initial_consonant = word.initial_consonant.chars().map(clean_char).collect();
        self.vowel = word.vowel.chars().map(clean_char).collect();
        self.final_consonant = word.final_consonant.to_string();

        self.letter_modifications = extract_letter_modifications(&raw);
        self.tone_mark = extract_tone(&raw);
    }

    /// Replace the last character in the string to some other character.
    pub fn replace_last_char(&mut self, ch: char) {
        let mut raw = self.to_string();
        let last_index = raw.chars().count() - 1;
        replace_nth_char(&mut raw, last_index, ch);
        self.set(raw);
    }

    /// Indicate whether a modification exist in the word
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
