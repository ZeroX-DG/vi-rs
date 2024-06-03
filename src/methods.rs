use phf::{phf_map, Map};

use crate::{
    processor::{
        add_tone, modify_letter, remove_tone, LetterModification, ToneMark, Transformation,
    },
    validation::is_valid_word,
    word::Word,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    AddTonemark(ToneMark),
    ModifyLetter(LetterModification),
    ModifyLetterOnCharacterFamily(LetterModification, char),
    InsertƯ,
    ResetInsertedƯ,
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

pub static TELEX: Definition = phf_map! {
    's' => &[Action::AddTonemark(ToneMark::Acute)],
    'f' => &[Action::AddTonemark(ToneMark::Grave)],
    'r' => &[Action::AddTonemark(ToneMark::HookAbove)],
    'x' => &[Action::AddTonemark(ToneMark::Tilde)],
    'j' => &[Action::AddTonemark(ToneMark::Underdot)],
    'a' => &[Action::ModifyLetterOnCharacterFamily(LetterModification::Circumflex, 'a')],
    'e' => &[Action::ModifyLetterOnCharacterFamily(LetterModification::Circumflex, 'e')],
    'o' => &[Action::ModifyLetterOnCharacterFamily(LetterModification::Circumflex, 'o')],
    'w' => &[Action::ResetInsertedƯ, Action::ModifyLetter(LetterModification::Horn), Action::ModifyLetter(LetterModification::Breve), Action::InsertƯ],
    'd' => &[Action::ModifyLetter(LetterModification::Dyet)],
    'z' => &[Action::RemoveToneMark],
};

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

    let mut last_executed_action = None;

    for ch in buffer {
        let lowercase_ch = ch.to_ascii_lowercase();

        // If a character is not recognised as a transformation character in definition. Skip it.
        if !definition.contains_key(&lowercase_ch) {
            word.push(ch);
            continue;
        }

        let fallback = format!("{}{}", word, ch);
        let actions = definition.get(&lowercase_ch).unwrap();

        let mut action_iter = actions.iter();
        let mut action = action_iter.next().unwrap();

        loop {
            let transformation = match action {
                Action::AddTonemark(tonemark) => add_tone(&mut word, tonemark),
                Action::ModifyLetter(modification) => modify_letter(&mut word, modification),
                Action::ModifyLetterOnCharacterFamily(modification, family_char)
                    if word.vowel.to_ascii_lowercase().contains(*family_char) =>
                {
                    modify_letter(&mut word, modification)
                }
                Action::RemoveToneMark => remove_tone(&mut word),
                Action::InsertƯ => {
                    let transformation = if word.vowel.is_empty() || word.to_string() == "gi" {
                        word.push(if ch.is_lowercase() { 'u' } else { 'U' });
                        let last_index = word.len() - 1;
                        word.letter_modifications
                            .push((last_index, LetterModification::Horn));
                        Transformation::LetterModificationAdded
                    } else {
                        Transformation::Ignored
                    };
                    transformation
                }
                Action::ResetInsertedƯ if matches!(last_executed_action, Some(Action::InsertƯ)) =>
                {
                    word.replace_last_char(ch);
                    Transformation::LetterModificationRemoved
                }
                _ => Transformation::Ignored,
            };

            // If the transformation cannot be applied, try the next action if there's one.
            if transformation == Transformation::Ignored {
                if let Some(next_action) = action_iter.next() {
                    action = next_action;
                    continue;
                }
            }

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

            // If the action is to trigger reset ư insert then we don't need further processing
            if *action == Action::ResetInsertedƯ {
                last_executed_action = Some(action.clone());
                break;
            }

            if !action_performed {
                word.push(ch);
                last_executed_action = None;
            } else if !is_valid_word(&word.to_string()) {
                word.set(fallback);
                last_executed_action = None;
            } else {
                last_executed_action = Some(action.clone());
            }
            break;
        }
    }

    output.push_str(&word.to_string());

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
