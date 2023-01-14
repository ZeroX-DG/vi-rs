use crate::util::{add_tone_or_append, modify_letter_or_append, remove_tone_or_append};

use super::processor::{LetterModification, ToneMark};

/// Transform input buffer containing a single word to vietnamese string output using vni mode.
///
/// # Example
/// ```
/// use vi::vni::transform_buffer;
///
/// let mut result = String::new();
/// transform_buffer("viet65".chars(), &mut result);
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(buffer: I, output: &mut String)
where
    I: IntoIterator<Item = char>,
{
    for ch in buffer {
        let ch = &ch;
        match ch {
            '1' => add_tone_or_append(output, &ToneMark::Acute, ch),
            '2' => add_tone_or_append(output, &ToneMark::Grave, ch),
            '3' => add_tone_or_append(output, &ToneMark::HookAbove, ch),
            '4' => add_tone_or_append(output, &ToneMark::Tilde, ch),
            '5' => add_tone_or_append(output, &ToneMark::Underdot, ch),
            '6' => modify_letter_or_append(output, &LetterModification::Circumflex, ch),
            '7' => modify_letter_or_append(output, &LetterModification::Horn, ch),
            '8' => modify_letter_or_append(output, &LetterModification::Breve, ch),
            '9' => modify_letter_or_append(output, &LetterModification::Dyet, ch),
            '0' => remove_tone_or_append(output, ch),
            _ => output.push(*ch),
        }
    }
}
