use regex::Regex;

pub fn clean_char(ch: char) -> char {
    let accents = vec![
        "aàảãáạăằẳẵắặâầẩẫấậ",
        "AÀẢÃÁẠĂẰẲẴẮẶÂẦẨẪẤẬ",
        "dđ", "DĐ",
        "eèẻẽéẹêềểễếệ",
        "EÈẺẼÉẸÊỀỂỄẾỆ",
        "iìỉĩíị",
        "IÌỈĨÍỊ",
        "oòỏõóọôồổỗốộơờởỡớợ",
        "OÒỎÕÓỌÔỒỔỖỐỘƠỜỞỠỚỢ",
        "uùủũúụưừửữứự",
        "UÙỦŨÚỤƯỪỬỮỨỰ",
        "yỳỷỹýỵ",
        "YỲỶỸÝỴ"
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
        "YỲỶỸÝỴ"
    ];
    for tone_mark in tone_mark_map {
        let regex = Regex::new(&format!("[{}]", &tone_mark
                                        .chars()
                                        .skip(1)
                                        .collect::<String>()));
        let replace_char = tone_mark.chars().next().unwrap();
        if let Ok(re) = regex {
            if re.is_match(&ch.to_string()) {
                return replace_char;
            }
        }
    }
    ch
}
