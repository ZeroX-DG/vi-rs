//! The vni method transformation
use crate::{
    processor::{
        add_tone, modify_letter, remove_tone, reposition_letter_modification, reposition_tone_mark,
        Transformation,
    },
    util::is_vowel,
    validation::is_valid_word,
    TransformResult,
};

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
pub fn transform_buffer<I>(buffer: I, output: &mut String) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    let mut result = String::new();
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    for ch in buffer {
        let ch = &ch;

        let fallback = format!("{}{}", result, ch);

        let transformation = match ch {
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
            _ => Transformation::Ignored,
        };

        if transformation == Transformation::ToneMarkRemoved {
            tone_mark_removed = true;
        }

        if transformation == Transformation::LetterModificationRemoved {
            letter_modification_removed = true;
        }

        match transformation {
            Transformation::LetterModificationAdded
            | Transformation::LetterModificationRemoved
            | Transformation::LetterModificationReplaced => {
                reposition_tone_mark(&mut result);
            }
            _ => {}
        }

        let action_performed = match transformation {
            Transformation::Ignored | Transformation::LetterModificationRemoved => false,
            // If tone mark was intentionally removed with 0 character then it's count as an action.
            Transformation::ToneMarkRemoved => *ch == '0',
            _ => true,
        };

        if !action_performed {
            result.push(*ch);
        } else if !is_valid_word(&result) {
            result = fallback;
        }

        if is_vowel(*ch) {
            reposition_tone_mark(&mut result);
        }
        reposition_letter_modification(&mut result);
    }
    output.push_str(&result);

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
