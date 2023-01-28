use crate::processor::{add_tone, modify_letter};
use crate::util::modify_letter_or_else;
use crate::validation::is_valid_word;

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
pub fn transform_buffer<I>(buffer: I, output: &mut String)
where
    I: IntoIterator<Item = char>,
{
    let mut result = String::new();
    for ch in buffer {
        let ch = &ch;
        let fallback = format!("{}{}", result, ch);
        let action_performed = match ch.to_ascii_lowercase() {
            's' => add_tone(&mut result, &ToneMark::Acute),
            'f' => add_tone(&mut result, &ToneMark::Grave),
            'r' => add_tone(&mut result, &ToneMark::HookAbove),
            'x' => add_tone(&mut result, &ToneMark::Tilde),
            'j' => add_tone(&mut result, &ToneMark::Underdot),

            'a' | 'e' | 'o' if contains_clean_char(&result, *ch) => {
                modify_letter(&mut result, &LetterModification::Circumflex)
            }
            'w' => modify_letter_or_else(&mut result, &LetterModification::Horn, |result| {
                modify_letter(result, &LetterModification::Breve)
            }),
            'd' => modify_letter(&mut result, &LetterModification::Dyet),
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

#[cfg(test)]
mod tests {
    use super::*;
    fn measure<F>(f: F) where F: FnOnce() {
        use std::time::Instant;
        let clock = Instant::now();
        f();
        println!("Time {:.2?}", clock.elapsed());
    }
    
    #[test]
    fn test_dummy() {
        measure(|| {
            let mut output = String::new();
            transform_buffer("jj".chars(), &mut output);
            println!("{:?}", output);
        });
        measure(|| {
            let mut output = String::new();
            transform_buffer("jjj".chars(), &mut output);
            println!("{:?}", output);
        });
        measure(|| {
            let mut output = String::new();
            transform_buffer("jjjjjjj".chars(), &mut output);
            println!("{:?}", output);
        });
        measure(|| {
            let mut output = String::new();
            transform_buffer("jjjjjjjjjjjjj".chars(), &mut output);
            println!("{:?}", output);
        });
    
        measure(|| {
            let mut output = String::new();
            transform_buffer("ddaay".chars(), &mut output);
            println!("{:?}", output);
        });
    }
}