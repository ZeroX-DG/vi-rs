use crate::util::{add_tone_or_append, modify_letter_or_append};

use super::processor::{LetterModification, ToneMark};

use super::util::clean_char;

fn modifiable_char(ch: &char, previous_ch: &char, modification: &LetterModification) -> bool {
    let clean_previous_ch = clean_char(*previous_ch);

    match modification {
        LetterModification::Circumflex => match clean_previous_ch {
            'a' | 'e' | 'o' | 'A' | 'E' | 'O' => {
                clean_previous_ch.to_ascii_lowercase() == ch.to_ascii_lowercase()
            }
            _ => false,
        },
        LetterModification::Horn => match clean_previous_ch {
            'u' | 'o' | 'U' | 'O' => true,
            _ => false,
        },
        LetterModification::Breve => match clean_previous_ch {
            'a' | 'A' => true,
            _ => false,
        },
        LetterModification::Dyet => match clean_previous_ch {
            'd' | 'D' => true,
            _ => false,
        },
    }
}

/// Transform input buffer to vietnamese string output using telex mode.
///
/// # Example
/// ```
/// use vi::telex::transform_buffer;
///
/// let result = transform_buffer(vec!['v', 'i', 'e', 'e', 't', 'j'].iter().cloned());
/// assert_eq!(result, "việt".to_owned());
/// ```
pub fn transform_buffer<I>(buffer: I) -> String
where
    I: IntoIterator<Item = char>,
{
    let mut content = String::new();

    let mut previous_ch = '\0';
    for ch in buffer {
        let ch = &ch;
        match ch {
            's' => add_tone_or_append(&mut content, &ToneMark::Acute, ch),
            'f' => add_tone_or_append(&mut content, &ToneMark::Grave, ch),
            'r' => add_tone_or_append(&mut content, &ToneMark::HookAbove, ch),
            'x' => add_tone_or_append(&mut content, &ToneMark::Tilde, ch),
            'j' => add_tone_or_append(&mut content, &ToneMark::Underdot, ch),

            'a' | 'e' | 'o'
                if modifiable_char(ch, &previous_ch, &LetterModification::Circumflex) =>
            {
                modify_letter_or_append(&mut content, &LetterModification::Circumflex, ch);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Horn) => {
                modify_letter_or_append(&mut content, &LetterModification::Horn, ch);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Breve) => {
                modify_letter_or_append(&mut content, &LetterModification::Breve, ch);
            }
            'd' if modifiable_char(ch, &previous_ch, &LetterModification::Dyet) => {
                modify_letter_or_append(&mut content, &LetterModification::Dyet, ch);
            }
            _ => content.push(*ch),
        }
        previous_ch = content.chars().last().unwrap_or('\0');
    }

    content
}
