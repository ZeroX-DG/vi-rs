use regex::Regex;

use crate::{processor::{modify_letter, LetterModification, ToneMark}, maps::{ACCUTE_MAP, GRAVE_MAP, HOOK_ABOVE_MAP, TILDE_MAP, DOT_MAP, HORN_MAP, BREVE_MAP, CIRCUMFLEX_MAP, DYET_MAP}};

pub fn clean_char(ch: char) -> char {
    let accents = vec![
        "aàảãáạăằẳẵắặâầẩẫấậ",
        "AÀẢÃÁẠĂẰẲẴẮẶÂẦẨẪẤẬ",
        "dđ",
        "DĐ",
        "eèẻẽéẹêềểễếệ",
        "EÈẺẼÉẸÊỀỂỄẾỆ",
        "iìỉĩíị",
        "IÌỈĨÍỊ",
        "oòỏõóọôồổỗốộơờởỡớợ",
        "OÒỎÕÓỌÔỒỔỖỐỘƠỜỞỠỚỢ",
        "uùủũúụưừửữứự",
        "UÙỦŨÚỤƯỪỬỮỨỰ",
        "yỳỷỹýỵ",
        "YỲỶỸÝỴ",
    ];
    for accent in accents {
        let regex = Regex::new(&format!("[{}]", &accent[1..]));
        let replace_char = accent.chars().next().unwrap();
        if let Ok(re) = regex {
            if re.is_match(&ch.to_string()) {
                return replace_char;
            }
        }
    }
    ch
}

pub fn remove_tone_mark(ch: char) -> char {
    let tone_mark_map = vec![
        "aàảãáạ",
        "ăằẳẵắặ",
        "âầẩẫấậ",
        "AÀẢÃÁẠ",
        "ĂẰẲẴẮẶ",
        "ÂẦẨẪẤẬ",
        "eèẻẽéẹ",
        "êềểễếệ",
        "EÈẺẼÉẸ",
        "ÊỀỂỄẾỆ",
        "iìỉĩíị",
        "IÌỈĨÍỊ",
        "oòỏõóọ",
        "ôồổỗốộ",
        "ơờởỡớợ",
        "OÒỎÕÓỌ",
        "ÔỒỔỖỐỘ",
        "ƠỜỞỠỚỢ",
        "uùủũúụ",
        "ưừửữứự",
        "UÙỦŨÚỤ",
        "ƯỪỬỮỨỰ",
        "yỳỷỹýỵ",
        "YỲỶỸÝỴ",
    ];
    for tone_mark in tone_mark_map {
        let regex = Regex::new(&format!(
            "[{}]",
            &tone_mark.chars().skip(1).collect::<String>()
        ));
        let replace_char = tone_mark.chars().next().unwrap();
        if let Ok(re) = regex {
            if re.is_match(&ch.to_string()) {
                return replace_char;
            }
        }
    }
    ch
}

pub fn modify_letter_or_else<F: FnMut(&mut String) -> bool>(
    input: &mut String,
    modification: &LetterModification,
    mut callback: F,
) -> bool {
    let letter_modified = modify_letter(input, modification);

    if !letter_modified {
        return callback(input)
    }

    true
}

const VOWELS: [char; 12] = ['a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y'];
const MODIFIED_VOWELS: [char; 6] = ['ă', 'â', 'ê', 'ô', 'ơ', 'ư'];
const MODIFIABLE_VOWELS: [char; 4] = ['a', 'e', 'o', 'u'];

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

pub struct WordComponents<'a> {
    word: &'a str,
    vowel_index_start: usize,
    final_consonant_index_start: usize,
    found_vowel: bool,
    found_initial_consonant: bool,
    found_final_consonant: bool
}

impl<'a> WordComponents<'a> {
    pub fn extract(input: &'a str) -> Self {
        let mut vowel_index_start = 0;
        let mut final_consonant_index_start = 0;
        let mut found_vowel = false;
        let mut found_initial_consonant = false;
        let mut found_final_consonant = false;

        for (index, ch) in input.char_indices().map(|(i, c)| (i, clean_char(c).to_ascii_lowercase())) {
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
            found_final_consonant
        }
    }

    pub fn initial_consonant(&self) -> &str {
        if !self.found_initial_consonant {
            return &self.word[..0]
        }
        &self.word[..self.vowel_index_start]
    }

    pub fn final_consonant(&self) -> &str {
        if !self.found_final_consonant {
            return &self.word[..0]
        }
        &self.word[self.final_consonant_index_start..]
    }

    pub fn vowel(&self) -> &str {
        if !self.found_vowel {
            return &self.word[..0]
        }
        let end_index = if self.found_final_consonant {
            self.final_consonant_index_start
        } else {
            self.word.len()
        };
        &self.word[self.vowel_index_start..end_index]
    }
}
