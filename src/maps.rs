//! Static character mappings for transformation.

use phf::{Map, OrderedSet, phf_map, phf_ordered_set};

/// An ordered set of lowercase vowels, complete with and without accents or tone marks.
pub static VOWELS: OrderedSet<char> = phf_ordered_set![
    'a', 'à', 'ả', 'ã', 'á', 'ạ', 'ă', 'ằ', 'ẳ', 'ẵ', 'ắ', 'ặ', 'â', 'ầ', 'ẩ', 'ẫ', 'ấ', 'ậ', 'e',
    'è', 'ẻ', 'ẽ', 'é', 'ẹ', 'ê', 'ề', 'ể', 'ễ', 'ế', 'ệ', 'i', 'ì', 'ỉ', 'ĩ', 'í', 'ị', 'o', 'ò',
    'ỏ', 'õ', 'ó', 'ọ', 'ô', 'ồ', 'ổ', 'ỗ', 'ố', 'ộ', 'ơ', 'ờ', 'ở', 'ỡ', 'ớ', 'ợ', 'u', 'ù', 'ủ',
    'ũ', 'ú', 'ụ', 'ư', 'ừ', 'ử', 'ữ', 'ứ', 'ự', 'y', 'ỳ', 'ỷ', 'ỹ', 'ý', 'ỵ'
];

/// A map of characters without tone mark to character with acute tone mark
pub static ACCUTE_MAP: Map<char, char> = phf_map! {
    'a' => 'á',
    'â' => 'ấ',
    'ă' => 'ắ',
    'e' => 'é',
    'ê' => 'ế',
    'i' => 'í',
    'o' => 'ó',
    'ô' => 'ố',
    'ơ' => 'ớ',
    'u' => 'ú',
    'ư' => 'ứ',
    'y' => 'ý',
    // uppercase
    'A' => 'Á',
    'Â' => 'Ấ',
    'Ă' => 'Ắ',
    'E' => 'É',
    'Ê' => 'Ế',
    'I' => 'Í',
    'O' => 'Ó',
    'Ô' => 'Ố',
    'Ơ' => 'Ớ',
    'U' => 'Ú',
    'Ư' => 'Ứ',
    'Y' => 'Ý',
};

/// A map of characters without tone mark to character with grave tone mark
pub static GRAVE_MAP: Map<char, char> = phf_map! {
    'a' => 'à',
    'â' => 'ầ',
    'ă' => 'ằ',
    'e' => 'è',
    'ê' => 'ề',
    'i' => 'ì',
    'o' => 'ò',
    'ô' => 'ồ',
    'ơ' => 'ờ',
    'u' => 'ù',
    'ư' => 'ừ',
    'y' => 'ỳ',
    // uppercase
    'A' => 'À',
    'Â' => 'Ầ',
    'Ă' => 'Ằ',
    'E' => 'È',
    'Ê' => 'Ề',
    'I' => 'Ì',
    'O' => 'Ò',
    'Ô' => 'Ồ',
    'Ơ' => 'Ờ',
    'U' => 'Ù',
    'Ư' => 'Ừ',
    'Y' => 'Ỳ',
};

/// A map of characters without tone mark to character with hook above tone mark
pub static HOOK_ABOVE_MAP: Map<char, char> = phf_map! {
    'a' => 'ả',
    'â' => 'ẩ',
    'ă' => 'ẳ',
    'e' => 'ẻ',
    'ê' => 'ể',
    'i' => 'ỉ',
    'o' => 'ỏ',
    'ô' => 'ổ',
    'ơ' => 'ở',
    'u' => 'ủ',
    'ư' => 'ử',
    'y' => 'ỷ',
    // uppercase
    'A' => 'Ả',
    'Ă' => 'Ẳ',
    'Â' => 'Ẩ',
    'E' => 'Ẻ',
    'Ê' => 'Ể',
    'O' => 'Ỏ',
    'Ô' => 'Ổ',
    'Ơ' => 'Ở',
    'I' => 'Ỉ',
    'U' => 'Ủ',
    'Ư' => 'Ử',
    'Y' => 'Ỷ',
};

