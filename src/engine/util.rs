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
        let replace_char = accent.chars().nth(0).unwrap();
        match regex {
            Ok(re) => {
                if re.is_match(&ch.to_string()) {
                    return replace_char;
                }
            },
            Err(_) => {}
        }
    }
    ch
}

pub fn remove_accents(ch: char) -> char {
    let accents = vec![
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
    for accent in accents {
        let regex = Regex::new(&format!("[{}]", &accent
                                        .chars()
                                        .skip(1)
                                        .collect::<String>()));
        let replace_char = accent.chars().nth(0).unwrap();
        match regex {
            Ok(re) => {
                if re.is_match(&ch.to_string()) {
                    return replace_char;
                }
            },
            Err(_) => {}
        }
    }
    ch
}
