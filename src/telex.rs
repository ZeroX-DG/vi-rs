//! The telex method transformation
use crate::processor::{add_tone, modify_letter, remove_tone, Transformation};
use crate::validation::is_valid_word;
use crate::word::Word;
use crate::TransformResult;

use super::processor::{LetterModification, ToneMark};

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
    let mut word = Word::empty();
    let mut ư_inserted_previously = false;
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    for ch in buffer {
        let fallback = format!("{}{}", word, ch);
        let ch_lowercase = ch.to_ascii_lowercase();

        if ch_lowercase != 'w' {
            ư_inserted_previously = false;
        }

        let transformation = match ch_lowercase {
            's' => add_tone(&mut word, &ToneMark::Acute),
            'f' => add_tone(&mut word, &ToneMark::Grave),
            'r' => add_tone(&mut word, &ToneMark::HookAbove),
            'x' => add_tone(&mut word, &ToneMark::Tilde),
            'j' => add_tone(&mut word, &ToneMark::Underdot),
            'z' => remove_tone(&mut word),
            'a' | 'e' | 'o' if word.vowel.to_ascii_lowercase().contains(ch_lowercase) => {
                modify_letter(&mut word, &LetterModification::Circumflex)
            }
            'w' if ư_inserted_previously => {
                word.replace_last_char(ch);
                Transformation::LetterModificationRemoved
            }
            'w' => match modify_letter(&mut word, &LetterModification::Horn) {
                Transformation::Ignored | Transformation::LetterModificationRemoved => {
                    match modify_letter(&mut word, &LetterModification::Breve) {
                        Transformation::Ignored | Transformation::LetterModificationRemoved => {
                            let transformation =
                                if word.vowel.is_empty() || word.to_string() == "gi" {
                                    word.push(if ch.is_lowercase() { 'u' } else { 'U' });
                                    let last_index = word.len() - 1;
                                    word.letter_modifications
                                        .push((last_index, LetterModification::Horn));
                                    Transformation::LetterModificationAdded
                                } else {
                                    Transformation::Ignored
                                };
                            ư_inserted_previously = transformation != Transformation::Ignored;
                            transformation
                        }
                        transformation => transformation,
                    }
                }
                transformation => transformation,
            },
            'd' => modify_letter(&mut word, &LetterModification::Dyet),
            _ => Transformation::Ignored,
        };

        if transformation == Transformation::ToneMarkRemoved {
            tone_mark_removed = true;
        }

        if transformation == Transformation::LetterModificationRemoved {
            letter_modification_removed = true;
        }

        let initial_ư_removed = Transformation::LetterModificationRemoved == transformation
            && ư_inserted_previously
            && word.len() == 1;

        let action_performed = match transformation {
            Transformation::LetterModificationRemoved if initial_ư_removed => true,
            Transformation::Ignored | Transformation::LetterModificationRemoved => false,
            // If tone mark was intentionally removed with z character then it's count as an action.
            Transformation::ToneMarkRemoved => ch_lowercase == 'z',
            _ => true,
        };

        if !action_performed {
            word.push(ch);
        } else if !initial_ư_removed && !is_valid_word(&word.to_string()) {
            word.set(fallback);
        }

        if initial_ư_removed {
            ư_inserted_previously = false;
        }
    }
    output.push_str(&word.to_string());

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
