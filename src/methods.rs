//! The definitions of different typing methods.
//!
//! Normally, for IME developers, you only need these things from this module:
//! - [`transform_buffer`] function to transfer your sequence of character into a syllable using a typing definition.
//! - [`TELEX`] typing definition that you can use to pass in [`transform_buffer`] to transform character sequence using telex method.
//! - [`VNI`] typing defnition that you can use to pass in [`transform_buffer`] to trasnform character sequence using vni method.
//!
//! ## Example
//!
//! To transform a character sequence using the VNI definition:
//! ```
//! use vi::methods::transform_buffer;
//!
//! let mut result = String::new();
//! transform_buffer(&vi::VNI, "viet65".chars(), &mut result);
//! assert_eq!(result, "việt".to_owned());
//! ```
//!
//! ## Define your own typing definition
//!
//! `vi-rs` support some typing methods out of the box such as `telex` and `vni`. However, should users ever need to define their
//! own typing methods, they can use the existing APIs in the module.
//!
//! To define a new typing definition, you need to declare a definition map, which is a [`phf::Map`]:
//!
//! ```
//! use phf::phf_map;
//! use vi::{
//!    processor::{LetterModification, ToneMark},
//!    Action, Definition,
//! };
//! use vi::methods::transform_buffer;
//!
//! pub static MY_VNI: Definition = phf_map! {
//!     '1' => &[Action::AddTonemark(ToneMark::Acute)],
//!     '2' => &[Action::AddTonemark(ToneMark::Grave)],
//!     '3' => &[Action::AddTonemark(ToneMark::HookAbove)],
//!     '4' => &[Action::AddTonemark(ToneMark::Tilde)],
//!     '5' => &[Action::AddTonemark(ToneMark::Underdot)],
//!     '6' => &[Action::ModifyLetter(LetterModification::Circumflex)],
//!     '7' => &[Action::ModifyLetter(LetterModification::Horn)],
//!     '8' => &[Action::ModifyLetter(LetterModification::Breve)],
//!     '9' => &[Action::ModifyLetter(LetterModification::Dyet)],
//!     'z' => &[Action::ResetInsertedƯ, Action::InsertƯ],
//!     '0' => &[Action::RemoveToneMark],
//! };
//!
//! // Then you can pass that in `transform_buffer` as usual:
//! let mut result = String::new();
//! transform_buffer(&MY_VNI, "chza".chars(), &mut result);
//! assert_eq!(result, "chưa".to_owned());
//! ```
use phf::{phf_map, Map};

use crate::{
    processor::{
        add_tone, modify_letter, remove_tone, LetterModification, ToneMark, Transformation,
    },
    validation::is_valid_syllable,
    syllable::Syllable,
};

/// An action to be listed as part of a typing definition.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// Add a tonemark
    AddTonemark(ToneMark),
    /// Apply letter modification where possible
    ModifyLetter(LetterModification),
    /// Apply letter modification only if the character family exist. For example,
    /// `ModifyLetterOnCharacterFamily(Circumflex, 'a')` will only apply circumflex
    /// modification if `a` or any character in the `a` family (`â`, `ă`).
    ModifyLetterOnCharacterFamily(LetterModification, char),
    /// Insert an ư character at the end of the syllable.
    InsertƯ,
    /// Remove the last ư character inserted at the end of the syllable. **Note:** this only trigger if the last action is `InsertƯ`.
    ResetInsertedƯ,
    /// Remove the tonemark from the syllable.
    RemoveToneMark,
}

/// A definition of a typing method.
///
/// The definition is a [`phf::Map`] with the key as the character that trigger an action and the value,
/// a list of actions that can be triggered by that character.
///
/// If a character can trigger different actions depending on what is possible, its value will contains multiple Action. For example,
///
/// ```
/// use phf::phf_map;
/// use vi::{
///    processor::{LetterModification, ToneMark},
///    Action, Definition,
/// };
/// pub static TELEX: Definition = phf_map! {
///     'w' => &[Action::ResetInsertedƯ, Action::ModifyLetter(LetterModification::Horn), Action::ModifyLetter(LetterModification::Breve), Action::InsertƯ],
/// };
/// ```
///
/// The definition above specify that `w` can trigger a `ResetInseretedƯ`, or if that doesn't work, a `ModifyLetter(LetterModification::Horn)` action
/// will be executed instead and so on, and so on, \*sniff\*. Note that as soon as one action in the list is applied, the rest of the actions
/// in the list will be ignored.
pub type Definition = Map<char, &'static [Action]>;

