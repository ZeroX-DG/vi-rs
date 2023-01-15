use crate::util::{add_tone_or_append, modify_letter_or_append, modify_letter_or_else};

use super::processor::{LetterModification, ToneMark};

use super::util::clean_char;

fn contains_clean_char(input: &str, ch: char) -> bool {
    input
        .chars()
        .map(clean_char)
        .map(|c| c.to_ascii_lowercase())
        .any(|clean_ch| clean_ch == ch)
}

/// Transform input buffer containing a single word to vietnamese string output using telex mode.
///
/// # Example
/// ```
/// use vi::telex::transform_buffer;
///
/// let mut result = String::new();
/// transform_buffer("vieetj".chars(), &mut result);
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(buffer: I, output: &mut String)
where
    I: IntoIterator<Item = char>,
{
    let mut result = String::new();
    for ch in buffer {
        let ch = &ch;
        match ch {
            's' => add_tone_or_append(&mut result, &ToneMark::Acute, ch),
            'f' => add_tone_or_append(&mut result, &ToneMark::Grave, ch),
            'r' => add_tone_or_append(&mut result, &ToneMark::HookAbove, ch),
            'x' => add_tone_or_append(&mut result, &ToneMark::Tilde, ch),
            'j' => add_tone_or_append(&mut result, &ToneMark::Underdot, ch),

            'a' | 'e' | 'o' if contains_clean_char(&result, *ch) => {
                modify_letter_or_append(&mut result, &LetterModification::Circumflex, ch)
            }
            'w' => modify_letter_or_else(&mut result, &LetterModification::Horn, |result| {
                modify_letter_or_append(result, &LetterModification::Breve, ch);
            }),
            'd' => modify_letter_or_append(&mut result, &LetterModification::Dyet, ch),
            _ => result.push(*ch),
        }
    }
    output.push_str(&result);
}
