use super::processor::{
    Action, ToneMark, LetterModification,
    add_tone, modify_letter
};

use super::util::clean_char;

fn modifiable_char(ch: &char, previous_ch: &char, modification: &LetterModification) -> bool {
    let clean_previous_ch = clean_char(*previous_ch);

    match modification {
        LetterModification::Circumflex => match clean_previous_ch {
            'a' | 'e' | 'o' | 'A' | 'E' | 'O' => clean_previous_ch.to_ascii_lowercase() == ch.to_ascii_lowercase(),
            _ => false
        }
        LetterModification::Horn => match clean_previous_ch {
            'u' | 'o' | 'U' | 'O' => true,
            _ => false
        }
        LetterModification::Breve => match clean_previous_ch {
            'a'| 'A' => true,
            _ => false
        }
        LetterModification::Dyet => match clean_previous_ch {
            'd' | 'D' => true,
            _ => false
        }
    }
}

fn modify_char(ch: &char, modification: &LetterModification) -> char {
    modify_letter(&ch.to_string(), modification).1.chars().last().unwrap()
}

/// Transform input buffer to vietnamese string output along with
/// a bool indicating if an action has been triggered. For example,
/// if the input is `['a', 'a']`, then the action add tone mark is 
/// triggered by the `a` character.
///
/// # Example
/// ```
/// use vi::telex::transform_buffer;
///
/// let result = transform_buffer(&vec!['v', 'i', 'e', 'e', 't', 'j']);
/// assert_eq!(result, (true, "việt".to_owned()));
/// ```
pub fn transform_buffer(buffer: &[char]) -> (bool, String) {
    let mut content = String::new();

    let mut actions: Vec<Action> = Vec::new();
    let mut previous_ch = '\0';
    for ch in buffer {
        match ch {
            's' => actions.push(Action::AddTone(ToneMark::Acute)),
            'f' => actions.push(Action::AddTone(ToneMark::Grave)),
            'r' => actions.push(Action::AddTone(ToneMark::HookAbove)),
            'x' => actions.push(Action::AddTone(ToneMark::Tilde)),
            'j' => actions.push(Action::AddTone(ToneMark::Underdot)),

            'a' | 'e' | 'o' if modifiable_char(ch, &previous_ch, &LetterModification::Circumflex) => {
                actions.push(Action::ModifyLetter(LetterModification::Circumflex));
                previous_ch = modify_char(&previous_ch, &LetterModification::Circumflex);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Horn) => {
                actions.push(Action::ModifyLetter(LetterModification::Horn));
                previous_ch = modify_char(&previous_ch, &LetterModification::Horn);
            }
            'w' if modifiable_char(ch, &previous_ch, &LetterModification::Breve) => {
                actions.push(Action::ModifyLetter(LetterModification::Breve));
                previous_ch = modify_char(&previous_ch, &LetterModification::Breve);
            }
            'd' if modifiable_char(ch, &previous_ch, &LetterModification::Dyet) => {
                actions.push(Action::ModifyLetter(LetterModification::Dyet));
                previous_ch = modify_char(&previous_ch, &LetterModification::Dyet);
            }
            _ => {
                content.push(*ch);
                previous_ch = *ch;
            },
        }
    }

    let action_count = actions.len();
    let has_action = if content.len() > 0 {
        action_count > 0
    } else {
        false
    };

    for action in actions {
        match action {
            Action::AddTone(tone_mark) => {
                let (add_success, new_content) = add_tone(&content, &tone_mark);

                content = new_content;

                if !add_success {
                    let trigger_ch = match tone_mark {
                        ToneMark::Acute     => 's',
                        ToneMark::Grave     => 'f',
                        ToneMark::HookAbove => 'r',
                        ToneMark::Tilde     => 'x',
                        ToneMark::Underdot  => 'j'
                    };
                    content.push(trigger_ch);
                }
            }
            Action::ModifyLetter(modification) => {
                let (modify_success, new_content) = modify_letter(&content, &modification);

                content = new_content;

                if !modify_success {
                    // Do nothing since this case will never happen with telex
                }
            }
            Action::RemoveTone => {
                // Do nothing since this case will never happen with telex
            }
        }
    }

    (has_action, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_acute_tone_normal() {
        let input: Vec<char> = vec!['v', 'i', 't', 's'];
        let (_, result) = transform_buffer(&input);
        let expected = "vít".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_acute_tone_failed() {
        let input: Vec<char> = vec!['v', 't', 's'];
        let (_, result) = transform_buffer(&input);
        let expected = "vts".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_normal() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', 'f'];
        let (_, result) = transform_buffer(&input);
        let expected = "hoàng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_double_edit() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', 'f', 'r'];
        let (_, result) = transform_buffer(&input);
        let expected = "hoảng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_no_content() {
        let input: Vec<char> = vec!['2', '3'];
        let (has_action, result) = transform_buffer(&input);
        let expected = "23".to_string();
        assert_eq!(result, expected);
        assert_eq!(has_action, false);
    }

    #[test]
    fn add_tone_overflow() {
        let input: Vec<char> = vec!['a', 's', 's'];
        let (has_action, result) = transform_buffer(&input);
        let expected = "as".to_string();
        assert_eq!(result, expected);
        assert_eq!(has_action, true);
    }

    #[test]
    fn add_tone_all_uppercase() {
        let input: Vec<char> = vec!['C', 'H', 'A', 'O', 'f'];
        let (has_action, result) = transform_buffer(&input);
        let expected = "CHÀO".to_string();
        assert_eq!(result, expected);
        assert_eq!(has_action, true);
    }

    #[test]
    fn modify_letter_normal() {
        let input: Vec<char> = vec!['v', 'o', 'w'];
        let (_, result) = transform_buffer(&input);
        let expected = "vơ".to_string();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn modify_letter_group() {
        let input: Vec<char> = vec!['v', 'u', 'o', 'w', 'n'];
        let (_, result) = transform_buffer(&input);
        let expected = "vươn".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_failed() {
        let input: Vec<char> = vec!['c', 'h', 'e', 'w'];
        let (_, result) = transform_buffer(&input);
        let expected = "chew".to_string();
        assert_eq!(result, expected);

        let input: Vec<char> = vec!['v', 'u', 'o', 'n', 'w'];
        let (_, result) = transform_buffer(&input);
        let expected = "vuonw".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_uppercase() {
        let input: Vec<char> = vec!['c', 'h', 'E', 'e'];
        let (_, result) = transform_buffer(&input);
        let expected = "chÊ".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_with_existing_tone() {
        let input: Vec<char> = vec!['c', 'h', 'e', 'j', 'e', 'c', 'h'];
        let (_, result) = transform_buffer(&input);
        let expected = "chệch".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_override() {
        let input: Vec<char> = vec!['a', 'a', 'w'];
        let (_, result) = transform_buffer(&input);
        let expected = "ă".to_string();
        assert_eq!(result, expected);
    }
}