use super::processor::{
    Action, ToneMark, LetterModification,
    add_tone, remove_tone, modify_letter
};

fn is_number(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false
    }
}

/// Transform input buffer to vietnamese string output along with
/// a bool indicating if an ction has been triggered. For example,
/// if the input is `['a', '1']`, then the action add tone mar is 
/// triggered by the `1` character.
///
/// # Example
/// ```
/// use vi::vni::transform_buffer;
///
/// let result = transform_buffer(&vec!['v', 'i', 'e', 't', '6', '5']);
/// assert_eq!(result, (true, "việt".to_owned()));
/// ```
pub fn transform_buffer(buffer: &[char]) -> (bool, String) {
    let mut content = String::new();
    let mut actions: Vec<Action> = Vec::new();
    for ch in buffer {
        if is_number(*ch) {
            // in vni, number denote an action like adding tone mark, remove
            // tone mark and changing letter to modified vietnamese letter.
            match ch {
                '1' => actions.push(Action::AddTone(ToneMark::Acute)),
                '2' => actions.push(Action::AddTone(ToneMark::Grave)),
                '3' => actions.push(Action::AddTone(ToneMark::HookAbove)),
                '4' => actions.push(Action::AddTone(ToneMark::Tilde)),
                '5' => actions.push(Action::AddTone(ToneMark::Underdot)),
                '6' => actions.push(Action::ModifyLetter(LetterModification::Circumflex)),
                '7' => actions.push(Action::ModifyLetter(LetterModification::Horn)),
                '8' => actions.push(Action::ModifyLetter(LetterModification::Breve)),
                '9' => actions.push(Action::ModifyLetter(LetterModification::Dyet)),
                '0' => actions.push(Action::RemoveTone),
                _ => {}
            }
        } else {
            content.push(*ch);
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
                        ToneMark::Acute     => '1',
                        ToneMark::Grave     => '2',
                        ToneMark::HookAbove => '3',
                        ToneMark::Tilde     => '4',
                        ToneMark::Underdot  => '5'
                    };
                    content.push(trigger_ch);
                }
            }
            Action::ModifyLetter(modification) => {
                let (modify_success, new_content) = modify_letter(&content, &modification);

                content = new_content;

                if !modify_success {
                    let trigger_ch = match modification {
                        LetterModification::Dyet       => '9',
                        LetterModification::Breve      => '8',
                        LetterModification::Horn       => '7',
                        LetterModification::Circumflex => '6'
                    };
                    content.push(trigger_ch);
                }
            }
            Action::RemoveTone => {
                let new_content = remove_tone(&content);
                if new_content == content {
                    content.push('0');
                } else {
                    content = new_content;
                }
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
        let input: Vec<char> = vec!['v', 'i', 't', '1'];
        let (_, result) = transform_buffer(&input);
        let expected = "vít".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_acute_tone_failed() {
        let input: Vec<char> = vec!['v', 't', '1'];
        let (_, result) = transform_buffer(&input);
        let expected = "vt1".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_normal() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', '2'];
        let (_, result) = transform_buffer(&input);
        let expected = "hoàng".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn add_tone_double_edit() {
        let input: Vec<char> = vec!['h', 'o', 'a', 'n', 'g', '2', '3'];
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
        let input: Vec<char> = vec!['a', '1', '1'];
        let (has_action, result) = transform_buffer(&input);
        let expected = "a1".to_string();
        assert_eq!(result, expected);
        assert_eq!(has_action, true);
    }

    #[test]
    fn add_tone_all_uppercase() {
        let input: Vec<char> = vec!['C', 'H', 'A', 'O', '2'];
        let (has_action, result) = transform_buffer(&input);
        let expected = "CHÀO".to_string();
        assert_eq!(result, expected);
        assert_eq!(has_action, true);
    }

    #[test]
    fn remove_tone_single() {
        let input: Vec<char> = vec!['l', 'u', 'a', 't', '6', '5', '0'];
        let (_, result) = transform_buffer(&input);
        let expected = "luât".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn remove_tone_double() {
        let input: Vec<char> = vec!['c', 'h', 'e', 't', '6', '1', '0', '0'];
        let (_, result) = transform_buffer(&input);
        let expected = "chet".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn remove_tone_exceed() {
        let input: Vec<char> = vec!['v', 'i', 't', '5', '0', '0'];
        let (_, result) = transform_buffer(&input);
        let expected = "vit0".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_normal() {
        let input: Vec<char> = vec!['v', 'o', '7'];
        let (_, result) = transform_buffer(&input);
        let expected = "vơ".to_string();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn modify_letter_group() {
        let input: Vec<char> = vec!['v', 'u', 'o', 'n', '7'];
        let (_, result) = transform_buffer(&input);
        let expected = "vươn".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_failed() {
        let input: Vec<char> = vec!['c', 'h', 'e', '7'];
        let (_, result) = transform_buffer(&input);
        let expected = "che7".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn modify_letter_uppercase() {
        let input: Vec<char> = vec!['c', 'h', 'E', '6'];
        let (_, result) = transform_buffer(&input);
        let expected = "chÊ".to_string();
        assert_eq!(result, expected);
    }
}
