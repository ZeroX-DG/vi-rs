//! The cache for syllable transformation.
//!
//! Since vi-rs work by looping through a list of character & apply transformation on a syllable,
//! it's much more beneficial to store the current state of the syllable as a struct rather than
//! a string so it doesn't need to be parsed everytime a transformation is applied.
//!
//! Normally you'd start by constructing an empty syllable at the start of the process,
//! and then perform various manipulations on the syllables. Afterwards, you can call `to_string()`
//! to retrieve a String value representing the final state of the syllable.
//!
//! ## Example:
//!
//! ```
//! use vi::syllable::Syllable;
//! use vi::processor::{modify_letter, add_tone, LetterModification, ToneMark};
//!
//! let mut syllable = Syllable::default();
//! syllable.push('t');
//! syllable.push('u');
//! syllable.push('y');
//! syllable.push('e');
//! syllable.push('t');
//!
//! modify_letter(&mut syllable, &LetterModification::Circumflex);
//! add_tone(&mut syllable, &ToneMark::Acute);
//!
//! println!("{}", syllable); // tuyết
//!
//! ```
use std::fmt::Display;

use crate::{
    editing::{add_modification_char, add_tone_char, get_tone_mark_placement, replace_nth_char},
    parsing::{extract_letter_modifications, extract_tone, parse_syllable},
    processor::{modify_letter, AccentStyle, LetterModification, ToneMark},
    util::clean_char,
};

/// Represent a syllable that is being transformed. This is so the syllable doesn't need to be re-parsed
/// during transformation stage. After all transformation is applied, the final state of the syllable
/// can be retreieved via the `to_string` method.
#[derive(Default)]
pub struct Syllable {
    /// The initial consonant of the syllable. This is always a clean text with no transformation applied.
    pub initial_consonant: String,
    /// The vowel of the syllable. This is always a clean text with no transformation applied.
    pub vowel: String,
    /// The final consonant of the syllable. This is always a clean text with no transformation applied.
    pub final_consonant: String,
    /// The tone mark of the syllable. This could be empty for syllable with no tone mark or "thanh ngang".
    pub tone_mark: Option<ToneMark>,
    /// The accent style used when rendering the syllable. Default to the [`AccentStyle::New`]
    pub accent_style: AccentStyle,
    /// Letter modifications on the syllable, along with the index that the modification is applying to.
    pub letter_modifications: Vec<(usize, LetterModification)>,
}

impl Syllable {
    /// The length of the syllable in characters instead of bytes.
    pub fn len(&self) -> usize {
        self.initial_consonant.chars().count()
            + self.vowel.chars().count()
            + self.final_consonant.chars().count()
    }

    /// Indicate whether the syllable have no initial consonant, vowel & final consonant.
    pub fn is_empty(&self) -> bool {
        self.initial_consonant.is_empty()
            && self.vowel.is_empty()
            && self.final_consonant.is_empty()
    }

    /// Push a character to the syllable. This will also trigger modification recalculation for the syllable.
    pub fn push(&mut self, ch: char) {
        let clean_syllable = format!(
            "{}{}{}{}",
            self.initial_consonant, self.vowel, self.final_consonant, ch
        );
        let (_, syllable) = parse_syllable(&clean_syllable).unwrap();
        self.initial_consonant = syllable.initial_consonant.chars().map(clean_char).collect();
        self.vowel = syllable.vowel.chars().map(clean_char).collect();
        self.final_consonant = syllable.final_consonant.to_string();

        self.recalculate_modifications();
    }

    /// Recalculate the position of the modification for the current syllable.
    pub fn recalculate_modifications(&mut self) {
        // consonants are required to recalculate, unless it's the syllable uoi
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

    /// Set a new value for the current syllable. This will parse the value into consonants, vowel, tonemark & modifications.
    pub fn set(&mut self, raw: String) {
        let (_, syllable) = parse_syllable(&raw).unwrap();
        self.initial_consonant = syllable.initial_consonant.chars().map(clean_char).collect();
        self.vowel = syllable.vowel.chars().map(clean_char).collect();
        self.final_consonant = syllable.final_consonant.to_string();

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

    /// Indicate whether a modification exist in the syllable
    pub fn contains_modification(&self, modification: &LetterModification) -> bool {
        self.letter_modifications
            .iter()
            .any(|(_, m)| m == modification)
    }
}

impl Display for Syllable {
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
            let tone_mark_position = get_tone_mark_placement(&result, &self.accent_style);
            let ch = result.chars().nth(tone_mark_position).unwrap();
            let replace_char = add_tone_char(ch, tone_mark);
            replace_nth_char(&mut result, tone_mark_position, replace_char);
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
impl From<&str> for Syllable {
    fn from(value: &str) -> Self {
        let mut syllable = Syllable::default();
        syllable.set(value.to_string());
        syllable
    }
}
