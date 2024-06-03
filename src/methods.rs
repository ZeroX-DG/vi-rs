use phf::{phf_map, Map};

use crate::{
    processor::{
        add_tone, modify_letter, remove_tone, LetterModification, ToneMark, Transformation,
    },
    validation::is_valid_word,
    word::Word,
};

#[derive(Debug, PartialEq)]
pub enum Action {
    AddTonemark(ToneMark),
    ModifyLetter(LetterModification),
    RemoveToneMark,
}

pub type Definition = Map<char, &'static [Action]>;

/// A result of a buffer transformation.
#[derive(Debug, Clone)]
pub struct TransformResult {
    /// Indicates whether a tone mark has been removed after the transformation.
    pub tone_mark_removed: bool,
    /// Indicates whether a letter modification has been removed after the transformation.
    pub letter_modification_removed: bool,
}

pub static VNI: Definition = phf_map! {
    '1' => &[Action::AddTonemark(ToneMark::Acute)],
    '2' => &[Action::AddTonemark(ToneMark::Grave)],
    '3' => &[Action::AddTonemark(ToneMark::HookAbove)],
    '4' => &[Action::AddTonemark(ToneMark::Tilde)],
    '5' => &[Action::AddTonemark(ToneMark::Underdot)],
    '6' => &[Action::ModifyLetter(LetterModification::Circumflex)],
    '7' => &[Action::ModifyLetter(LetterModification::Horn)],
    '8' => &[Action::ModifyLetter(LetterModification::Breve)],
    '9' => &[Action::ModifyLetter(LetterModification::Dyet)],
    '0' => &[Action::RemoveToneMark],
};

/// TODO: Define Telex
pub static TELEX: Definition = phf_map! {};

pub fn transform_buffer<I>(
    definition: &Definition,
    buffer: I,
    output: &mut String,
) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    let mut word = Word::empty();
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    for ch in buffer {
        let lowercase_ch = ch.to_ascii_lowercase();

        // If a character is not recognised as a transformation character in definition. Skip it.
        if !definition.contains_key(&lowercase_ch) {
            word.push(ch);
            continue;
        }

        let actions = definition.get(&lowercase_ch).unwrap();

        for action in actions.iter() {
            let fallback = format!("{}{}", word, ch);

            let transformation = match action {
                Action::AddTonemark(tonemark) => add_tone(&mut word, tonemark),
                Action::ModifyLetter(modification) => modify_letter(&mut word, modification),
                Action::RemoveToneMark => remove_tone(&mut word),
            };

            if transformation == Transformation::ToneMarkRemoved {
                tone_mark_removed = true;
            }

            if transformation == Transformation::LetterModificationRemoved {
                letter_modification_removed = true;
            }

            let action_performed = match transformation {
                Transformation::Ignored | Transformation::LetterModificationRemoved => false,
                // If tone mark was intentionally removed with z character then it's count as an action.
                Transformation::ToneMarkRemoved => *action == Action::RemoveToneMark,
                _ => true,
            };

            if !action_performed {
                word.push(ch);
            } else if !is_valid_word(&word.to_string()) {
                word.set(fallback);
            }
        }
    }

    output.push_str(&word.to_string());

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
