use phf::{phf_set, Set};

use crate::{
    maps::{
        ACCENT_VOWELS, ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP,
        HOOK_ABOVE_MAP, HORN_MAP, TILDE_MAP, VOWELS,
    },
    parsing::parse_vowel,
    processor::{add_tone_char, modify_letter, LetterModification, ToneMark},
};

pub fn clean_char(ch: char) -> char {
    let is_uppercase = ch.is_uppercase();
    let accents = vec![
        "aàảãáạăằẳẵắặâầẩẫấậ",
        "dđ",
        "eèẻẽéẹêềểễếệ",
        "iìỉĩíị",
        "oòỏõóọôồổỗốộơờởỡớợ",
        "uùủũúụưừửữứự",
        "yỳỷỹýỵ",
    ];
    let ch_lowercase = ch.to_lowercase().to_string();
    let mut result = ch;
    for accent in accents {
        if accent.contains(&ch_lowercase) {
            result = accent.chars().next().unwrap();
        }
    }

    if is_uppercase {
        result = result.to_ascii_uppercase();
    }

    result
}

pub fn remove_tone_mark(ch: char) -> char {
    let is_uppercase = ch.is_uppercase();
    let ch_lowercase = ch.to_lowercase().next().unwrap();

    let Some(ch_index) = VOWELS.get_index(&ch_lowercase) else {
        return ch;
    };
    let reset_index = ch_index - ch_index % 6;
    let mut result = *VOWELS.index(reset_index).unwrap();

    if is_uppercase {
        result = result.to_uppercase().next().unwrap();
    }

    result
}

pub fn remove_modification(ch: char) -> char {
    let clean_ch = clean_char(ch);

    match extract_tone_char(ch) {
        Some(tone_mark) => add_tone_char(clean_ch, &tone_mark),
        None => clean_ch,
    }
}

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

pub fn replace_last_char(input: &mut String, ch: char) {
    let last_index = input.chars().count() - 1;
    replace_nth_char(input, last_index, ch);
}

pub fn modify_letter_or_else<F: FnMut(&mut String) -> bool>(
    input: &mut String,
    modification: &LetterModification,
    mut callback: F,
) -> bool {
    let letter_modified = modify_letter(input, modification);

    if !letter_modified {
        return callback(input);
    }

    true
}

pub fn insert_ư_if_vowel_not_present(input: &mut String, is_uppercase: bool) -> bool {
    let Ok((_, vowel)) = parse_vowel(input) else {
        return false;
    };

    if !vowel.is_empty() {
        return false;
    }

    let insert_ch = if !is_uppercase { 'ư' } else { 'Ư' };
    input.push(insert_ch);

    true
}

const MODIFIED_VOWELS: Set<char> = phf_set!['ă', 'â', 'ê', 'ô', 'ơ', 'ư'];
const MODIFIABLE_VOWELS: Set<char> = phf_set!['a', 'e', 'o', 'u'];

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c) || VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn is_vowel_with_accent(c: char) -> bool {
    ACCENT_VOWELS.contains(&c) || ACCENT_VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn is_modified_vowels(c: char) -> bool {
    MODIFIED_VOWELS.contains(&c) || MODIFIED_VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn is_modifiable_vowels(c: char) -> bool {
    MODIFIABLE_VOWELS.contains(&c) || MODIFIABLE_VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn extract_letter_modifications(input: &str) -> Vec<(usize, LetterModification)> {
    input
        .chars()
        .enumerate()
        .filter_map(|(index, ch)| {
            if HORN_MAP.values().find(|c| **c == ch).is_some() {
                return Some((index, LetterModification::Horn));
            }
            if BREVE_MAP.values().find(|c| **c == ch).is_some() {
                return Some((index, LetterModification::Breve));
            }
            if CIRCUMFLEX_MAP.values().find(|c| **c == ch).is_some() {
                return Some((index, LetterModification::Circumflex));
            }
            if DYET_MAP.values().find(|c| **c == ch).is_some() {
                return Some((index, LetterModification::Dyet));
            }
            None
        })
        .collect()
}

pub fn extract_tone(input: &str) -> Option<ToneMark> {
    for ch in input.chars() {
        let Some(tone_mark) = extract_tone_char(ch) else {
            continue;
        };
        return Some(tone_mark);
    }
    None
}

pub fn extract_tone_char(ch: char) -> Option<ToneMark> {
    if ACCUTE_MAP.values().find(|c| **c == ch).is_some() {
        return Some(ToneMark::Acute);
    }
    if GRAVE_MAP.values().find(|c| **c == ch).is_some() {
        return Some(ToneMark::Grave);
    }
    if HOOK_ABOVE_MAP.values().find(|c| **c == ch).is_some() {
        return Some(ToneMark::HookAbove);
    }
    if TILDE_MAP.values().find(|c| **c == ch).is_some() {
        return Some(ToneMark::Tilde);
    }
    if DOT_MAP.values().find(|c| **c == ch).is_some() {
        return Some(ToneMark::Underdot);
    }
    None
}
