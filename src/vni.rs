use crate::util::{add_tone_or_append, modify_letter_or_append, remove_tone_or_append};

use super::processor::{LetterModification, ToneMark};

/// Transform input buffer to vietnamese string output using vni mode.
///
/// # Example
/// ```
/// use vi::vni::transform_buffer;
///
/// let result = transform_buffer(vec!['v', 'i', 'e', 't', '6', '5'].iter().cloned());
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(buffer: I) -> String
where
    I: IntoIterator<Item = char>,
{
    let mut content = String::new();
    for ch in buffer {
        let ch = &ch;
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