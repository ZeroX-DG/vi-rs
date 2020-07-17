use super::util::{remove_tone_mark, clean_char};
use super::maps::{
    ACCUTE_MAP, GRAVE_MAP, HOOK_ABOVE_MAP, TILDE_MAP, DOT_MAP,
    CIRCUMFLEX_MAP, DYET_MAP, HORN_MAP, BREVE_MAP
};

const VOWELS: [char; 12] = ['a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y'];
const MODIFIED_VOWELS: [char; 6] = ['ă', 'â', 'ê', 'ô', 'ơ', 'ư'];

fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}

fn is_modified_vowels(c: char) -> bool {
    MODIFIED_VOWELS.contains(&c)
}

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
    Underdot
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
    Dyet
}

/// An action contained in an input string
pub enum Action {
    AddTone(ToneMark),
    ModifyLetter(LetterModification),
    RemoveTone
}

/// Get the main sound of a word which is the part that start
/// with a vowel and end with word end or a non-vowel char
pub fn get_word_mid(word: String) -> Option<(usize, String)> {
    let mut result = String::new();
    let mut found_word_mid = false;
    let mut start_index: usize = 0;
    let lower_word = word.to_lowercase();
    for (index, ch) in lower_word.chars().enumerate() {
        if is_vowel(ch) {
            if ch == 'u' && index > 0 {
                let prev_ch = word.chars().nth(index - 1).unwrap();
                if prev_ch == 'q' {
                    continue; // special case 'qu' is start sound
                }
            }
            if ch == 'i' && index > 0 {
                let prev_ch = word.chars().nth(index - 1).unwrap();
                if prev_ch == 'g' {
                    continue; // special case 'gi' is start sound
                }
            }
            result.push(ch);
            if !found_word_mid {
                found_word_mid = true;
                start_index = index;
            }
        } else {
            if found_word_mid {
                break
            }
        }
    }
    if !found_word_mid {
        return None
    }
    Some((start_index, result))
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
fn get_tone_mark_placement(input: &String) -> Option<usize> {
    if let Some((mid_index, word_mid)) = get_word_mid(input.clone()) {
        let is_end_with_mid = input.len() == mid_index + word_mid.len();
        if word_mid.len() == 1 { // single vowel
            return Some(mid_index);
        }
        if let Some(pos) = index_of(&word_mid, |c| c == 'ơ') {
            return Some(mid_index + pos);
        }
        if let Some(pos) = index_of(&word_mid, is_modified_vowels) {
            return Some(mid_index + pos);
        }
        for pair in ["oa", "oe", "oo", "uy"].iter() {
            if let Some(pos) = has_pair(&word_mid, pair) {
                return Some(mid_index + pos + 1);
            }
        }
        if is_end_with_mid {
            if word_mid.len() >= 2 {
                return Some(mid_index + word_mid.len() - 2)
            }
        }
        if let Some(pos) = index_of(&word_mid, is_vowel) {
            return Some(mid_index + pos);
        }
    }
    None
}

fn has_pair(input: &String, pair: &str) -> Option<usize> {
    if let (Some(ch), Some(ch_next)) = (pair.chars().nth(0), pair.chars().nth(1)) {
        if let Some(pos) = index_of(&input, |c| c == ch) {
            if let Some(target_ch) = input.chars().nth(pos + 1) {
                if target_ch == ch_next {
                    return Some(pos);
                }
            }
        }
    }
    None
}

fn index_of<F: Fn(char) -> bool>(input: &String, test: F) -> Option<usize> {
    return input.chars().position(test)
}

fn replace_char_at(input: &String, index: usize, ch: char) -> String {
    let mut result: String = input.chars().take(index).collect();
    result.push(ch);
    result.push_str(&input.chars().skip(index + 1).collect::<String>());
    result
}

fn extract_tone(input: &String) -> Option<ToneMark> {
    for ch in input.chars() {
        if ACCUTE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(ToneMark::Acute)
        }
        if GRAVE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(ToneMark::Grave)
        }
        if HOOK_ABOVE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(ToneMark::HookAbove)
        }
        if TILDE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(ToneMark::Tilde)
        }
        if DOT_MAP.values().find(|c| **c == ch).is_some() {
            return Some(ToneMark::Underdot)
        }
    }
    None
}

