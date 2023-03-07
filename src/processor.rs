//! The core of the engine, where all the transformation algorithms lives.
//!
//! The idea is both the telex & vni modules will use the transformation algorithms
//! from this module to perform text transformation according to their method rules.
use phf::{phf_set, Map, Set};

use super::maps::{
    ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP, HOOK_ABOVE_MAP, HORN_MAP,
    TILDE_MAP,
};
use super::util::{clean_char, remove_tone_mark};
use crate::parsing::{parse_vowel, parse_word, WordComponents};
use crate::util::{
    extract_letter_modifications, extract_tone, remove_modification, replace_nth_char,
};

/// Maximum length of a Vietnamese "word" is 7 letters long (nghiêng)
const MAX_WORD_LENGTH: usize = 7;
const SPECIAL_VOWEL_PAIRS: Set<&'static str> = phf_set!("oa", "oe", "oo", "uy", "uo");

/// A tone mark in Vietnamese
///
/// - **Acute:** Dấu sắc
/// - **Grave:** Dấu huyền
/// - **HookAbove:** Dấu hỏi
/// - **Tilde:** Dấu ngã
/// - **Underdot:** Dấu nặng
#[derive(Debug, PartialEq)]
pub enum ToneMark {
    Acute,
    Grave,
    HookAbove,
    Tilde,
    Underdot,
}

/// A modification to be apply to a letter
///
/// - **Circumflex:** The chevron shaped (ˆ) part on top of a character.
/// - **Breve:** The part that shaped like a bottom half of a circle (˘)
/// - **Horn:** The hook that attach to the character. For example, ư
/// - **Dyet:** The line that go through the character d (đ).
#[derive(Debug, PartialEq)]
pub enum LetterModification {
    Circumflex,
    Breve,
    Horn,
    Dyet,
}

/// Get nth character to place tone mark
///
/// # Rules:
/// 1. If a vowel contains ơ, tone mark goes there
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

/// Add tone mark to input
/// Return if the tone mark has been added or not
pub fn add_tone(buffer: &mut String, tone_mark: &ToneMark) -> bool {
    if buffer.chars().count() > MAX_WORD_LENGTH {
        return false;
    }

    if let Some(existing_tone_mark) = extract_tone(&buffer) {
        *buffer = buffer.chars().map(remove_tone_mark).collect();

        if existing_tone_mark == *tone_mark {
            return false;
        }
    }

    let Ok((_, components)) = parse_word(buffer) else {
        return false;
    };

    if components.vowel.is_empty() {
        return false;
    }

    let tone_mark_position = get_tone_mark_placement(&components);

    let tone_mark_ch = buffer.chars().nth(tone_mark_position).expect(&format!(
        "Unable to retrieve character at index {} from {}",
        tone_mark_position, buffer
    ));
    let replace_char = add_tone_char(tone_mark_ch, tone_mark);

    replace_nth_char(buffer, tone_mark_position, replace_char);
    true
}

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

/// change a letter to vietnamese modified letter
/// Return if the letter has been modified or not and what's the output
pub fn modify_letter(buffer: &mut String, modification: &LetterModification) -> bool {
    if buffer.chars().count() > MAX_WORD_LENGTH {
        return false;
    }

    let modifications = extract_letter_modifications(buffer);

    // Remove existing modification if it's already been added
    let raw_buffer: String = buffer.chars().map(remove_tone_mark).collect();
    for (index, existing_modification) in modifications {
        if existing_modification == *modification {
            if existing_modification == LetterModification::Horn && !raw_buffer.contains("ươ") {
                break;
            }
            let ch = buffer.chars().nth(index).unwrap();
            replace_nth_char(buffer, index, remove_modification(ch));
            return false;
        }
    }

    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    // Only d will get the Dyet modification and d is always in front
    if let LetterModification::Dyet = modification {
        let Some(first_ch) = buffer.chars().nth(0) else {
            return false;
        };
        let cleaned_ch = clean_char(first_ch);
        if map.contains_key(&cleaned_ch) {
            replace_nth_char(buffer, 0, map[&cleaned_ch]);
            return true;
        }
        return false;
    }

    let cleaned_buffer: String = buffer
        .chars()
        .map(clean_char)
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let Ok((_, vowel)) = parse_vowel(&cleaned_buffer) else {
        return false;
    };

    if vowel.is_empty() {
        return false;
    }

    fn get_map_char(buffer: &str, index: usize, map: &Map<char, char>) -> char {
        let ch = buffer.chars().nth(index).unwrap();
        if map.contains_key(&ch) {
            map[&ch]
        } else {
            map[&clean_char(ch)]
        }
    }

    if let LetterModification::Circumflex = modification {
        let index = vec![
            cleaned_buffer.find('a'),
            cleaned_buffer.find('o'),
            cleaned_buffer.find('e'),
        ]
        .into_iter()
        .max();

        if let Some(Some(index)) = index {
            let ch = get_map_char(&buffer, index, map);
            replace_nth_char(buffer, index, ch);
            return true;
        }
        return false;
    }

    if let LetterModification::Breve = modification {
        let Some(index) = cleaned_buffer.find('a') else {
            return false;
        };
        let ch = get_map_char(&buffer, index, map);
        replace_nth_char(buffer, index, ch);
        return true;
    }

    if let LetterModification::Horn = modification {
        if vowel == "oa" {
            return false;
        }

        if vowel == "uo" || vowel == "uoi" || vowel == "uou" {
            let clean_index = cleaned_buffer.find(vowel).unwrap();

            let ch = get_map_char(&buffer, clean_index, map);
            replace_nth_char(buffer, clean_index, ch);

            let ch = get_map_char(&buffer, clean_index + 1, map);
            replace_nth_char(buffer, clean_index + 1, ch);

            return true;
        }

        if let Some(index) = cleaned_buffer.find('u') {
            let ch = get_map_char(&buffer, index, map);
            replace_nth_char(buffer, index, ch);
            return true;
        }

        if let Some(index) = cleaned_buffer.find('o') {
            let ch = get_map_char(&buffer, index, map);
            replace_nth_char(buffer, index, ch);
            return true;
        }
    }

    return false;
}

/// Remove the tone for the letter
pub fn remove_tone(input: &mut String) -> bool {
    if input.chars().count() > MAX_WORD_LENGTH {
        return false;
    }
    let mut result = input.chars().map(remove_tone_mark).collect::<String>();
    if result == *input {
        result = result.chars().map(clean_char).collect();
    }
    let tone_removed = result != *input;
    *input = result;

    tone_removed
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
        let modified = modify_letter(&mut buffer, &LetterModification::Circumflex);
        let expected = "ệ";
        assert!(modified);
        assert_eq!(buffer, expected);

        let mut buffer = "Ẹ".to_string();
        let modified = modify_letter(&mut buffer, &LetterModification::Circumflex);
        let expected = "Ệ";
        assert!(modified);
        assert_eq!(buffer, expected);
    }
}
