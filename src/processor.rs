//! The core of the engine, where all the transformation algorithms lives.
//!
//! The idea is both the telex & vni modules will use the transformation algorithms
//! from this module to perform text transformation according to their method rules.
use super::maps::{BREVE_MAP, CIRCUMFLEX_MAP, DYET_MAP, HORN_MAP};
use crate::word::Word;

/// Maximum length of a Vietnamese "word" is 7 letters long (nghiêng)
const MAX_WORD_LENGTH: u8 = 7;

/// Vietnamese's tone mark
#[derive(Debug, PartialEq, Clone)]
pub enum ToneMark {
    /// Dấu sắc
    Acute,
    /// Dấu huyền
    Grave,
    /// Dấu hỏi
    HookAbove,
    /// Dấu ngã
    Tilde,
    /// Dấu nặng
    Underdot,
}

/// A modification to be apply to a letter
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LetterModification {
    /// The chevron shaped (ˆ) part on top of a character.
    Circumflex,
    /// The part that shaped like a bottom half of a circle (˘)
    Breve,
    /// The hook that attach to the character. For example, ư
    Horn,
    /// The line that go through the character d (đ).
    Dyet,
}

/// A resulted transformation
#[derive(Debug, PartialEq, Clone)]
pub enum Transformation {
    /// A tone mark has been successfully added on a "tone-less" word.
    ToneMarkAdded,
    /// A new tone mark has been placed to replace an existing tone mark.
    ToneMarkReplaced,
    /// A tone mark has been removed from the word
    ToneMarkRemoved,

    /// A letter modification has been added on a word without removing any existing modification.
    LetterModificationAdded,
    /// A letter modification has been added to replace an existing modification.
    LetterModificationReplaced,
    /// A letter modification has been removed from the word
    LetterModificationRemoved,

    /// The transformation cannot be applied and has been ignored
    Ignored,
}

/// Add tone mark to input.
/// Return AddToneResult
pub fn add_tone(word: &mut Word, tone_mark: &ToneMark) -> Transformation {
    if word.is_emtpy() || word.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    if let Some(existing_tone_mark) = word.tone_mark.clone() {
        if existing_tone_mark == *tone_mark {
            word.tone_mark = None;
            return Transformation::ToneMarkRemoved;
        } else {
            word.tone_mark = Some(tone_mark.clone());
            return Transformation::ToneMarkReplaced;
        }
    } else {
        word.tone_mark = Some(tone_mark.clone());
        return Transformation::ToneMarkAdded;
    }
}

/// change a letter to vietnamese modified letter.
/// Return if the letter has been modified or not and what's the output.
pub fn modify_letter(word: &mut Word, modification: &LetterModification) -> Transformation {
    if word.is_emtpy() || word.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    // Remove the modification if it's already exist
    if word.letter_modifications.contains(modification) {
        word.letter_modifications.remove(word.letter_modifications.iter().position(|m| m == modification).unwrap());
        return Transformation::LetterModificationRemoved;
    }

    // Add the modification if it's dyet (because you can't replace dyet with anything, only add or remove)
    if *modification == LetterModification::Dyet {
        if let Some(first_char) = word.initial_consonant.chars().nth(0) {
            if DYET_MAP.contains_key(&first_char) {
                word.letter_modifications.push(LetterModification::Dyet);
                return Transformation::LetterModificationAdded;
            }
        }
        return Transformation::Ignored;
    }

    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    // Add the modification if the word have no modification or only have dyet modification
    if word.letter_modifications.is_empty() || word.letter_modifications == [LetterModification::Dyet] {
        // No letter can be transformed
        if !word.vowel.contains(|c| map.contains_key(&c)) {
            return Transformation::Ignored;
        }
        word.letter_modifications.push(modification.clone());
        return Transformation::LetterModificationAdded;
    }

    // No letter can be transformed
    if !word.vowel.contains(|c| map.contains_key(&c)) {
        return Transformation::Ignored;
    }

    // Otherwise replace the modification
    word.letter_modifications.retain(|modification| *modification == LetterModification::Dyet);
    word.letter_modifications.push(modification.clone());
    Transformation::LetterModificationReplaced
}


/// Remove the tone for the letter
pub fn remove_tone(input: &mut Word) -> Transformation {
    if input.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    let tone_removed = input.tone_mark.is_some();
    input.tone_mark = None;

    if tone_removed {
        Transformation::ToneMarkRemoved
    } else {
        Transformation::Ignored
    }
}
