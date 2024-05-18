//! The core of the engine, where all the transformation algorithms lives.
//!
//! The idea is both the telex & vni modules will use the transformation algorithms
//! from this module to perform text transformation according to their method rules.
use std::collections::HashSet;

use phf::{phf_set, Set};

use super::maps::{
    ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP, HOOK_ABOVE_MAP, HORN_MAP,
    TILDE_MAP,
};
use super::util::{clean_char, remove_tone_mark};
use crate::parsing::{parse_word, WordComponents};
use crate::util::{
    extract_letter_modifications, extract_tone, remove_modification, replace_nth_char,
};

/// Maximum length of a Vietnamese "word" is 7 letters long (nghiêng)
const MAX_WORD_LENGTH: usize = 7;
const SPECIAL_VOWEL_PAIRS: Set<&'static str> = phf_set!("oa", "oe", "oo", "uy", "uo", "ie");

/// Vietnamese's tone mark
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq, Eq, Hash)]
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

macro_rules! replace_modification_or_ignore {
    ($buffer:ident, $modification:expr, $map:ident, $index:expr) => {
        let ch = $buffer
            .chars()
            .nth($index)
            .map(remove_modification)
            .unwrap();
        let Some(replace_ch) = $map.get(&ch).map(|ch| *ch) else {
            log::warn!(
                "Couldn't retrieve replace char for {} for {:?}",
                ch,
                $modification
            );
            return Transformation::Ignored;
        };
        replace_nth_char($buffer, $index, replace_ch);
    };
}

/// Get nth character to place tone mark
///
/// # Rules:
/// 1. If a vowel contains ơ or ê, tone mark goes there
/// 2. If a vowel contains `oa`, `oe`, `oo`, `oy`, tone mark should be on the
/// second character
/// 3. If a vowel end with 2 put it on the first one
/// 4. Else, but tone mark on second vowel character
fn get_tone_mark_placement(components: &WordComponents) -> usize {
    let vowel = components.vowel;
    let vowel_len = vowel.chars().count();
    let vowel_index = components.initial_consonant.chars().count();
    // If there's only one vowel, then it's guaranteed that the tone mark will go there
    if vowel_len == 1 {
        return vowel_index;
    }

    // If vowel contains "ơ" then tone mark goes there.
    if let Some((index, _)) = vowel.chars().enumerate().find(|(_, ch)| *ch == 'ơ') {
        return vowel_index + index;
    }

    // If vowel contains "ê" then tone mark goes there.
    if let Some((index, _)) = vowel.chars().enumerate().find(|(_, ch)| *ch == 'ê') {
        return vowel_index + index;
    }

    // If vowel contains "â" then tone mark goes there.
    if let Some((index, _)) = vowel.chars().enumerate().find(|(_, ch)| *ch == 'â') {
        return vowel_index + index;
    }

    // Special vowels require the tone mark to be placed on the second character
    let raw_vowel: String = vowel.chars().map(clean_char).collect();
    if SPECIAL_VOWEL_PAIRS
        .iter()
        .any(|pair| raw_vowel.contains(pair))
    {
        return vowel_index + 1;
    }

    // If a word end with 2 character vowel, put it on the first character
    if components.final_consonant.is_empty() && vowel_len == 2 {
        return vowel_index;
    }

    // Else, put tone mark on second vowel
    vowel_index + 1
}

/// Add tone mark to input.
/// Return AddToneResult
pub fn add_tone(buffer: &mut String, tone_mark: &ToneMark) -> Transformation {
    if buffer.is_empty() || buffer.chars().count() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    let mut tone_mark_replaced = false;

    if let Some(existing_tone_mark) = extract_tone(buffer) {
        *buffer = buffer.chars().map(remove_tone_mark).collect();

        if existing_tone_mark == *tone_mark {
            return Transformation::ToneMarkRemoved;
        }
        tone_mark_replaced = true;
    }

    let Ok((_, components)) = parse_word(buffer) else {
        return Transformation::Ignored;
    };

    if components.vowel.is_empty() {
        return Transformation::Ignored;
    }

    let tone_mark_position = get_tone_mark_placement(&components);

    let tone_mark_ch = buffer.chars().nth(tone_mark_position).unwrap_or_else(|| {
        panic!(
            "Unable to retrieve character at index {} from {}",
            tone_mark_position, buffer
        )
    });
    let replace_char = add_tone_char(tone_mark_ch, tone_mark);

    replace_nth_char(buffer, tone_mark_position, replace_char);

    if tone_mark_replaced {
        Transformation::ToneMarkReplaced
    } else {
        Transformation::ToneMarkAdded
    }
}