/// A result of a buffer transformation.
#[derive(Debug, Clone)]
pub struct TransformResult {
    /// Indicates whether a tone mark has been removed after the transformation.
    pub tone_mark_removed: bool,
    /// Indicates whether a letter modification has been removed after the transformation.
    pub letter_modification_removed: bool,
}

/// A definition for the VNI typing method with these configuration:
///
/// - `1` -> Acute (thêm dấu sắc)
/// - `2` -> Grave (thêm dấu huyền)
/// - `3` -> HookAbove (thêm dấu hỏi)
/// - `4` -> Tilde (thêm dấu ngã)
/// - `5` -> Underdot (thêm dấu nặng)
/// - `6` -> Circumflex (thêm dấu ^)
/// - `7` -> Horn (thêm dấu móc cho ư hoặc ơ)
/// - `8` -> Breve (thêm dấu cho a thành ă)
/// - `9` -> Dyet (thêm dấu gạch cho d thành đ)
/// - `0` -> RemoveToneMark bỏ dấu thanh (sắc, hỏi, ngã, huyền)
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

/// A definition for the Telex typing method with these configuration:
///
/// - `s` -> Acute (thêm dấu sắc)
/// - `f` -> Grave (thêm dấu huyền)
/// - `r` -> HookAbove (thêm dấu hỏi)
/// - `x` -> Tilde (thêm dấu ngã)
/// - `j` -> Underdot (thêm dấu nặng)
/// - `a` -> Circumflex for a (thêm dấu ^ cho chữ a)
/// - `e` -> Circumflex for e (thêm dấu ^ cho chữ e)
/// - `o` -> Circumflex for o (thêm dấu ^ cho chữ o)
/// - `w` -> Horn for ư/ơ or Breve for a (thêm dấu móc cho ư hoặc ơ hoặc thêm dấu cho a thành ă)
/// - `d` -> Dyet (thêm dấu gạch cho d thành đ)
/// - `z` -> RemoveToneMark bỏ dấu thanh (sắc, hỏi, ngã, huyền)
///
/// **Note:**
/// - By default `w` inserted by itself will be inserted as `ư` in the syllable.
/// - An `u` followed by a `w` will produce: `ư`, and if you add another `w`, it will result in `uw`.
/// - A `w` will produce `ư`, and if it's followed by a `w`, it will not produce `uw` but will replace `ư` with `w`.
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

/// Transform a buffer of characters using a typing method definition.
///
/// # Example
///
/// ```
/// use vi::methods::transform_buffer;
///
/// let mut result = String::new();
/// transform_buffer(&vi::VNI, "viet65".chars(), &mut result);
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(
    definition: &Definition,
    buffer: I,
    output: &mut String,
) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    let mut syllable = Syllable::empty();
    let mut tone_mark_removed = false;
    let mut letter_modification_removed = false;

    let mut last_executed_action = None;

    for ch in buffer {
        let lowercase_ch = ch.to_ascii_lowercase();

        // If a character is not recognised as a transformation character in definition. Skip it.
        if !definition.contains_key(&lowercase_ch) {
            syllable.push(ch);
            continue;
        }

        let fallback = format!("{}{}", syllable, ch);
        let actions = definition.get(&lowercase_ch).unwrap();

        let mut action_iter = actions.iter();
        let mut action = action_iter.next().unwrap();

        loop {
            let transformation = match action {
                Action::AddTonemark(tonemark) => add_tone(&mut syllable, tonemark),
                Action::ModifyLetter(modification) => modify_letter(&mut syllable, modification),
                Action::ModifyLetterOnCharacterFamily(modification, family_char)
                    if syllable.vowel.to_ascii_lowercase().contains(*family_char) =>
                {
                    modify_letter(&mut syllable, modification)
                }
                Action::RemoveToneMark => remove_tone(&mut syllable),
                Action::InsertƯ => {
                    if syllable.vowel.is_empty() || syllable.to_string() == "gi" {
                        syllable.push(if ch.is_lowercase() { 'u' } else { 'U' });
                        let last_index = syllable.len() - 1;
                        syllable.letter_modifications
                            .push((last_index, LetterModification::Horn));
                        Transformation::LetterModificationAdded
                    } else {
                        Transformation::Ignored
                    }
                }
                Action::ResetInsertedƯ if matches!(last_executed_action, Some(Action::InsertƯ)) =>
                {
                    syllable.replace_last_char(ch);
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
                syllable.push(ch);
                last_executed_action = None;
            } else if !is_valid_syllable(&syllable.to_string()) {
                syllable.set(fallback);
                last_executed_action = None;
            } else {
                last_executed_action = Some(action.clone());
            }
            break;
        }
    }

    output.push_str(&syllable.to_string());

    TransformResult {
        tone_mark_removed,
        letter_modification_removed,
    }
}
