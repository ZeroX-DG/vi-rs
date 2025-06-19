//! The core of the engine, where all the transformation algorithms lives.
//!
//! The idea is both the telex & vni modules will use the transformation algorithms
//! from this module to perform text transformation according to their method rules.
use super::maps::{BREVE_MAP, CIRCUMFLEX_MAP, DYET_MAP, HORN_MAP};
use crate::{editing::get_modification_positions, syllable::Syllable};

/// Maximum length of a Vietnamese "syllable" is 7 letters long (nghiêng)
const MAX_WORD_LENGTH: usize = 7;

/// Vietnamese tone marks.
///
/// Represents the five tone marks used in Vietnamese writing system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ToneMark {
    /// Dấu sắc (acute accent) - rising tone
    Acute,
    /// Dấu huyền (grave accent) - falling tone
    Grave,
    /// Dấu hỏi (hook above) - dipping tone
    HookAbove,
    /// Dấu ngã (tilde) - creaky rising tone
    Tilde,
    /// Dấu nặng (dot below) - creaky falling tone
    Underdot,
}

/// Determines how accent marks are placed on syllables
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AccentStyle {
    /// Old-style accent placement rules.
    Old,
    /// New-style accent placement (default).
    #[default]
    New,
}

/// A modification to be applied to a letter.
///
/// Represents the diacritical marks that modify the base form of Vietnamese letters.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LetterModification {
    /// The circumflex (ˆ) diacritic - changes a, e, o to â, ê, ô
    Circumflex,
    /// The breve (˘) diacritic - changes a to ă
    Breve,
    /// The horn diacritic - changes o, u to ơ, ư
    Horn,
    /// The stroke through d - changes d to đ
    Dyet,
}

/// Result of a transformation operation.
///
/// Indicates what happened when attempting to apply a transformation to a syllable.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Transformation {
    /// A tone mark has been successfully added to a syllable without a tone mark.
    ToneMarkAdded,
    /// A new tone mark has been placed to replace an existing tone mark.
    ToneMarkReplaced,
    /// A tone mark has been removed from the syllable.
    ToneMarkRemoved,

    /// A letter modification has been added without removing any existing modification.
    LetterModificationAdded,
    /// A letter modification has been added to replace an existing modification.
    LetterModificationReplaced,
    /// A letter modification has been removed from the syllable.
    LetterModificationRemoved,

    /// The transformation cannot be applied and has been ignored.
    Ignored,
}

/// Add tone mark to input.
///
/// Returns the result of the transformation operation.
///
/// # Examples
///
/// ```
/// use vi::{Syllable, processor::{add_tone, ToneMark, Transformation}};
///
/// let mut syllable = Syllable::new("hello");
/// let result = add_tone(&mut syllable, &ToneMark::Acute);
/// assert_eq!(result, Transformation::ToneMarkAdded);
/// ```
#[must_use]
pub fn add_tone(syllable: &mut Syllable, tone_mark: &ToneMark) -> Transformation {
    if syllable.is_empty() || syllable.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    if syllable.vowel.is_empty() {
        return Transformation::Ignored;
    }

    if let Some(existing_tone_mark) = syllable.tone_mark {
        if existing_tone_mark == *tone_mark {
            syllable.tone_mark = None;
            Transformation::ToneMarkRemoved
        } else {
            syllable.tone_mark = Some(*tone_mark);
            Transformation::ToneMarkReplaced
        }
    } else {
        syllable.tone_mark = Some(*tone_mark);
        Transformation::ToneMarkAdded
    }
}

/// Change a letter to Vietnamese modified letter.
///
/// Returns the result of the modification operation.
///
/// # Examples
///
/// ```
/// use vi::{Syllable, processor::{modify_letter, LetterModification, Transformation}};
///
/// let mut syllable = Syllable::new("a");
/// let result = modify_letter(&mut syllable, &LetterModification::Circumflex);
/// assert_eq!(result, Transformation::LetterModificationAdded);
/// ```
#[must_use]
pub fn modify_letter(syllable: &mut Syllable, modification: &LetterModification) -> Transformation {
    if syllable.is_empty() || syllable.len() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    // Remove the modification if it's already exist except for horn when two character with the same horn modification can exist
    if syllable.contains_modification(modification) {
        // Special case where you don't remove the modification but add another one on top if possible
        if let LetterModification::Horn = modification {
            let current_modifications: Vec<&(usize, LetterModification)> = syllable
                .letter_modifications
                .iter()
                .filter(|(_, modification)| *modification == LetterModification::Horn)
                .collect();
            let horn_modification_count = current_modifications.len();

            let vowel = syllable.vowel.to_lowercase();
            let vowel_index = syllable.initial_consonant.chars().count();
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
                syllable
                    .letter_modifications
                    .push((*other_modification_position, LetterModification::Horn));
                return Transformation::LetterModificationAdded;
            }
        }
        syllable.letter_modifications.remove(
            syllable
                .letter_modifications
                .iter()
                .position(|(_, m)| m == modification)
                .unwrap(),
        );
        return Transformation::LetterModificationRemoved;
    }

    // Add the modification if it's dyet (because you can't replace dyet with anything, only add or remove)
    if *modification == LetterModification::Dyet {
        if let Some(first_char) = syllable.initial_consonant.chars().next() {
            if DYET_MAP.contains_key(&first_char) {
                syllable
                    .letter_modifications
                    .push((0, LetterModification::Dyet));
                return Transformation::LetterModificationAdded;
            }
        }
        return Transformation::Ignored;
    }

    // Ignore special case
    if *modification == LetterModification::Horn && syllable.vowel.to_lowercase() == "oa" {
        return Transformation::Ignored;
    }

    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    // Add the modification if the syllable have no modification or only have dyet modification
    if syllable.letter_modifications.is_empty()
        || (syllable.letter_modifications.len() == 1
            && syllable.contains_modification(&LetterModification::Dyet))
    {
        // No letter can be transformed
        if !syllable.vowel.contains(|c| map.contains_key(&c)) {
            return Transformation::Ignored;
        }

        let positions = get_modification_positions(syllable, modification);

        if positions.is_empty() {
            return Transformation::Ignored;
        }

        for position in positions {
            syllable
                .letter_modifications
                .push((position, *modification));
        }

        return Transformation::LetterModificationAdded;
    }

    // No letter can be transformed
    if !syllable.vowel.contains(|c| map.contains_key(&c)) {
        return Transformation::Ignored;
    }

    // Otherwise replace the modification
    let positions = get_modification_positions(syllable, modification);
    syllable
        .letter_modifications
        .retain(|(_, modification)| *modification == LetterModification::Dyet);
    for position in positions {
        syllable
            .letter_modifications
            .push((position, *modification));
    }
    Transformation::LetterModificationReplaced
}

/// Remove the tone for the letter
pub fn remove_tone(input: &mut Syllable) -> Transformation {
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
