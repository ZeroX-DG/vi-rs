//! The telex method transformation
use crate::processor::{
    add_tone, modify_letter, remove_tone, reposition_letter_modification, reposition_tone_mark,
    Transformation,
};
use crate::util::{
    insert_ư_if_vowel_not_present, is_vowel, modify_letter_or_else, replace_last_char,
};
use crate::validation::is_valid_word;
use crate::TransformResult;

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
pub fn transform_buffer<I>(buffer: I, output: &mut String) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    let mut result = String::new();
    let mut ư_inserted_previously = false;
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    for ch in buffer {
        let fallback = format!("{}{}", result, ch);
        let ch_lowercase = ch.to_ascii_lowercase();

        if ch_lowercase != 'w' {
            ư_inserted_previously = false;
        }

        let transformation = match ch_lowercase {
            's' => add_tone(&mut result, &ToneMark::Acute),
            'f' => add_tone(&mut result, &ToneMark::Grave),
            'r' => add_tone(&mut result, &ToneMark::HookAbove),
            'x' => add_tone(&mut result, &ToneMark::Tilde),
            'j' => add_tone(&mut result, &ToneMark::Underdot),
            'z' => remove_tone(&mut result),
            'a' | 'e' | 'o' if contains_clean_char(&result, ch_lowercase) => {
                modify_letter(&mut result, &LetterModification::Circumflex)
            }
            'w' if ư_inserted_previously => {
                replace_last_char(&mut result, ch);
                Transformation::LetterModificationAdded
            }
            'w' => modify_letter_or_else(&mut result, &LetterModification::Horn, |result| {
                modify_letter_or_else(result, &LetterModification::Breve, |result| {
                    let transformation = insert_ư_if_vowel_not_present(result, ch.is_uppercase());
                    ư_inserted_previously = transformation != Transformation::Ignored;
                    transformation
                })
            }),
            'd' => modify_letter(&mut result, &LetterModification::Dyet),
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
            // If tone mark was intentionally removed with z character then it's count as an action.
            Transformation::ToneMarkRemoved => ch_lowercase == 'z',
            _ => true,
        };

        if !action_performed {
            result.push(ch);
        } else if !ư_inserted_previously && !is_valid_word(&result) {
            result = fallback;
        }

        if is_vowel(ch) {
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