fn extract_letter_modification(input: &String) -> Option<LetterModification> {
    for ch in input.chars() {
        if HORN_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Horn)
        }
        if BREVE_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Breve)
        }
        if CIRCUMFLEX_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Circumflex)
        }
        if DYET_MAP.values().find(|c| **c == ch).is_some() {
            return Some(LetterModification::Dyet)
        }
    }
    None
}

/// Add tone mark to input
/// Return if the tone mark has been added or not and what's the output
pub fn add_tone(input: &String, tone_mark: &ToneMark) -> (bool, String) {
    let clean_input = input.clone()
        .chars()
        .map(remove_tone_mark)
        .collect::<String>();

    if let Some(existing_tone) = extract_tone(input) {
        if existing_tone == *tone_mark {
            return (false, clean_input);
        }
    }

    let tone_mark_pos_result = get_tone_mark_placement(&clean_input);
    if let Some(tone_mark_pos) = tone_mark_pos_result {
        let tone_mark_ch = clean_input
            .chars()
            .nth(tone_mark_pos)
            .unwrap();
        let tone_mark_map = match tone_mark {
            ToneMark::Acute     => &ACCUTE_MAP,
            ToneMark::Grave     => &GRAVE_MAP,
            ToneMark::HookAbove => &HOOK_ABOVE_MAP,
            ToneMark::Tilde     => &TILDE_MAP,
            ToneMark::Underdot  => &DOT_MAP
        };
        let replace_char: char = if tone_mark_map.contains_key(&tone_mark_ch) {
            tone_mark_map[&tone_mark_ch]
        } else {
            tone_mark_ch
        };
        return (true, replace_char_at(&clean_input, tone_mark_pos, replace_char));
    }
    (false, input.clone())
}

/// change a letter to vietnamese modified letter
/// Return if the letter has been modified or not and what's the output
pub fn modify_letter(input: &String, modification: &LetterModification) -> (bool, String) {
    let map = match modification {
        LetterModification::Horn       => &HORN_MAP,
        LetterModification::Breve      => &BREVE_MAP,
        LetterModification::Circumflex => &CIRCUMFLEX_MAP,
        LetterModification::Dyet       => &DYET_MAP
    };
    let mut result = input.clone();

    let clean_input = input.clone()
        .chars()
        .map(clean_char)
        .collect::<String>();

    if let Some(existing_modification) = extract_letter_modification(input) {
        if existing_modification == *modification {
            return (false, clean_input);
        }
    }

    for (index, ch) in clean_input.clone().chars().enumerate() {
        if map.contains_key(&ch) {
            result = replace_char_at(&result, index, map[&ch]);
        }
    }
    (result != *input, result)
}

/// Remove the tone for the letter
pub fn remove_tone(input: &String) -> String {
    let new_input: String = input.clone()
        .chars()
        .map(remove_tone_mark)
        .collect();
    if new_input == *input {
        return new_input.chars().map(clean_char).collect();
    }
    return new_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_word_mid_normal() {
        let result = get_word_mid("viet".to_owned());
        let expected: Option<(usize, String)> = Some((1, "ie".to_owned()));
        assert_eq!(result, expected);
    }

    #[test]
    fn get_word_mid_empty() {
        let result = get_word_mid("vt".to_owned());
        let expected: Option<(usize, String)> = None;
        assert_eq!(result, expected); 
    }

    #[test]
    fn get_word_mid_double_start_tone() {
        let result = get_word_mid("quai".to_owned());
        let expected: Option<(usize, String)> = Some((2, "ai".to_owned()));
        assert_eq!(result, expected); 
    }

    #[test]
    fn get_word_mid_double_start_tone_2() {
        let result = get_word_mid("gia".to_owned());
        let expected: Option<(usize, String)> = Some((2, "a".to_owned()));
        assert_eq!(result, expected); 
    }

    #[test]
    fn get_tone_mark_placement_normal() {
        let result = get_tone_mark_placement(&"choe".to_owned());
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_special() {
        let result = get_tone_mark_placement(&"chieu".to_owned());
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_mid_not_end() {
        let result = get_tone_mark_placement(&"hoang".to_owned());
        let expected: Option<usize> = Some(2);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_u_and_o() {
        let result = get_tone_mark_placement(&"ngươi".to_owned());
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_uppercase() {
        let result = get_tone_mark_placement(&"chÊt".to_owned());
        let expected: Option<usize> = Some(2);
        assert_eq!(result, expected);
    }
}
