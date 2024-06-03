//! Functions used for character editing.
//!
//! These functions work directly with character & string instead of the abstract word struct.
use crate::{
    maps::{
        ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP, HOOK_ABOVE_MAP,
        HORN_MAP, TILDE_MAP,
    },
    parsing::parse_word,
    processor::{LetterModification, ToneMark},
    word::Word,
};

const SPECIAL_VOWEL_PAIRS: [&str; 6] = ["oa", "oe", "oo", "uy", "uo", "ie"];

/// Get nth character to place tone mark
///
/// # Rules:
/// 1. If a vowel contains ơ or ê, tone mark goes there
/// 2. If a vowel contains `oa`, `oe`, `oo`, `oy`, tone mark should be on the
/// second character
/// 3. If a vowel end with 2 put it on the first one
/// 4. Else, but tone mark on second vowel character
pub fn get_tone_mark_placement(raw_word: &str) -> usize {
    let (_, word) = parse_word(raw_word).unwrap();
    let vowel = &word.vowel;
    let vowel_len = vowel.chars().count();
    let vowel_index = word.initial_consonant.chars().count();
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
    if SPECIAL_VOWEL_PAIRS.iter().any(|pair| vowel.contains(pair)) {
        return vowel_index + 1;
    }

    // If a word end with 2 character vowel, put it on the first character
    if word.final_consonant.is_empty() && vowel_len == 2 {
        return vowel_index;
    }

    // Else, put tone mark on second vowel
    vowel_index + 1
}

/// Replace a character at an index in an input char.
///
/// Note: It's character index, not byte index.
pub fn replace_nth_char(input: &mut String, replace_index: usize, replace_ch: char) {
    *input = input
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            if index == replace_index {
                replace_ch
            } else {
                ch
            }
        })
        .collect();
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

/// Add modification to input character.
/// Return a modified character.
pub fn add_modification_char(ch: char, modification: &LetterModification) -> char {
    let modification_map = match modification {
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
        LetterModification::Horn => &HORN_MAP,
    };
    *modification_map.get(&ch).unwrap_or(&ch)
}

/// Get index of the characters to modify
///
/// # Rules:
/// 1. If the modification is Dyet, it's always the first letter.
/// 2. If the modification is Circumflex, it's always on a, o, or e, which ever come first.
/// 3. If the modification is Breve, it's always on a.
/// 4. If the modification is Horn:
///     a. if the vowel is oa, ignore
///     b. if the vowel is uo & only the initial consonant is present, then it's on the o
///     c. if the vowel is uo, uoi or uou, then it's on the first two chars
///     d. if the vowel contains u then it's on u, otherwise if it contains o then it's on o
pub fn get_modification_positions(word: &Word, modification: &LetterModification) -> Vec<usize> {
    if let LetterModification::Dyet = modification {
        return vec![0];
    }

    let vowel_index = word.initial_consonant.chars().count();

    let vowel = word.vowel.to_lowercase();

    if let LetterModification::Circumflex = modification {
        let indexes = [vowel.find('a'), vowel.find('o'), vowel.find('e')]
            .iter()
            .filter_map(|index| *index)
            .collect::<Vec<usize>>();

        // There has to be exactly 1 character that is valid for circumflex. Never 2 or more.
        if indexes.len() != 1 {
            return Vec::new();
        }

        let index = *indexes.first().unwrap();
        return vec![vowel_index + index];
    }

    if let LetterModification::Breve = modification {
        let Some(index) = vowel.find('a') else {
            return Vec::new();
        };
        return vec![vowel_index + index];
    }

    if let LetterModification::Horn = modification {
        if vowel == "oa" {
            return Vec::new();
        }

        if vowel == "uo" && !word.initial_consonant.is_empty() && word.final_consonant.is_empty() {
            return vec![vowel_index + 1];
        }

        if vowel == "uo" || vowel == "uoi" || vowel == "uou" {
            return vec![vowel_index, vowel_index + 1];
        }

        if let Some(index) = vowel.find('u').or(vowel.find('o')) {
            return vec![vowel_index + index];
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tone_mark_placement_normal() {
        let result = get_tone_mark_placement("choe");
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_special() {
        let result = get_tone_mark_placement("chieu");
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_mid_not_end() {
        let result = get_tone_mark_placement("hoang");
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_u_and_o() {
        let result = get_tone_mark_placement("ngươi");
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_uppercase() {
        let result = get_tone_mark_placement("chÊt");
        let expected = 2;
        assert_eq!(result, expected);

        let result = get_tone_mark_placement("chiÊt");
        let expected = 3;
        assert_eq!(result, expected);

        let result = get_tone_mark_placement("cAu");
        let expected = 1;
        assert_eq!(result, expected);
    }
}
