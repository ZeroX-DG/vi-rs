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

pub fn get_word_mid(word: String) -> Option<(usize, String)> {
    let mut result = String::new();
    let mut found_word_mid = false;
    let mut start_index: usize = 0;
    for (index, ch) in word.chars().enumerate() {
        if is_vowel(ch) {
            if ch == 'u' {
                if let Some(prev_ch) = word.chars().nth(index - 1) {
                    if prev_ch == 'q' {
                        continue; // special case 'qu' is start sound
                    }
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
/// 2. If a word contains ư, tone mark goes there
/// 3. If a modified letter goes with a non-modified vowel, tone mark should be 
/// on modifed letter
/// 4. If a word contains `oa`, `oe`, `oo`, `oy`, tone mark should be on the
/// second vowel
/// 5. Else, but tone mark on whatever vowel comes first
fn get_tone_mark_placement(input: String) -> Option<usize> {
    if let Some((mid_index, word_mid)) = get_word_mid(input.clone()) {
        let is_end_with_mid = input.len() == mid_index + word_mid.len();
        if word_mid.len() == 1 { // single vowel
            return Some(mid_index);
        }
        if let Some(pos) = index_of(&word_mid, |c| c == 'ư') {
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
    if let Some(ch) = pair.chars().nth(0) {
        if let Some(ch_next) = pair.chars().nth(1) {
            if let Some(pos) = index_of(&input, |c| c == ch) {
                if let Some(target_ch) = input.chars().nth(pos + 1) {
                    if target_ch == ch_next {
                        return Some(pos);
                    }
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

/// Add tone mark to input
pub fn add_tone(input: &String, tone_mark: &ToneMark) -> String {
    let tone_mark_pos_result = get_tone_mark_placement(input.clone());
    if let Some(tone_mark_pos) = tone_mark_pos_result {
        let tone_mark_ch = input.chars().nth(tone_mark_pos).unwrap();
        let replace_char = match tone_mark {
            ToneMark::Acute => {
                match tone_mark_ch {
                    'a' => 'á',
                    'â' => 'ấ',
                    'ă' => 'ắ',
                    'e' => 'é',
                    'ê' => 'ế',
                    'i' => 'í',
                    'o' => 'ó',
                    'ô' => 'ố',
                    'ơ' => 'ớ',
                    'u' => 'ú',
                    'ư' => 'ứ',
                    'y' => 'ý',
                    _ => tone_mark_ch
                }
            }
            ToneMark::Grave => {
                match tone_mark_ch {
                    'a' => 'à',
                    'â' => 'ầ',
                    'ă' => 'ằ',
                    'e' => 'è',
                    'ê' => 'ề',
                    'i' => 'ì',
                    'o' => 'ò',
                    'ô' => 'ồ',
                    'ơ' => 'ờ',
                    'u' => 'ù',
                    'ư' => 'ừ',
                    'y' => 'ỳ',
                    _ => tone_mark_ch
                }
            }
            ToneMark::HookAbove => {
                match tone_mark_ch {
                    'a' => 'ả',
                    'â' => 'ẩ',
                    'ă' => 'ẳ',
                    'e' => 'ẻ',
                    'ê' => 'ể',
                    'i' => 'ỉ',
                    'o' => 'ỏ',
                    'ô' => 'ổ',
                    'ơ' => 'ở',
                    'u' => 'ủ',
                    'ư' => 'ử',
                    'y' => 'ỷ',
                    _ => tone_mark_ch
                }
            }
            ToneMark::Tilde => {
                match tone_mark_ch {
                    'a' => 'ã',
                    'â' => 'ẫ',
                    'ă' => 'ẵ',
                    'e' => 'ẽ',
                    'ê' => 'ễ',
                    'i' => 'ĩ',
                    'o' => 'õ',
                    'ô' => 'ỗ',
                    'ơ' => 'ỡ',
                    'u' => 'ũ',
                    'ư' => 'ữ',
                    'y' => 'ỹ',
                    _ => tone_mark_ch
                }
            }
            ToneMark::Underdot => {
                match tone_mark_ch {
                    'a' => 'ạ',
                    'â' => 'ậ',
                    'ă' => 'ặ',
                    'e' => 'ẹ',
                    'ê' => 'ệ',
                    'i' => 'ị',
                    'o' => 'ọ',
                    'ô' => 'ộ',
                    'ơ' => 'ợ',
                    'u' => 'ụ',
                    'ư' => 'ự',
                    'y' => 'ỵ',
                    _ => tone_mark_ch
                }
            }
        };
        return replace_char_at(input, tone_mark_pos, replace_char);
    }
    input.clone()
}

pub fn modify_letter(content: &mut String, modification: LetterModification) {

}

pub fn remove_tone(content: &mut String) {

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
    fn get_tone_mark_placement_normal() {
        let result = get_tone_mark_placement("choe".to_owned());
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_special() {
        let result = get_tone_mark_placement("chieu".to_owned());
        let expected: Option<usize> = Some(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_mark_placement_mid_not_end() {
        let result = get_tone_mark_placement("hoang".to_owned());
        let expected: Option<usize> = Some(2);
        assert_eq!(result, expected);
    }
}
