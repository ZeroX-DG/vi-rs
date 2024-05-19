//! The core of the engine, where all the transformation algorithms lives.
//!
//! The idea is both the telex & vni modules will use the transformation algorithms
//! from this module to perform text transformation according to their method rules.
use super::maps::{BREVE_MAP, CIRCUMFLEX_MAP, DYET_MAP, HORN_MAP};
use crate::{editing::get_modification_positions, word::Word};

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

    if word.vowel.is_empty() {
        return Transformation::Ignored;
    }

    if let Some(existing_tone_mark) = word.tone_mark.clone() {
        if existing_tone_mark == *tone_mark {
            word.tone_mark = None;
            Transformation::ToneMarkRemoved
        } else {
            word.tone_mark = Some(tone_mark.clone());
            Transformation::ToneMarkReplaced
        }
    } else {
        word.tone_mark = Some(tone_mark.clone());
        Transformation::ToneMarkAdded
    }
}

/// change a letter to vietnamese modified letter.
/// Return if the letter has been modified or not and what's the output.
pub fn modify_letter(word: &mut Word, modification: &LetterModification) -> Transformation {
    if word.is_emtpy() || word.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    // Remove the modification if it's already exist except for horn when two character with the same horn modification can exist
    if word.contains_modification(modification) {
        // Special case where you don't remove the modification but add another one on top if possible
        if let LetterModification::Horn = modification {
            let current_modifications: Vec<&(usize, LetterModification)> = word
                .letter_modifications
                .iter()
                .filter(|(_, modification)| *modification == LetterModification::Horn)
                .collect();
            let horn_modification_count = current_modifications.len();

            let vowel = word.vowel.to_lowercase();
            let vowel_index = word.initial_consonant.chars().count();
            let modification_possibilities: Vec<usize> = [vowel.find('u'), vowel.find('o')]
                .iter()
                .filter_map(|index| index.map(|index| vowel_index + index))
                .collect();

            let max_horn_modification_possible = modification_possibilities.len();

            if horn_modification_count < max_horn_modification_possible {
                let other_modification_position = modification_possibilities
                    .iter()
                    .find(|index| {
                        current_modifications
                            .iter()
                            .any(|(current_index, _)| *current_index != **index)
                    })
                    .unwrap();
                word.letter_modifications
                    .push((*other_modification_position, LetterModification::Horn));
                return Transformation::LetterModificationAdded;
            }
        }
        word.letter_modifications.remove(
            word.letter_modifications
                .iter()
                .position(|(_, m)| m == modification)
                .unwrap(),
        );
        return Transformation::LetterModificationRemoved;
    }

    // Add the modification if it's dyet (because you can't replace dyet with anything, only add or remove)
    if *modification == LetterModification::Dyet {
        if let Some(first_char) = word.initial_consonant.chars().next() {
            if DYET_MAP.contains_key(&first_char) {
                word.letter_modifications
                    .push((0, LetterModification::Dyet));
                return Transformation::LetterModificationAdded;
            }
        }
        return Transformation::Ignored;
    }

    // Ignore special case
    if *modification == LetterModification::Horn && word.vowel.to_lowercase() == "oa" {
        return Transformation::Ignored;
    }

    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    // Add the modification if the word have no modification or only have dyet modification
    if word.letter_modifications.is_empty()
        || (word.letter_modifications.len() == 1
            && word.contains_modification(&LetterModification::Dyet))
    {
        // No letter can be transformed
        if !word.vowel.contains(|c| map.contains_key(&c)) {
            return Transformation::Ignored;
        }

        let positions = get_modification_positions(word, modification);

        if positions.is_empty() {
            return Transformation::Ignored;
        }

        for position in positions {
            word.letter_modifications
                .push((position, modification.clone()));
        }

        return Transformation::LetterModificationAdded;
    }

    // No letter can be transformed
    if !word.vowel.contains(|c| map.contains_key(&c)) {
        return Transformation::Ignored;
    }

    // Otherwise replace the modification
    let positions = get_modification_positions(word, modification);
    word.letter_modifications
        .retain(|(_, modification)| *modification == LetterModification::Dyet);
    for position in positions {
        word.letter_modifications
            .push((position, modification.clone()));
    }
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
