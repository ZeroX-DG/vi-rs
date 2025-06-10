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
        add_tone, modify_letter, remove_tone, AccentStyle, LetterModification, ToneMark,
        Transformation,
    },
    syllable::Syllable,
    validation::is_valid_syllable,
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

/// Transforms a buffer of characters using a typing method definition with a given accent style.
///
/// This is the customizable version of [`transform_buffer`] that lets you choose how accents are applied.
///
/// # Example
///
/// ```
/// use vi::{
///     processor::AccentStyle,
///     methods::transform_buffer_with_style
/// };
///
/// let mut result = String::new();
/// transform_buffer_with_style(&vi::TELEX, AccentStyle::Old, "hoas".chars(), &mut result);
/// assert_eq!(result, "hóa".to_owned());
/// ```
pub fn transform_buffer_with_style<I>(
    definition: &Definition,
    accent_style: AccentStyle,
    buffer: I,
    output: &mut String,
) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    let mut syllable = Syllable {
        accent_style,
        ..Default::default()
    };

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
                        syllable
                            .letter_modifications
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
    transform_buffer_with_style(definition, AccentStyle::default(), buffer, output)
}

/// An incremental buffer for character-by-character Vietnamese text transformation.
///
/// This structure allows for incremental processing of Vietnamese input, where characters
/// are added one at a time and the transformation result can be viewed at any point.
/// This is particularly useful for input method engines that need to display preview
/// text as the user types.
///
/// # Memory Optimization
///
/// The buffer caches the current syllable state and transformation history to avoid
/// recomputing the entire transformation on each character addition.
///
/// # Examples
///
/// ```
/// use vi::methods::transform_buffer_incremental;
///
/// let mut buffer = transform_buffer_incremental(&vi::TELEX);
///
/// buffer.push('v');
/// assert_eq!(buffer.view(), "v");
///
/// buffer.push('i');
/// assert_eq!(buffer.view(), "vi");
///
/// buffer.push('e');
/// assert_eq!(buffer.view(), "vie");
///
/// buffer.push('t');
/// assert_eq!(buffer.view(), "viet");
///
/// buffer.push('s');
/// assert_eq!(buffer.view(), "viét");
/// ```
#[derive(Debug, Clone)]
pub struct IncrementalBuffer<'def> {
    /// Reference to the typing method definition
    definition: &'def Definition,
    /// The current syllable state
    syllable: Syllable,
    /// Input characters received so far
    input: Vec<char>,
    /// Cached output string to avoid recomputation
    output: String,
    /// Cumulative transformation result
    result: TransformResult,
    /// The last executed action for state tracking
    last_executed_action: Option<Action>,
}

impl<'def> IncrementalBuffer<'def> {
    /// Creates a new incremental buffer with the specified typing definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::IncrementalBuffer;
    ///
    /// let buffer = IncrementalBuffer::new(&vi::TELEX);
    /// assert_eq!(buffer.view(), "");
    /// ```
    #[inline]
    pub fn new(definition: &'def Definition) -> Self {
        Self::new_with_style(definition, AccentStyle::default())
    }

    /// Creates a new incremental buffer with the specified typing definition and accent style.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::{methods::IncrementalBuffer, processor::AccentStyle};
    ///
    /// let buffer = IncrementalBuffer::new_with_style(&vi::TELEX, AccentStyle::Old);
    /// assert_eq!(buffer.view(), "");
    /// ```
    #[inline]
    pub fn new_with_style(definition: &'def Definition, accent_style: AccentStyle) -> Self {
        Self {
            definition,
            syllable: Syllable {
                accent_style,
                ..Default::default()
            },
            input: Vec::new(),
            output: String::new(),
            result: TransformResult {
                tone_mark_removed: false,
                letter_modification_removed: false,
            },
            last_executed_action: None,
        }
    }

