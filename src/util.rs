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
pub const fn clean_char(ch: char) -> char {
    match ch {
        // Lowercase a family
        'a' | 'à' | 'ả' | 'ã' | 'á' | 'ạ' | 'ă' | 'ằ' | 'ẳ' | 'ẵ' | 'ắ' | 'ặ' | 'â' | 'ầ' | 'ẩ' | 'ẫ' | 'ấ' | 'ậ' => 'a',
        // Uppercase A family
        'A' | 'À' | 'Ả' | 'Ã' | 'Á' | 'Ạ' | 'Ă' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ắ' | 'Ặ' | 'Â' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ấ' | 'Ậ' => 'A',
        // Lowercase d family
        'd' | 'đ' => 'd',
        // Uppercase D family
        'D' | 'Đ' => 'D',
        // Lowercase e family
        'e' | 'è' | 'ẻ' | 'ẽ' | 'é' | 'ẹ' | 'ê' | 'ề' | 'ể' | 'ễ' | 'ế' | 'ệ' => 'e',
        // Uppercase E family
        'E' | 'È' | 'Ẻ' | 'Ẽ' | 'É' | 'Ẹ' | 'Ê' | 'Ề' | 'Ể' | 'Ễ' | 'Ế' | 'Ệ' => 'E',
        // Lowercase i family
        'i' | 'ì' | 'ỉ' | 'ĩ' | 'í' | 'ị' => 'i',
        // Uppercase I family
        'I' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Í' | 'Ị' => 'I',
        // Lowercase o family
        'o' | 'ò' | 'ỏ' | 'õ' | 'ó' | 'ọ' | 'ô' | 'ồ' | 'ổ' | 'ỗ' | 'ố' | 'ộ' | 'ơ' | 'ờ' | 'ở' | 'ỡ' | 'ớ' | 'ợ' => 'o',
        // Uppercase O family
        'O' | 'Ò' | 'Ỏ' | 'Õ' | 'Ó' | 'Ọ' | 'Ô' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ố' | 'Ộ' | 'Ơ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ớ' | 'Ợ' => 'O',
        // Lowercase u family
        'u' | 'ù' | 'ủ' | 'ũ' | 'ú' | 'ụ' | 'ư' | 'ừ' | 'ử' | 'ữ' | 'ứ' | 'ự' => 'u',
        // Uppercase U family
        'U' | 'Ù' | 'Ủ' | 'Ũ' | 'Ú' | 'Ụ' | 'Ư' | 'Ừ' | 'Ử' | 'Ữ' | 'Ứ' | 'Ự' => 'U',
        // Lowercase y family
        'y' | 'ỳ' | 'ỷ' | 'ỹ' | 'ý' | 'ỵ' => 'y',
        // Uppercase Y family
        'Y' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ý' | 'Ỵ' => 'Y',
        // Any other character remains unchanged
        _ => ch,
    }
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
    matches!(
        c,
        'a' | 'à'
            | 'ả'
            | 'ã'
            | 'á'
            | 'ạ'
            | 'ă'
            | 'ằ'
            | 'ẳ'
            | 'ẵ'
            | 'ắ'
            | 'ặ'
            | 'â'
            | 'ầ'
            | 'ẩ'
            | 'ẫ'
            | 'ấ'
            | 'ậ'
            | 'e'
            | 'è'
            | 'ẻ'
            | 'ẽ'
            | 'é'
            | 'ẹ'
            | 'ê'
            | 'ề'
            | 'ể'
            | 'ễ'
            | 'ế'
            | 'ệ'
            | 'i'
            | 'ì'
            | 'ỉ'
            | 'ĩ'
            | 'í'
            | 'ị'
            | 'o'
            | 'ò'
            | 'ỏ'
            | 'õ'
            | 'ó'
            | 'ọ'
            | 'ô'
            | 'ồ'
            | 'ổ'
            | 'ỗ'
            | 'ố'
            | 'ộ'
            | 'ơ'
            | 'ờ'
            | 'ở'
            | 'ỡ'
            | 'ớ'
            | 'ợ'
            | 'u'
            | 'ù'
            | 'ủ'
            | 'ũ'
            | 'ú'
            | 'ụ'
            | 'ư'
            | 'ừ'
            | 'ử'
            | 'ữ'
            | 'ứ'
            | 'ự'
            | 'y'
            | 'ỳ'
            | 'ỷ'
            | 'ỹ'
            | 'ý'
            | 'ỵ'
            | 'A'
            | 'À'
            | 'Ả'
            | 'Ã'
            | 'Á'
            | 'Ạ'
            | 'Ă'
            | 'Ằ'
            | 'Ẳ'
            | 'Ẵ'
            | 'Ắ'
            | 'Ặ'
            | 'Â'
            | 'Ầ'
            | 'Ẩ'
            | 'Ẫ'
            | 'Ấ'
            | 'Ậ'
            | 'E'
            | 'È'
            | 'Ẻ'
            | 'Ẽ'
            | 'É'
            | 'Ẹ'
            | 'Ê'
            | 'Ề'
            | 'Ể'
            | 'Ễ'
            | 'Ế'
            | 'Ệ'
            | 'I'
            | 'Ì'
            | 'Ỉ'
            | 'Ĩ'
            | 'Í'
            | 'Ị'
            | 'O'
            | 'Ò'
            | 'Ỏ'
            | 'Õ'
            | 'Ó'
            | 'Ọ'
            | 'Ô'
            | 'Ồ'
            | 'Ổ'
            | 'Ỗ'
            | 'Ố'
            | 'Ộ'
            | 'Ơ'
            | 'Ờ'
            | 'Ở'
            | 'Ỡ'
            | 'Ớ'
            | 'Ợ'
            | 'U'
            | 'Ù'
            | 'Ủ'
            | 'Ũ'
            | 'Ú'
            | 'Ụ'
            | 'Ư'
            | 'Ừ'
            | 'Ử'
            | 'Ữ'
            | 'Ứ'
            | 'Ự'
            | 'Y'
            | 'Ỳ'
            | 'Ỷ'
            | 'Ỹ'
            | 'Ý'
            | 'Ỵ'
    )
}