/// Add tone mark to input character.
/// Return a new char with the tone mark.
pub fn add_tone_char(ch: char, tone_mark: &ToneMark) -> char {
    let tone_mark_map = match tone_mark {
        ToneMark::Acute => &ACCUTE_MAP,
        ToneMark::Grave => &GRAVE_MAP,
        ToneMark::HookAbove => &HOOK_ABOVE_MAP,
        ToneMark::Tilde => &TILDE_MAP,
        ToneMark::Underdot => &DOT_MAP,
    };
    *tone_mark_map.get(&ch).unwrap_or(&ch)
}

/// change a letter to vietnamese modified letter.
/// Return if the letter has been modified or not and what's the output.
pub fn modify_letter(buffer: &mut String, modification: &LetterModification) -> Transformation {
    if buffer.is_empty() || buffer.chars().count() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }

    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    let existing_modifications = extract_letter_modifications(buffer);

    // NOTE: This could means:
    // - Valid place but overflow (aaa -> âa -> the last a is overflow)
    // - Valid place but need replace (aaw -> âw -> the â need to be replaced with ă)
    let is_modificable_char_present = buffer.contains(|c| map.contains_key(&c));

    // - No valid place to modify
    let is_modification_impossible =
        !is_modificable_char_present && !buffer.contains(|c| map.contains_key(&clean_char(c)));

    if is_modification_impossible {
        return Transformation::Ignored;
    }

    // Modification overflow is when a modification cannot be applied since it's already been applied.
    let is_modification_overflow = existing_modifications
        .iter()
        .any(|(_, existing_modification)| existing_modification == modification);

    let mut modification_replaced_index = None;

    if !is_modificable_char_present {
        existing_modifications
            .iter()
            .filter(|(_, existing_modification)| existing_modification == modification)
            .for_each(|(index, _)| {
                modification_replaced_index = Some(*index);
                let ch = buffer.chars().nth(*index).map(remove_modification).unwrap();
                replace_nth_char(buffer, *index, ch);
            });

        if is_modification_overflow {
            return Transformation::LetterModificationRemoved;
        }
    }

    // Only d will get the Dyet modification and d is always at the start
    // If there's no d, we'll ignore this transformation.
    if let LetterModification::Dyet = modification {
        replace_modification_or_ignore!(buffer, modification, map, 0);
        if let Some(0) = modification_replaced_index {
            return Transformation::LetterModificationReplaced;
        }
        return Transformation::LetterModificationAdded;
    }

    let cleaned_buffer: String = buffer
        .chars()
        .map(clean_char)
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let Ok((_, word)) = parse_word(&cleaned_buffer) else {
        return Transformation::Ignored;
    };

    let vowel = word.vowel;
    let initial_consonant = word.initial_consonant;
    let final_consonant = word.final_consonant;

    if vowel.is_empty() {
        return Transformation::Ignored;
    }

    if let LetterModification::Circumflex = modification {
        let indexes = [
            cleaned_buffer.find('a'),
            cleaned_buffer.find('o'),
            cleaned_buffer.find('e'),
        ]
        .iter()
        .filter_map(|index| *index)
        .collect::<Vec<usize>>();

        // There has to be exactly 1 character that is valid for circumflex. Never 2 or more.
        if indexes.len() != 1 {
            return Transformation::Ignored;
        }

        let index = *indexes.first().unwrap();

        replace_modification_or_ignore!(buffer, modification, map, index);
        if let Some(replace_index) = modification_replaced_index {
            if replace_index == index {
                return Transformation::LetterModificationReplaced;
            }
        }
        return Transformation::LetterModificationAdded;
    }

    if let LetterModification::Breve = modification {
        let Some(index) = cleaned_buffer.find('a') else {
            return Transformation::Ignored;
        };

        replace_modification_or_ignore!(buffer, modification, map, index);
        if let Some(replace_index) = modification_replaced_index {
            if replace_index == index {
                return Transformation::LetterModificationReplaced;
            }
        }
        return Transformation::LetterModificationAdded;
    }

    if let LetterModification::Horn = modification {
        if vowel == "oa" {
            return Transformation::Ignored;
        }

        if vowel == "uo" && !initial_consonant.is_empty() && final_consonant.is_empty() {
            let index = cleaned_buffer.find(vowel).unwrap();

            replace_modification_or_ignore!(buffer, modification, map, index + 1);
            return Transformation::LetterModificationAdded;
        }

        if vowel == "uo" || vowel == "uoi" || vowel == "uou" {
            let index = cleaned_buffer.find(vowel).unwrap();

            replace_modification_or_ignore!(buffer, modification, map, index);
            replace_modification_or_ignore!(buffer, modification, map, index + 1);

            if let Some(replace_index) = modification_replaced_index {
                if replace_index == index || replace_index == index + 1 {
                    return Transformation::LetterModificationReplaced;
                }
            }
            return Transformation::LetterModificationAdded;
        }

        if let Some(vowel_relative_index) = vowel.find('u').or(vowel.find('o')) {
            let Some(vowel_index) = cleaned_buffer.find(vowel) else {
                return Transformation::Ignored;
            };
            let index = vowel_index + vowel_relative_index;

            replace_modification_or_ignore!(buffer, modification, map, index);

            if let Some(replace_index) = modification_replaced_index {
                if replace_index == index {
                    return Transformation::LetterModificationReplaced;
                }
            }
            return Transformation::LetterModificationAdded;
        }
    }

    Transformation::Ignored
}

