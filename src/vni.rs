use crate::util::{add_tone_or_append, modify_letter_or_append, remove_tone_or_append};

use super::processor::{
    LetterModification, ToneMark,
};

/// Transform input buffer to vietnamese string output. For example,
/// if the input is `['a', '1']`, then the action add tone mark is
/// triggered by the `1` character and the returned result would be 'á'.
///
/// # Example
/// ```
/// use vi::vni::transform_buffer;
///
/// let result = transform_buffer(&vec!['v', 'i', 'e', 't', '6', '5']);
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<'a, I>(buffer: I) -> String
    where I: IntoIterator<Item = &'a char>
{
    let mut content = String::new();
    for ch in buffer {
        match ch {
            '1' => add_tone_or_append(&mut content, &ToneMark::Acute, ch),
            '2' => add_tone_or_append(&mut content, &ToneMark::Grave, ch),
            '3' => add_tone_or_append(&mut content, &ToneMark::HookAbove, ch),
            '4' => add_tone_or_append(&mut content, &ToneMark::Tilde, ch),
            '5' => add_tone_or_append(&mut content, &ToneMark::Underdot, ch),
            '6' => modify_letter_or_append(&mut content, &LetterModification::Circumflex, ch),
            '7' => modify_letter_or_append(&mut content, &LetterModification::Horn, ch),
            '8' => modify_letter_or_append(&mut content, &LetterModification::Breve, ch),
            '9' => modify_letter_or_append(&mut content, &LetterModification::Dyet, ch),
            '0' => remove_tone_or_append(&mut content),
            _ => content.push(*ch),
        }
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_acute_tone_normal() {
        let input: Vec<char> = vec!['v', 'i', 't', '1'];
        let result = transform_buffer(&input);
        let expected = "vít".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_acute_tone_failed() {
        let input: Vec<char> = vec!['v', 't', '1'];
        let result = transform_buffer(&input);
        let expected = "vt1".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_normal() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', '2'];
        let result = transform_buffer(&input);
        let expected = "hoàng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_double_edit() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', '2', '3'];
        let result = transform_buffer(&input);
        let expected = "hoảng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_no_content() {
        let input: Vec<char> = vec!['2', '3'];
        let result = transform_buffer(&input);
        let expected = "23".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_overflow() {
        let input: Vec<char> = vec!['a', '1', '1'];
        let result = transform_buffer(&input);
        let expected = "a1".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_all_uppercase() {
        let input: Vec<char> = vec!['C', 'H', 'A', 'O', '2'];
        let result = transform_buffer(&input);
        let expected = "CHÀO".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn remove_tone_single() {
        let input: Vec<char> = vec!['l', 'u', 'a', 't', '6', '5', '0'];
        let result = transform_buffer(&input);
        let expected = "luât".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn remove_tone_double() {
        let input: Vec<char> = vec!['c', 'h', 'e', 't', '6', '1', '0', '0'];
        let result = transform_buffer(&input);
        let expected = "chet".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn remove_tone_exceed() {
        let input: Vec<char> = vec!['v', 'i', 't', '5', '0', '0'];
        let result = transform_buffer(&input);
        let expected = "vit0".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_normal() {
        let input: Vec<char> = vec!['v', 'o', '7'];
        let result = transform_buffer(&input);
        let expected = "vơ".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_group() {
        let input: Vec<char> = vec!['v', 'u', 'o', 'n', '7'];
        let result = transform_buffer(&input);
        let expected = "vươn".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_failed() {
        let input: Vec<char> = vec!['c', 'h', 'e', '7'];
        let result = transform_buffer(&input);
        let expected = "che7".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_uppercase() {
        let input: Vec<char> = vec!['c', 'h', 'E', '6'];
        let result = transform_buffer(&input);
        let expected = "chÊ".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_with_existing_tone() {
        let input: Vec<char> = vec!['c', 'h', 'e', 'c', 'h', '5', '6'];
        let result = transform_buffer(&input);
        let expected = "chệch".to_string();
        assert_eq!(result, expected);
    }
}
