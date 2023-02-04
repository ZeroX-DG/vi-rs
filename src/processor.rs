use crate::parsing::parse_vowel;
use crate::util::{
    extract_letter_modification, get_char_at, get_next_char_index, is_modifiable_vowels,
    is_modified_vowels, is_vowel_with_accent,
};

use super::maps::{
    ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP, HOOK_ABOVE_MAP, HORN_MAP,
    TILDE_MAP,
};
use super::util::{clean_char, remove_tone_mark};

/// Maximum length of a Vietnamese "word" is 7 letters long (nghiêng)
const MAX_WORD_LENGTH: usize = 7;

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

/// Get position to place tone mark
///
/// # Rules:
/// 1. Tone mark always above vowel (a, ă, â, e, ê, i, o, ô, ơ, u, ư, y)
/// 2. If a word contains ơ, tone mark goes there
/// 3. If a modified letter goes with a non-modified vowel, tone mark should be
/// on modifed letter
/// 4. If a word contains `oa`, `oe`, `oo`, `oy`, tone mark should be on the
/// second vowel
/// 5. If a word end with 2 or 3 vowel, put it on the second last one
/// 6. Else, but tone mark on whatever vowel comes first
fn get_tone_mark_placement(input: &str) -> Option<usize> {
    let Ok((_, vowel)) = parse_vowel(input) else {
        return None;
    };

    if vowel.is_empty() {
        return None;
    }

    input.find(vowel).map(|index| {
        let vowel_len = vowel.chars().count();
        // If there's only one vowel, then it's guaranteed that the tone mark will go there
        if vowel_len == 1 {
            return index;
        }

        // If vowel already contains a letter with tone mark. Use that letter's position
        if let Some((offset, _)) = vowel
            .chars()
            .enumerate()
            .find(|(_, ch)| is_vowel_with_accent(*ch))
        {
            return index + offset;
        }

        // If vowel contains "ơ" then tone mark goes there.
        if let Some(pos) = input.find('ơ') {
            return pos;
        }

        // If there's a modified vowels then tone mark goes there.
        if let Some(pos) = input.find(is_modified_vowels) {
            return pos;
        }

        // If a word contains `oa`, `oe`, `oo`, `oy`, tone mark should be on the second vowel
        for pair in ["oa", "oe", "oo", "uy"].iter() {
            if let Some(pos) = input.find(pair) {
                return get_next_char_index(input, pos);
            }
        }

        // If a word end with 2 or 3 vowel, put it on the second last one
        let is_end_with_vowel = input.len() == index + vowel_len;
        if is_end_with_vowel && vowel_len >= 2 {
            return index + vowel.char_indices().nth(vowel_len - 2).unwrap().0;
        }

        // If there's a modifiable vowels then tone mark goes there.
        if let Some(pos) = input.find(is_modifiable_vowels) {
            return pos;
        }

        // Else, but tone mark on whatever vowel comes first
        index
    })
}

fn replace_char_at(input: &mut String, index: usize, ch: char) {
    let range = input
        .char_indices()
        .find(|(pos, _)| *pos == index)
        .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
        .unwrap();
    input.replace_range(range, &ch.to_string());
}

/// Add tone mark to input
/// Return if the tone mark has been added or not
pub fn add_tone(buffer: &mut String, tone_mark: &ToneMark) -> bool {
    if buffer.chars().count() > MAX_WORD_LENGTH {
        return false;
    }

    let Some(tone_mark_position) = get_tone_mark_placement(buffer) else {
        return false;
    };
    let tone_mark_ch = get_char_at(buffer, tone_mark_position).unwrap();

    let tone_mark_map = match tone_mark {
        ToneMark::Acute => &ACCUTE_MAP,
        ToneMark::Grave => &GRAVE_MAP,
        ToneMark::HookAbove => &HOOK_ABOVE_MAP,
        ToneMark::Tilde => &TILDE_MAP,
        ToneMark::Underdot => &DOT_MAP,
    };
    *buffer = buffer.chars().map(remove_tone_mark).collect();
    // Tone mark already existed. Only remove tone mark & do nothing else.
    if tone_mark_map
        .values()
        .find(|ch| **ch == tone_mark_ch)
        .is_some()
    {
        return false;
    }

    let tone_mark_ch = get_char_at(buffer, tone_mark_position).unwrap();
    let replace_char = tone_mark_map.get(&tone_mark_ch).unwrap_or(&tone_mark_ch);
    replace_char_at(buffer, tone_mark_position, *replace_char);
    true
}

/// change a letter to vietnamese modified letter
/// Return if the letter has been modified or not and what's the output
pub fn modify_letter(buffer: &mut String, modification: &LetterModification) -> bool {
    if buffer.chars().count() > MAX_WORD_LENGTH {
        return false;
    }
    let map = match modification {
        LetterModification::Horn => &HORN_MAP,
        LetterModification::Breve => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet => &DYET_MAP,
    };

    let mut is_existing_modification = false;
    let mut modified = false;
    let mut last_index = 0;

    if let Some(existing_modification) = extract_letter_modification(buffer) {
        if existing_modification == *modification {
            is_existing_modification = true;
        }
    }

    while let Some((index, ch)) = buffer
        .char_indices()
        .skip_while(|(index, _)| *index < last_index)
        .next()
    {
        last_index = index + 1;

        let cleaned_ch = clean_char(ch);
        if is_existing_modification == true {
            modified = false;
            replace_char_at(buffer, index, cleaned_ch);
        } else if is_modified_vowels(ch) && map.contains_key(&cleaned_ch) {
            dbg!(2);
            modified = true;
            replace_char_at(buffer, index, map[&cleaned_ch]);
        } else if map.contains_key(&ch) {
            dbg!(3);
            modified = true;
            replace_char_at(buffer, index, map[&ch]);
        }
    }
    modified
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
        let result = get_tone_mark_placement("choe");
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_special() {
        let result = get_tone_mark_placement("chieu");
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_mid_not_end() {
        let result = get_tone_mark_placement("hoang");
        let expected: Option<usize> = Some(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_u_and_o() {
        let result = get_tone_mark_placement("ngươi");
        let expected: Option<usize> = Some(4);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_uppercase() {
        let result = get_tone_mark_placement("chÊt");
        let expected: Option<usize> = Some(2);
        assert_eq!(result, expected);

        let result = get_tone_mark_placement("chiÊt");
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);

        let result = get_tone_mark_placement("cAu");
        let expected: Option<usize> = Some(1);
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
