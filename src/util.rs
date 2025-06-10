//! Useful utilities functions that might be helpful for developing a Vietnamese IME.

/// Strip off tone mark & modifications from an input char.
///
/// This function removes Vietnamese diacritics and tone marks from a character,
/// returning the base character while preserving case.
///
/// # Examples
///
/// ```
/// use vi::util::clean_char;
///
/// assert_eq!(clean_char('á'), 'a');
/// assert_eq!(clean_char('Ế'), 'E');
/// assert_eq!(clean_char('ự'), 'u');
/// ```
#[inline]
pub fn clean_char(ch: char) -> char {
    let is_uppercase = ch.is_uppercase();
    static ACCENTS: [&str; 7] = [
        "aàảãáạăằẳẵắặâầẩẫấậ",
        "dđ",
        "eèẻẽéẹêềểễếệ",
        "iìỉĩíị",
        "oòỏõóọôồổỗốộơờởỡớợ",
        "uùủũúụưừửữứự",
        "yỳỷỹýỵ",
    ];
    let ch_lowercase = ch.to_lowercase().to_string();
    let mut result = ch;
    for accent in ACCENTS {
        if accent.contains(&ch_lowercase) {
            result = accent.chars().next().unwrap();
        }
    }

    if is_uppercase {
        result = result.to_ascii_uppercase();
    }

    result
}

/// Check if a character is a vowel.
///
/// This function checks if the given character is a Vietnamese vowel,
/// including all variations with tone marks and diacritics.
///
/// # Examples
///
/// ```
/// use vi::util::is_vowel;
///
/// assert!(is_vowel('a'));
/// assert!(is_vowel('ế'));
/// assert!(is_vowel('Ư'));
/// assert!(!is_vowel('b'));
/// ```
#[inline]
pub const fn is_vowel(c: char) -> bool {
    // For const fn, we need to use a simpler approach
    matches!(c,
        'a' | 'à' | 'ả' | 'ã' | 'á' | 'ạ' | 'ă' | 'ằ' | 'ẳ' | 'ẵ' | 'ắ' | 'ặ' |
        'â' | 'ầ' | 'ẩ' | 'ẫ' | 'ấ' | 'ậ' | 'e' | 'è' | 'ẻ' | 'ẽ' | 'é' | 'ẹ' |
        'ê' | 'ề' | 'ể' | 'ễ' | 'ế' | 'ệ' | 'i' | 'ì' | 'ỉ' | 'ĩ' | 'í' | 'ị' |
        'o' | 'ò' | 'ỏ' | 'õ' | 'ó' | 'ọ' | 'ô' | 'ồ' | 'ổ' | 'ỗ' | 'ố' | 'ộ' |
        'ơ' | 'ờ' | 'ở' | 'ỡ' | 'ớ' | 'ợ' | 'u' | 'ù' | 'ủ' | 'ũ' | 'ú' | 'ụ' |
        'ư' | 'ừ' | 'ử' | 'ữ' | 'ứ' | 'ự' | 'y' | 'ỳ' | 'ỷ' | 'ỹ' | 'ý' | 'ỵ' |
        'A' | 'À' | 'Ả' | 'Ã' | 'Á' | 'Ạ' | 'Ă' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ắ' | 'Ặ' |
        'Â' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ấ' | 'Ậ' | 'E' | 'È' | 'Ẻ' | 'Ẽ' | 'É' | 'Ẹ' |
        'Ê' | 'Ề' | 'Ể' | 'Ễ' | 'Ế' | 'Ệ' | 'I' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Í' | 'Ị' |
        'O' | 'Ò' | 'Ỏ' | 'Õ' | 'Ó' | 'Ọ' | 'Ô' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ố' | 'Ộ' |
        'Ơ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ớ' | 'Ợ' | 'U' | 'Ù' | 'Ủ' | 'Ũ' | 'Ú' | 'Ụ' |
        'Ư' | 'Ừ' | 'Ử' | 'Ữ' | 'Ứ' | 'Ự' | 'Y' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ý' | 'Ỵ'
    )
}
