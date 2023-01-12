use regex::Regex;

use crate::processor::{ToneMark, add_tone, modify_letter, LetterModification, remove_tone};

pub fn clean_char(ch: char) -> char {
    let accents = vec![
        "aàảãáạăằẳẵắặâầẩẫấậ",
        "AÀẢÃÁẠĂẰẲẴẮẶÂẦẨẪẤẬ",
        "dđ",
        "DĐ",
        "eèẻẽéẹêềểễếệ",
        "EÈẺẼÉẸÊỀỂỄẾỆ",
        "iìỉĩíị",
        "IÌỈĨÍỊ",
        "oòỏõóọôồổỗốộơờởỡớợ",
        "OÒỎÕÓỌÔỒỔỖỐỘƠỜỞỠỚỢ",
        "uùủũúụưừửữứự",
        "UÙỦŨÚỤƯỪỬỮỨỰ",
        "yỳỷỹýỵ",
        "YỲỶỸÝỴ",
    ];
    for accent in accents {
        let regex = Regex::new(&format!("[{}]", &accent[1..]));
        let replace_char = accent.chars().next().unwrap();
        if let Ok(re) = regex {
            if re.is_match(&ch.to_string()) {
                return replace_char;
            }
        }
    }
    ch
}

pub fn remove_tone_mark(ch: char) -> char {
    let tone_mark_map = vec![
        "aàảãáạ",
        "ăằẳẵắặ",
        "âầẩẫấậ",
        "AÀẢÃÁẠ",
        "ĂẰẲẴẮẶ",
        "ÂẦẨẪẤẬ",
        "eèẻẽéẹ",
        "êềểễếệ",
        "EÈẺẼÉẸ",
        "ÊỀỂỄẾỆ",
        "iìỉĩíị",
        "IÌỈĨÍỊ",
        "oòỏõóọ",
        "ôồổỗốộ",
        "ơờởỡớợ",
        "OÒỎÕÓỌ",
        "ÔỒỔỖỐỘ",
        "ƠỜỞỠỚỢ",
        "uùủũúụ",
        "ưừửữứự",
        "UÙỦŨÚỤ",
        "ƯỪỬỮỨỰ",
        "yỳỷỹýỵ",
        "YỲỶỸÝỴ",
    ];
    for tone_mark in tone_mark_map {
        let regex = Regex::new(&format!(
            "[{}]",
            &tone_mark.chars().skip(1).collect::<String>()
        ));
        let replace_char = tone_mark.chars().next().unwrap();
        if let Ok(re) = regex {
            if re.is_match(&ch.to_string()) {
                return replace_char;
            }
        }
    }
    ch
}

pub fn add_tone_or_append(input: &mut String, tone_mark: &ToneMark, append_char: &char) {
    let (tone_added, mut result) = add_tone(input, tone_mark);

    if !tone_added {
        // Append the trigger char if tone mark is not added
        result.push(*append_char);
    }

    *input = result
}

pub fn modify_letter_or_append(input: &mut String, modification: &LetterModification, append_char: &char) {
    let (letter_modified, mut result) = modify_letter(input, modification);

    if !letter_modified {
        // Append the trigger char if tone mark is not added
        result.push(*append_char);
    }

    *input = result
}

pub fn remove_tone_or_append(input: &mut String) {
    let mut result = remove_tone(input);

    if result == *input {
        result.push('0');
    }
    *input = result
}