/// Re-position existing tone mark to a valid position
pub fn reposition_tone_mark(buffer: &mut String) {
    if let Some(existing_tone_mark) = extract_tone(buffer) {
        *buffer = buffer.chars().map(remove_tone_mark).collect();
        add_tone(buffer, &existing_tone_mark);
    }
}

/// Re-position existing letter modification to a valid position
pub fn reposition_letter_modification(buffer: &mut String) {
    let Ok((_, word)) = parse_word(buffer) else {
        return;
    };

    let cleaned_vowel: String = word
        .vowel
        .chars()
        .map(clean_char)
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if word.initial_consonant.is_empty()
        && word.final_consonant.is_empty()
        && cleaned_vowel != "uoi"
    {
        return;
    }

    // Special case for ưo where the reposition can only be decided when the final consonant is present
    if word.vowel.eq_ignore_ascii_case("ưo")
        && !word.initial_consonant.is_empty()
        && word.final_consonant.is_empty()
    {
        return;
    }

    let existing_modifications = extract_letter_modifications(buffer)
        .into_iter()
        .map(|(_, modification)| modification)
        .collect::<HashSet<LetterModification>>();

    *buffer = buffer.chars().map(remove_modification).collect();

    for modification in existing_modifications {
        modify_letter(buffer, &modification);
    }
}

/// Remove the tone for the letter
pub fn remove_tone(input: &mut String) -> Transformation {
    if input.chars().count() > MAX_WORD_LENGTH {
        return Transformation::Ignored;
    }
    let mut result = input.chars().map(remove_tone_mark).collect::<String>();
    if result == *input {
        result = result.chars().map(clean_char).collect();
    }
    let tone_removed = result != *input;
    *input = result;

    if tone_removed {
        Transformation::ToneMarkRemoved
    } else {
        Transformation::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tone_mark_placement_normal() {
        let (_, components) = parse_word("choe").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_special() {
        let (_, components) = parse_word("chieu").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_mid_not_end() {
        let (_, components) = parse_word("hoang").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_u_and_o() {
        let (_, components) = parse_word("ngươi").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_uppercase() {
        let (_, components) = parse_word("chÊt").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 2;
        assert_eq!(result, expected);

        let (_, components) = parse_word("chiÊt").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 3;
        assert_eq!(result, expected);

        let (_, components) = parse_word("cAu").unwrap();
        let result = get_tone_mark_placement(&components);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_existing_tone_mark() {
        let mut buffer = "ẹ".to_string();
        let transformation = modify_letter(&mut buffer, &LetterModification::Circumflex);
        let expected = "ệ";
        assert!(transformation == Transformation::LetterModificationAdded);
        assert_eq!(buffer, expected);

        let mut buffer = "Ẹ".to_string();
        let transformation = modify_letter(&mut buffer, &LetterModification::Circumflex);
        let expected = "Ệ";
        assert!(transformation == Transformation::LetterModificationAdded);
        assert_eq!(buffer, expected);
    }
}
