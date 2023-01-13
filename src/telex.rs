use crate::util::{add_tone_or_append, modify_letter_or_append};

use super::processor::{LetterModification, ToneMark};

use super::util::clean_char;

fn modifiable_char(ch: &char, previous_ch: &char, modification: &LetterModification) -> bool {
    let clean_previous_ch = clean_char(*previous_ch);

    match modification {
        LetterModification::Circumflex => match clean_previous_ch {
            'a' | 'e' | 'o' | 'A' | 'E' | 'O' => {
                clean_previous_ch.to_ascii_lowercase() == ch.to_ascii_lowercase()
            }
            _ => false,
        },
        LetterModification::Horn => match clean_previous_ch {
            'u' | 'o' | 'U' | 'O' => true,
            _ => false,
        },
        LetterModification::Breve => match clean_previous_ch {
            'a' | 'A' => true,
            _ => false,
        },
        LetterModification::Dyet => match clean_previous_ch {
            'd' | 'D' => true,
            _ => false,
        },
    }
}

/// Transform input buffer to vietnamese string output using telex mode.
///
/// # Example
/// ```
/// use vi::telex::transform_buffer;
///
/// let result = transform_buffer(vec!['v', 'i', 'e', 'e', 't', 'j'].iter().cloned());
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(buffer: I) -> String
where
    I: IntoIterator<Item = char>,
{
    let mut content = String::new();

    let mut previous_ch = '\0';
    for ch in buffer {
        let ch = &ch;
        match ch {
            's' => add_tone_or_append(&mut content, &ToneMark::Acute, ch),
            'f' => add_tone_or_append(&mut content, &ToneMark::Grave, ch),
            'r' => add_tone_or_append(&mut content, &ToneMark::HookAbove, ch),
            'x' => add_tone_or_append(&mut content, &ToneMark::Tilde, ch),
            'j' => add_tone_or_append(&mut content, &ToneMark::Underdot, ch),

            'a' | 'e' | 'o'
                if modifiable_char(ch, &previous_ch, &LetterModification::Circumflex) =>
            {
                modify_letter_or_append(&mut content, &LetterModification::Circumflex, ch);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Horn) => {
                modify_letter_or_append(&mut content, &LetterModification::Horn, ch);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Breve) => {
                modify_letter_or_append(&mut content, &LetterModification::Breve, ch);
            }
            'd' if modifiable_char(ch, &previous_ch, &LetterModification::Dyet) => {
                modify_letter_or_append(&mut content, &LetterModification::Dyet, ch);
            }
            _ => content.push(*ch),
        }
        previous_ch = content.chars().last().unwrap_or('\0');
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_acute_tone_normal() {
        let input: Vec<char> = vec!['v', 'i', 't', 's'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "vít".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_acute_tone_failed() {
        let input: Vec<char> = vec!['v', 't', 's'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "vts".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_normal() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', 'f'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "hoàng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_double_edit() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', 'f', 'r'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "hoảng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_no_content() {
        let input: Vec<char> = vec!['2', '3'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "23".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_overflow() {
        let input: Vec<char> = vec!['a', 's', 's'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "as".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_action_before_text() {
        let input: Vec<char> = vec!['r', 'u'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "ru".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_all_uppercase() {
        let input: Vec<char> = vec!['C', 'H', 'A', 'O', 'f'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "CHÀO".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_normal() {
        let input: Vec<char> = vec!['v', 'o', 'w'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "vơ".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_group() {
        let input: Vec<char> = vec!['v', 'u', 'o', 'w', 'n'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "vươn".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_failed() {
        let input: Vec<char> = vec!['c', 'h', 'e', 'w'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "chew".to_string();
        assert_eq!(result, expected);

        let input: Vec<char> = vec!['v', 'u', 'o', 'n', 'w'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "vuonw".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_uppercase() {
        let input: Vec<char> = vec!['c', 'h', 'E', 'e'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "chÊ".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_with_existing_tone() {
        let input: Vec<char> = vec!['c', 'h', 'e', 'j', 'e', 'c', 'h'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "chệch".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_override() {
        let input: Vec<char> = vec!['a', 'a', 'w'];
        let result = transform_buffer(input.iter().cloned());
        let expected = "ă".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_if_invalid() {
        let input: Vec<char> = vec![
            'z', 'z', 'z', 'j', 'j', 'j', 'j', 'h', 'h', 'h', 'k', 'k', 'k',
        ];
        let result = transform_buffer(input.iter().cloned());
        let expected = "zzzjjjjhhhkkk".to_string();
        assert_eq!(result, expected);
    }
}