    /// Adds a character to the buffer and updates the transformation result.
    ///
    /// Returns the transformation result for this character addition.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::VNI);
    /// let result = buffer.push('v');
    /// assert_eq!(buffer.view(), "v");
    /// ```
    #[must_use]
    pub fn push(&mut self, ch: char) -> TransformResult {
        self.input.push(ch);

        let lowercase_ch = ch.to_ascii_lowercase();

        // If a character is not recognised as a transformation character in definition. Skip it.
        if !self.definition.contains_key(&lowercase_ch) {
            self.syllable.push(ch);
            self.update_output();
            return TransformResult {
                tone_mark_removed: false,
                letter_modification_removed: false,
            };
        }

        let fallback = format!("{}{}", self.syllable, ch);
        let actions = self.definition.get(&lowercase_ch).unwrap();

        let mut action_iter = actions.iter();
        let mut action = action_iter.next().unwrap();

        let mut char_result = TransformResult {
            tone_mark_removed: false,
            letter_modification_removed: false,
        };

        loop {
            let transformation = match action {
                Action::AddTonemark(tonemark) => add_tone(&mut self.syllable, tonemark),
                Action::ModifyLetter(modification) => modify_letter(&mut self.syllable, modification),
                Action::ModifyLetterOnCharacterFamily(modification, family_char)
                    if self.syllable.vowel.to_ascii_lowercase().contains(*family_char) =>
                {
                    modify_letter(&mut self.syllable, modification)
                }
                Action::RemoveToneMark => remove_tone(&mut self.syllable),
                Action::InsertƯ => {
                    if self.syllable.vowel.is_empty() || self.syllable.to_string() == "gi" {
                        self.syllable.push(if ch.is_lowercase() { 'u' } else { 'U' });
                        let last_index = self.syllable.len() - 1;
                        self.syllable
                            .letter_modifications
                            .push((last_index, LetterModification::Horn));
                        Transformation::LetterModificationAdded
                    } else {
                        Transformation::Ignored
                    }
                }
                Action::ResetInsertedƯ if matches!(self.last_executed_action, Some(Action::InsertƯ)) =>
                {
                    self.syllable.replace_last_char(ch);
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
                char_result.tone_mark_removed = true;
                self.result.tone_mark_removed = true;
            }

            if transformation == Transformation::LetterModificationRemoved {
                char_result.letter_modification_removed = true;
                self.result.letter_modification_removed = true;
            }

            let action_performed = match transformation {
                Transformation::Ignored | Transformation::LetterModificationRemoved => false,
                // If tone mark was intentionally removed with z character then it's count as an action.
                Transformation::ToneMarkRemoved => *action == Action::RemoveToneMark,
                _ => true,
            };

            // If the action is to trigger reset ư insert then we don't need further processing
            if *action == Action::ResetInsertedƯ {
                self.last_executed_action = Some(action.clone());
                break;
            }

            if !action_performed {
                self.syllable.push(ch);
                self.last_executed_action = None;
            } else if !is_valid_syllable(&self.syllable.to_string()) {
                self.syllable.set(fallback);
                self.last_executed_action = None;
            } else {
                self.last_executed_action = Some(action.clone());
            }
            break;
        }

        self.update_output();
        char_result
    }

    /// Returns the current transformed output as a string slice.
    ///
    /// This provides immediate access to the current state of the transformation
    /// without needing to recompute it.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// buffer.push('v');
    /// buffer.push('i');
    /// buffer.push('e');
    /// buffer.push('t');
    /// buffer.push('s');
    /// assert_eq!(buffer.view(), "viét");
    /// ```
    #[inline]
    pub fn view(&self) -> &str {
        &self.output
    }

    /// Returns the cumulative transformation result.
    ///
    /// This includes information about whether any tone marks or letter modifications
    /// have been removed during the entire transformation process.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// buffer.push('v');
    /// buffer.push('i');
    /// buffer.push('e');
    /// buffer.push('t');
    /// buffer.push('s');
    /// buffer.push('z'); // Remove tone mark
    ///
    /// let result = buffer.result();
    /// assert!(result.tone_mark_removed);
    /// ```
    #[inline]
    pub fn result(&self) -> &TransformResult {
        &self.result
    }

    /// Returns the input characters received so far.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// buffer.push('v');
    /// buffer.push('i');
    /// buffer.push('e');
    /// buffer.push('t');
    /// buffer.push('s');
    ///
    /// assert_eq!(buffer.input(), &['v', 'i', 'e', 't', 's']);
    /// ```
    #[inline]
    pub fn input(&self) -> &[char] {
        &self.input
    }

    /// Clears the buffer, resetting it to an empty state.
    ///
    /// This removes all input characters, resets the syllable state, and clears
    /// the output and transformation results.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// buffer.push('v');
    /// buffer.push('i');
    /// buffer.push('e');
    /// buffer.push('t');
    /// buffer.push('s');
    /// assert_eq!(buffer.view(), "viét");
    ///
    /// buffer.clear();
    /// assert_eq!(buffer.view(), "");
    /// assert!(buffer.input().is_empty());
    /// ```
    pub fn clear(&mut self) {
        let accent_style = self.syllable.accent_style.clone();
        self.syllable = Syllable {
            accent_style,
            ..Default::default()
        };
        self.input.clear();
        self.output.clear();
        self.result = TransformResult {
            tone_mark_removed: false,
            letter_modification_removed: false,
        };
        self.last_executed_action = None;
    }

    /// Returns whether the buffer is empty (contains no input characters).
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// assert!(buffer.is_empty());
    ///
    /// buffer.push('v');
    /// assert!(!buffer.is_empty());
    ///
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Returns the number of input characters in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use vi::methods::transform_buffer_incremental;
    ///
    /// let mut buffer = transform_buffer_incremental(&vi::TELEX);
    /// assert_eq!(buffer.len(), 0);
    ///
    /// buffer.push('v');
    /// buffer.push('i');
    /// buffer.push('e');
    /// buffer.push('t');
    /// buffer.push('s');
    /// assert_eq!(buffer.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Updates the cached output string from the current syllable state.
    ///
    /// This is called internally after each transformation to maintain
    /// the cached output for efficient access via `view()`.
    fn update_output(&mut self) {
        self.output.clear();
        self.output.push_str(&self.syllable.to_string());
    }
}

/// Creates a new incremental buffer for character-by-character Vietnamese text transformation.
///
/// This is a convenience function that creates an [`IncrementalBuffer`] with the default
/// accent style.
///
/// # Examples
///
/// ```
/// use vi::methods::transform_buffer_incremental;
///
/// let mut buffer = transform_buffer_incremental(&vi::TELEX);
/// buffer.push('v');
/// buffer.push('i');
/// buffer.push('e');
/// buffer.push('t');
/// buffer.push('s');
/// assert_eq!(buffer.view(), "viét");
/// ```
#[inline]
pub fn transform_buffer_incremental(definition: &Definition) -> IncrementalBuffer<'_> {
    IncrementalBuffer::new(definition)
}

/// Creates a new incremental buffer with a specific accent style.
///
/// This allows you to choose between old and new accent placement styles.
///
/// # Examples
///
/// ```
/// use vi::{methods::transform_buffer_incremental_with_style, processor::AccentStyle};
///
/// let mut buffer = transform_buffer_incremental_with_style(&vi::TELEX, AccentStyle::Old);
/// buffer.push('h');
/// buffer.push('o');
/// buffer.push('a');
/// buffer.push('s');
/// assert_eq!(buffer.view(), "hóa");
/// ```
#[inline]
pub fn transform_buffer_incremental_with_style(
    definition: &Definition,
    accent_style: AccentStyle,
) -> IncrementalBuffer<'_> {
    IncrementalBuffer::new_with_style(definition, accent_style)
}