/// A map of characters without tone mark to character with tilde tone mark
pub static TILDE_MAP: Map<char, char> = phf_map! {
    'a' => 'ã',
    'ă' => 'ẵ',
    'â' => 'ẫ',
    'e' => 'ẽ',
    'ê' => 'ễ',
    'o' => 'õ',
    'ô' => 'ỗ',
    'ơ' => 'ỡ',
    'i' => 'ĩ',
    'u' => 'ũ',
    'ư' => 'ữ',
    'y' => 'ỹ',
    // uppercase
    'A' => 'Ã',
    'Ă' => 'Ẵ',
    'Â' => 'Ẫ',
    'E' => 'Ẽ',
    'Ê' => 'Ễ',
    'O' => 'Õ',
    'Ô' => 'Ỗ',
    'Ơ' => 'Ỡ',
    'I' => 'Ĩ',
    'U' => 'Ũ',
    'Ư' => 'Ữ',
    'Y' => 'Ỹ',
};

/// A map of characters without tone mark to character with dot tone mark
pub static DOT_MAP: Map<char, char> = phf_map! {
    'a' => 'ạ',
    'ă' => 'ặ',
    'â' => 'ậ',
    'e' => 'ẹ',
    'ê' => 'ệ',
    'o' => 'ọ',
    'ô' => 'ộ',
    'ơ' => 'ợ',
    'i' => 'ị',
    'u' => 'ụ',
    'ư' => 'ự',
    'y' => 'ỵ',
    // uppercase
    'A' => 'Ạ',
    'Ă' => 'Ặ',
    'Â' => 'Ậ',
    'E' => 'Ẹ',
    'Ê' => 'Ệ',
    'O' => 'Ọ',
    'Ô' => 'Ộ',
    'Ơ' => 'Ợ',
    'I' => 'Ị',
    'U' => 'Ụ',
    'Ư' => 'Ự',
    'Y' => 'Ỵ',
};

/// A map of characters without accent to character with circumflex accent
pub static CIRCUMFLEX_MAP: Map<char, char> = phf_map! {
    'a' => 'â',
    'e' => 'ê',
    'o' => 'ô',
    'ạ' => 'ậ',
    'ẹ' => 'ệ',
    'ọ' => 'ộ',
    'á' => 'ấ',
    'é' => 'ế',
    'ó' => 'ố',
    'ả' => 'ẩ',
    'ẻ' => 'ể',
    'ỏ' => 'ổ',
    'ã' => 'ẫ',
    'ẽ' => 'ễ',
    'õ' => 'ỗ',
    'à' => 'ầ',
    'è' => 'ề',
    'ò' => 'ồ',
    // uppercase
    'A' => 'Â',
    'E' => 'Ê',
    'O' => 'Ô',
    'Ạ' => 'Ậ',
    'Ẹ' => 'Ệ',
    'Ọ' => 'Ộ',
    'Á' => 'Ấ',
    'É' => 'Ế',
    'Ó' => 'Ố',
    'Ả' => 'Ẩ',
    'Ẻ' => 'Ể',
    'Ỏ' => 'Ổ',
    'Ã' => 'Ẫ',
    'Ẽ' => 'Ễ',
    'Õ' => 'Ỗ',
    'À' => 'Ầ',
    'È' => 'Ề',
    'Ò' => 'Ồ',
};

/// A map of characters without accent to character with dyet accent
pub static DYET_MAP: Map<char, char> = phf_map! {
    'd' => 'đ',
    'D' => 'Đ',
};

/// A map of characters without accent to character with horn accent
pub static HORN_MAP: Map<char, char> = phf_map! {
    'u' => 'ư',
    'o' => 'ơ',
    'ú' => 'ứ',
    'ó' => 'ớ',
    'ù' => 'ừ',
    'ò' => 'ờ',
    'ủ' => 'ử',
    'ỏ' => 'ở',
    'ũ' => 'ữ',
    'õ' => 'ỡ',
    'ọ' => 'ợ',
    'ụ' => 'ự',
    // uppercase
    'U' => 'Ư',
    'O' => 'Ơ',
    'Ú' => 'Ứ',
    'Ó' => 'Ớ',
    'Ù' => 'Ừ',
    'Ò' => 'Ờ',
    'Ủ' => 'Ử',
    'Ỏ' => 'Ở',
    'Ũ' => 'Ữ',
    'Õ' => 'Ỡ',
    'Ọ' => 'Ợ',
    'Ụ' => 'Ự',
};

/// A map of characters without accent to character with breve accent
pub static BREVE_MAP: Map<char, char> = phf_map! {
    'a' => 'ă',
    'á' => 'ắ',
    'à' => 'ằ',
    'ả' => 'ẳ',
    'ã' => 'ẵ',
    'ạ' => 'ặ',
    // uppercase
    'A' => 'Ă',
    'Á' => 'Ắ',
    'À' => 'Ằ',
    'Ả' => 'Ẳ',
    'Ã' => 'Ẵ',
    'Ạ' => 'Ặ',
};
