use phf::{phf_set, Set};

use crate::{
    maps::{
        ACCUTE_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DOT_MAP, DYET_MAP, GRAVE_MAP, HOOK_ABOVE_MAP,
        HORN_MAP, TILDE_MAP, VOWELS,
    },
    processor::{modify_letter, LetterModification, ToneMark},
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

const MODIFIED_VOWELS: Set<char> = phf_set!['ă', 'â', 'ê', 'ô', 'ơ', 'ư'];
const MODIFIABLE_VOWELS: Set<char> = phf_set!['a', 'e', 'o', 'u'];

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c) || VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn is_modified_vowels(c: char) -> bool {
    MODIFIED_VOWELS.contains(&c) || MODIFIED_VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn is_modifiable_vowels(c: char) -> bool {
    MODIFIABLE_VOWELS.contains(&c) || MODIFIABLE_VOWELS.contains(&c.to_lowercase().next().unwrap())
}

pub fn extract_tone(input: &str) -> Option<ToneMark> {
    for ch in input.chars() {
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
    }
    None
}

pub fn extract_letter_modification(input: &str) -> Option<LetterModification> {
    for ch in input.chars() {
        if HORN_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Horn);
        }
        if BREVE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Breve);
        }
        if CIRCUMFLEX_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Circumflex);
        }
        if DYET_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Dyet);
        }
    }
    None
}

pub fn get_next_char_index(input: &str, current_index: usize) -> usize {
    let mut index = current_index + 1;
    while !input.is_char_boundary(index) {
        index += 1;
    }
    index
}

pub fn get_char_at(input: &str, index: usize) -> Option<char> {
    input
        .get(index..get_next_char_index(input, index))
        .map(|res| res.chars().next().unwrap())
}

pub struct WordComponents<'a> {
    word: &'a str,
    vowel_index_start: usize,
    final_consonant_index_start: usize,
    found_vowel: bool,
    found_initial_consonant: bool,
    found_final_consonant: bool,
}

impl<'a> WordComponents<'a> {
    pub fn extract(input: &'a str) -> Self {
        let mut vowel_index_start = 0;
        let mut final_consonant_index_start = 0;
        let mut found_vowel = false;
        let mut found_initial_consonant = false;
        let mut found_final_consonant = false;

        for (index, ch) in input.char_indices() {
            if !found_vowel && !is_vowel(ch) {
                found_initial_consonant = true;
            }

            if !found_vowel && is_vowel(ch) {
                vowel_index_start = index;
                found_vowel = true;
            }

            if found_vowel && !found_final_consonant && !is_vowel(ch) {
                final_consonant_index_start = index;
                found_final_consonant = true;
                break;
            }
        }

        Self {
            word: input,
            vowel_index_start,
            final_consonant_index_start,
            found_vowel,
            found_initial_consonant,
            found_final_consonant,
        }
    }

    pub fn initial_consonant(&self) -> &str {
        if !self.found_initial_consonant {
            return &self.word[..0];
        }
        &self.word[..self.vowel_index_start]
    }

    pub fn final_consonant(&self) -> &str {
        if !self.found_final_consonant {
            return &self.word[..0];
        }
        &self.word[self.final_consonant_index_start..]
    }

    pub fn vowel(&self) -> &str {
        if !self.found_vowel {
            return &self.word[..0];
        }
        let end_index = if self.found_final_consonant {
            self.final_consonant_index_start
        } else {
            self.word.len()
        };
        &self.word[self.vowel_index_start..end_index]
    }
}
