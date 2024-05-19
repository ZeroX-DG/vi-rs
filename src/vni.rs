//! The vni method transformation
use crate::{
    processor::{add_tone, modify_letter, remove_tone, Transformation},
    validation::is_valid_word,
    word::Word,
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
    let mut word = Word::empty();
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    for ch in buffer {
        let ch = &ch;
        let fallback = format!("{}{}", word, ch);

        let transformation = match ch {
            '1' => add_tone(&mut word, &ToneMark::Acute),
            '2' => add_tone(&mut word, &ToneMark::Grave),
            '3' => add_tone(&mut word, &ToneMark::HookAbove),
            '4' => add_tone(&mut word, &ToneMark::Tilde),
            '5' => add_tone(&mut word, &ToneMark::Underdot),
            '6' => modify_letter(&mut word, &LetterModification::Circumflex),
            '7' => modify_letter(&mut word, &LetterModification::Horn),
            '8' => modify_letter(&mut word, &LetterModification::Breve),
            '9' => modify_letter(&mut word, &LetterModification::Dyet),
            '0' => remove_tone(&mut word),
            _ => Transformation::Ignored,
        };

        if transformation == Transformation::ToneMarkRemoved {
            tone_mark_removed = true;
        }

        if transformation == Transformation::LetterModificationRemoved {
            letter_modification_removed = true;
        }

        let action_performed = match transformation {
            Transformation::Ignored | Transformation::LetterModificationRemoved => false,
            // If tone mark was intentionally removed with 0 character then it's count as an action.
            Transformation::ToneMarkRemoved => *ch == '0',
            _ => true,
        };

        if !action_performed {
            word.push(*ch);
        } else if !is_valid_word(&word.to_string()) {
            word.set(fallback);
        }
    }

    output.push_str(&word.to_string());

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
