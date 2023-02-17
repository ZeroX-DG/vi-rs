use crate::processor::{add_tone, modify_letter, remove_tone};
use crate::util::modify_letter_or_else;
use crate::validation::is_valid_word;

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
        let fallback = format!("{}{}", result, ch);
        let ch_lowercase = ch.to_ascii_lowercase();
        let action_performed = match ch_lowercase {
            's' => add_tone(&mut result, &ToneMark::Acute),
            'f' => add_tone(&mut result, &ToneMark::Grave),
            'r' => add_tone(&mut result, &ToneMark::HookAbove),
            'x' => add_tone(&mut result, &ToneMark::Tilde),
            'j' => add_tone(&mut result, &ToneMark::Underdot),
            'z' => remove_tone(&mut result),

            'a' | 'e' | 'o' if contains_clean_char(&result, ch_lowercase) => {
                modify_letter(&mut result, &LetterModification::Circumflex)
            }
            'w' => modify_letter_or_else(&mut result, &LetterModification::Horn, |result| {
                modify_letter_or_else(result, &LetterModification::Breve, |result| {
                    let insert_ch = if ch.is_lowercase() { 'ư' } else { 'Ư' };
                    result.push(insert_ch);
                    true
                })
            }),
            'd' => modify_letter(&mut result, &LetterModification::Dyet),
            _ => false,
        };

        if !action_performed {
            result.push(*ch);
        } else if !is_valid_word(&result) {
            result = fallback;
        }
    }
    output.push_str(&result);
}
