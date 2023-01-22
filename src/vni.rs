use crate::{validation::is_valid_word, processor::{add_tone, modify_letter, remove_tone}};

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
    let mut result = String::new();
    for ch in buffer {
        let ch = &ch;

        let fallback = format!("{}{}", result, ch);

        let action_performed = match ch {
            '1' => add_tone(&mut result, &ToneMark::Acute),
            '2' => add_tone(&mut result, &ToneMark::Grave),
            '3' => add_tone(&mut result, &ToneMark::HookAbove),
            '4' => add_tone(&mut result, &ToneMark::Tilde),
            '5' => add_tone(&mut result, &ToneMark::Underdot),
            '6' => modify_letter(&mut result, &LetterModification::Circumflex),
            '7' => modify_letter(&mut result, &LetterModification::Horn),
            '8' => modify_letter(&mut result, &LetterModification::Breve),
            '9' => modify_letter(&mut result, &LetterModification::Dyet),
            '0' => remove_tone(&mut result),
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
