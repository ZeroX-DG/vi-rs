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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_char_a_family() {
        // Lowercase a family
        assert_eq!(clean_char('a'), 'a');
        assert_eq!(clean_char('à'), 'a');
        assert_eq!(clean_char('ả'), 'a');
        assert_eq!(clean_char('ã'), 'a');
        assert_eq!(clean_char('á'), 'a');
        assert_eq!(clean_char('ạ'), 'a');
        assert_eq!(clean_char('ă'), 'a');
        assert_eq!(clean_char('ằ'), 'a');
        assert_eq!(clean_char('ẳ'), 'a');
        assert_eq!(clean_char('ẵ'), 'a');
        assert_eq!(clean_char('ắ'), 'a');
        assert_eq!(clean_char('ặ'), 'a');
        assert_eq!(clean_char('â'), 'a');
        assert_eq!(clean_char('ầ'), 'a');
        assert_eq!(clean_char('ẩ'), 'a');
        assert_eq!(clean_char('ẫ'), 'a');
        assert_eq!(clean_char('ấ'), 'a');
        assert_eq!(clean_char('ậ'), 'a');

        // Uppercase A family
        assert_eq!(clean_char('A'), 'A');
        assert_eq!(clean_char('À'), 'A');
        assert_eq!(clean_char('Ả'), 'A');
        assert_eq!(clean_char('Ã'), 'A');
        assert_eq!(clean_char('Á'), 'A');
        assert_eq!(clean_char('Ạ'), 'A');
        assert_eq!(clean_char('Ă'), 'A');
        assert_eq!(clean_char('Ằ'), 'A');
        assert_eq!(clean_char('Ẳ'), 'A');
        assert_eq!(clean_char('Ẵ'), 'A');
        assert_eq!(clean_char('Ắ'), 'A');
        assert_eq!(clean_char('Ặ'), 'A');
        assert_eq!(clean_char('Â'), 'A');
        assert_eq!(clean_char('Ầ'), 'A');
        assert_eq!(clean_char('Ẩ'), 'A');
        assert_eq!(clean_char('Ẫ'), 'A');
        assert_eq!(clean_char('Ấ'), 'A');
        assert_eq!(clean_char('Ậ'), 'A');
    }

    #[test]
    fn test_clean_char_d_family() {
        // Lowercase d family
        assert_eq!(clean_char('d'), 'd');
        assert_eq!(clean_char('đ'), 'd');

        // Uppercase D family
        assert_eq!(clean_char('D'), 'D');
        assert_eq!(clean_char('Đ'), 'D');
    }

    #[test]
    fn test_clean_char_e_family() {
        // Lowercase e family
        assert_eq!(clean_char('e'), 'e');
        assert_eq!(clean_char('è'), 'e');
        assert_eq!(clean_char('ẻ'), 'e');
        assert_eq!(clean_char('ẽ'), 'e');
        assert_eq!(clean_char('é'), 'e');
        assert_eq!(clean_char('ẹ'), 'e');
        assert_eq!(clean_char('ê'), 'e');
        assert_eq!(clean_char('ề'), 'e');
        assert_eq!(clean_char('ể'), 'e');
        assert_eq!(clean_char('ễ'), 'e');
        assert_eq!(clean_char('ế'), 'e');
        assert_eq!(clean_char('ệ'), 'e');

        // Uppercase E family
        assert_eq!(clean_char('E'), 'E');
        assert_eq!(clean_char('È'), 'E');
        assert_eq!(clean_char('Ẻ'), 'E');
        assert_eq!(clean_char('Ẽ'), 'E');
        assert_eq!(clean_char('É'), 'E');
        assert_eq!(clean_char('Ẹ'), 'E');
        assert_eq!(clean_char('Ê'), 'E');
        assert_eq!(clean_char('Ề'), 'E');
        assert_eq!(clean_char('Ể'), 'E');
        assert_eq!(clean_char('Ễ'), 'E');
        assert_eq!(clean_char('Ế'), 'E');
        assert_eq!(clean_char('Ệ'), 'E');
    }

    #[test]
    fn test_clean_char_i_family() {
        // Lowercase i family
        assert_eq!(clean_char('i'), 'i');
        assert_eq!(clean_char('ì'), 'i');
        assert_eq!(clean_char('ỉ'), 'i');
        assert_eq!(clean_char('ĩ'), 'i');
        assert_eq!(clean_char('í'), 'i');
        assert_eq!(clean_char('ị'), 'i');

        // Uppercase I family
        assert_eq!(clean_char('I'), 'I');
        assert_eq!(clean_char('Ì'), 'I');
        assert_eq!(clean_char('Ỉ'), 'I');
        assert_eq!(clean_char('Ĩ'), 'I');
        assert_eq!(clean_char('Í'), 'I');
        assert_eq!(clean_char('Ị'), 'I');
    }

    #[test]
    fn test_clean_char_o_family() {
        // Lowercase o family
        assert_eq!(clean_char('o'), 'o');
        assert_eq!(clean_char('ò'), 'o');
        assert_eq!(clean_char('ỏ'), 'o');
        assert_eq!(clean_char('õ'), 'o');
        assert_eq!(clean_char('ó'), 'o');
        assert_eq!(clean_char('ọ'), 'o');
        assert_eq!(clean_char('ô'), 'o');
        assert_eq!(clean_char('ồ'), 'o');
        assert_eq!(clean_char('ổ'), 'o');
        assert_eq!(clean_char('ỗ'), 'o');
        assert_eq!(clean_char('ố'), 'o');
        assert_eq!(clean_char('ộ'), 'o');
        assert_eq!(clean_char('ơ'), 'o');
        assert_eq!(clean_char('ờ'), 'o');
        assert_eq!(clean_char('ở'), 'o');
        assert_eq!(clean_char('ỡ'), 'o');
        assert_eq!(clean_char('ớ'), 'o');
        assert_eq!(clean_char('ợ'), 'o');

        // Uppercase O family
        assert_eq!(clean_char('O'), 'O');
        assert_eq!(clean_char('Ò'), 'O');
        assert_eq!(clean_char('Ỏ'), 'O');
        assert_eq!(clean_char('Õ'), 'O');
        assert_eq!(clean_char('Ó'), 'O');
        assert_eq!(clean_char('Ọ'), 'O');
        assert_eq!(clean_char('Ô'), 'O');
        assert_eq!(clean_char('Ồ'), 'O');
        assert_eq!(clean_char('Ổ'), 'O');
        assert_eq!(clean_char('Ỗ'), 'O');
        assert_eq!(clean_char('Ố'), 'O');
        assert_eq!(clean_char('Ộ'), 'O');
        assert_eq!(clean_char('Ơ'), 'O');
        assert_eq!(clean_char('Ờ'), 'O');
        assert_eq!(clean_char('Ở'), 'O');
        assert_eq!(clean_char('Ỡ'), 'O');
        assert_eq!(clean_char('Ớ'), 'O');
        assert_eq!(clean_char('Ợ'), 'O');
    }

    #[test]
    fn test_clean_char_u_family() {
        // Lowercase u family
        assert_eq!(clean_char('u'), 'u');
        assert_eq!(clean_char('ù'), 'u');
        assert_eq!(clean_char('ủ'), 'u');
        assert_eq!(clean_char('ũ'), 'u');
        assert_eq!(clean_char('ú'), 'u');
        assert_eq!(clean_char('ụ'), 'u');
        assert_eq!(clean_char('ư'), 'u');
        assert_eq!(clean_char('ừ'), 'u');
        assert_eq!(clean_char('ử'), 'u');
        assert_eq!(clean_char('ữ'), 'u');
        assert_eq!(clean_char('ứ'), 'u');
        assert_eq!(clean_char('ự'), 'u');

        // Uppercase U family
        assert_eq!(clean_char('U'), 'U');
        assert_eq!(clean_char('Ù'), 'U');
        assert_eq!(clean_char('Ủ'), 'U');
        assert_eq!(clean_char('Ũ'), 'U');
        assert_eq!(clean_char('Ú'), 'U');
        assert_eq!(clean_char('Ụ'), 'U');
        assert_eq!(clean_char('Ư'), 'U');
        assert_eq!(clean_char('Ừ'), 'U');
        assert_eq!(clean_char('Ử'), 'U');
        assert_eq!(clean_char('Ữ'), 'U');
        assert_eq!(clean_char('Ứ'), 'U');
        assert_eq!(clean_char('Ự'), 'U');
    }

    #[test]
    fn test_clean_char_y_family() {
        // Lowercase y family
        assert_eq!(clean_char('y'), 'y');
        assert_eq!(clean_char('ỳ'), 'y');
        assert_eq!(clean_char('ỷ'), 'y');
        assert_eq!(clean_char('ỹ'), 'y');
        assert_eq!(clean_char('ý'), 'y');
        assert_eq!(clean_char('ỵ'), 'y');

        // Uppercase Y family
        assert_eq!(clean_char('Y'), 'Y');
        assert_eq!(clean_char('Ỳ'), 'Y');
        assert_eq!(clean_char('Ỷ'), 'Y');
        assert_eq!(clean_char('Ỹ'), 'Y');
        assert_eq!(clean_char('Ý'), 'Y');
        assert_eq!(clean_char('Ỵ'), 'Y');
    }

    #[test]
    fn test_clean_char_non_vietnamese() {
        // Non-Vietnamese characters should remain unchanged
        assert_eq!(clean_char('b'), 'b');
        assert_eq!(clean_char('B'), 'B');
        assert_eq!(clean_char('c'), 'c');
        assert_eq!(clean_char('C'), 'C');
        assert_eq!(clean_char('f'), 'f');
        assert_eq!(clean_char('F'), 'F');
        assert_eq!(clean_char('g'), 'g');
        assert_eq!(clean_char('G'), 'G');
        assert_eq!(clean_char('h'), 'h');
        assert_eq!(clean_char('H'), 'H');
        assert_eq!(clean_char('j'), 'j');
        assert_eq!(clean_char('J'), 'J');
        assert_eq!(clean_char('k'), 'k');
        assert_eq!(clean_char('K'), 'K');
        assert_eq!(clean_char('l'), 'l');
        assert_eq!(clean_char('L'), 'L');
        assert_eq!(clean_char('m'), 'm');
        assert_eq!(clean_char('M'), 'M');
        assert_eq!(clean_char('n'), 'n');
        assert_eq!(clean_char('N'), 'N');
        assert_eq!(clean_char('p'), 'p');
        assert_eq!(clean_char('P'), 'P');
        assert_eq!(clean_char('q'), 'q');
        assert_eq!(clean_char('Q'), 'Q');
        assert_eq!(clean_char('r'), 'r');
        assert_eq!(clean_char('R'), 'R');
        assert_eq!(clean_char('s'), 's');
        assert_eq!(clean_char('S'), 'S');
        assert_eq!(clean_char('t'), 't');
        assert_eq!(clean_char('T'), 'T');
        assert_eq!(clean_char('v'), 'v');
        assert_eq!(clean_char('V'), 'V');
        assert_eq!(clean_char('w'), 'w');
        assert_eq!(clean_char('W'), 'W');
        assert_eq!(clean_char('x'), 'x');
        assert_eq!(clean_char('X'), 'X');
        assert_eq!(clean_char('z'), 'z');
        assert_eq!(clean_char('Z'), 'Z');
    }

    #[test]
    fn test_clean_char_special_characters() {
        // Special characters and numbers should remain unchanged
        assert_eq!(clean_char('0'), '0');
        assert_eq!(clean_char('1'), '1');
        assert_eq!(clean_char('9'), '9');
        assert_eq!(clean_char(' '), ' ');
        assert_eq!(clean_char('.'), '.');
        assert_eq!(clean_char(','), ',');
        assert_eq!(clean_char('!'), '!');
        assert_eq!(clean_char('?'), '?');
        assert_eq!(clean_char('-'), '-');
        assert_eq!(clean_char('_'), '_');
        assert_eq!(clean_char('('), '(');
        assert_eq!(clean_char(')'), ')');
    }

    #[test]
    fn test_clean_char_const_fn() {
        // Test that the function can be used in const contexts
        const CLEANED_A: char = clean_char('á');
        const CLEANED_E: char = clean_char('Ế');
        const CLEANED_U: char = clean_char('ự');

        assert_eq!(CLEANED_A, 'a');
        assert_eq!(CLEANED_E, 'E');
        assert_eq!(CLEANED_U, 'u');
    }
}
