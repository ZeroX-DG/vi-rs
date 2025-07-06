use std::collections::{HashMap, HashSet};

/// Generates all valid Vietnamese syllables based on phonological and orthographic rules.
/// The program combines initials, rhymes, and tones according to a strict set of rules,
/// then prints each unique, valid syllable to standard output.
fn main() {
    let initials = get_initials();
    let rhymes = get_rhymes();
    let tone_map = get_tone_map();

    // Use a HashSet to automatically handle duplicates.
    let mut generated_syllables = HashSet::new();

    for &initial in initials.iter() {
        for &rhyme in rhymes.iter() {
            // Rule 1: Check if the initial and rhyme can be legally combined.
            if !is_valid_combination(initial, rhyme) {
                continue;
            }

            let base_syllable = format!("{initial}{rhyme}");

            // Rule 2: Apply tones based on whether the rhyme is "checked" or not.
            let is_checked_rhyme = rhyme.ends_with('c')
                || rhyme.ends_with("ch")
                || rhyme.ends_with('p')
                || rhyme.ends_with('t');

            if is_checked_rhyme {
                // Checked rhymes only allow Sắc (index 2) and Nặng (index 5) tones.
                for tone_index in [2, 5] {
                    let final_syllable = apply_tone(&base_syllable, rhyme, tone_index, &tone_map);
                    generated_syllables.insert(final_syllable);
                }
            } else {
                // Open and nasal rhymes can take all 6 tones.
                for tone_index in 0..6 {
                    let final_syllable = apply_tone(&base_syllable, rhyme, tone_index, &tone_map);
                    generated_syllables.insert(final_syllable);
                }
            }
        }
    }

    // Sort the results for consistent, readable output.
    let mut sorted_syllables: Vec<String> = generated_syllables.into_iter().collect();
    sorted_syllables.sort();

    // Print each syllable to stdout.
    for syllable in &sorted_syllables {
        println!("{syllable}");
    }

    // Print a summary to stderr.
    eprintln!(
        "\nSuccessfully generated {} unique Vietnamese syllables.",
        sorted_syllables.len()
    );
}

/// Checks if an initial consonant can legally precede a rhyme.
/// This enforces fundamental Vietnamese orthographic rules.
fn is_valid_combination(initial: &str, rhyme: &str) -> bool {
    let first_char_of_rhyme = rhyme.chars().next().unwrap_or(' ');
    let is_front_vowel_rhyme = "ieêy".contains(first_char_of_rhyme);

    // Rule: k/g/ng vs c/gh/ngh
    match initial {
        "c" | "g" | "ng" => {
            if is_front_vowel_rhyme {
                return false;
            } // c, g, ng must be followed by other vowels.
        }
        "k" | "gh" | "ngh" => {
            if !is_front_vowel_rhyme {
                return false;
            } // k, gh, ngh must be followed by i, e, ê, y.
        }
        "qu" => {
            // 'qu' acts as a single unit and cannot be followed by a rhyme starting with 'u' or 'o'.
            if "uo".contains(first_char_of_rhyme) {
                return false;
            }
        }
        "gi" => {
            // Avoid "gi" + i-initial rhyme like "giiên", which is not a valid word.
            if first_char_of_rhyme == 'i' {
                return false;
            }
        }
        _ => {}
    }

    true
}

/// Applies a tone mark to the correct vowel in a syllable.
/// The logic follows a priority system for placing the diacritic.
fn apply_tone(
    base_syllable: &str,
    rhyme: &str,
    tone_index: usize,
    tone_map: &HashMap<char, [&'static str; 6]>,
) -> String {
    // Ngang tone (index 0) has no diacritic.
    if tone_index == 0 {
        return base_syllable.to_string();
    }

    let mut tone_char: Option<char> = None;

    // Rule 1: Handle special diphthong exceptions first.
    // For 'ua', 'ưa', 'ia', the tone goes on the first vowel.
    if rhyme == "ua" || rhyme == "ưa" || rhyme == "ia" {
        tone_char = rhyme.chars().next();
    }
    // For 'uy', the tone goes on the second vowel, 'y'.
    else if rhyme.ends_with("uy") {
        tone_char = Some('y');
    }

    // Rule 2: If no special case was met, apply the general priority rule.
    // The tone goes on the first vowel found from this priority list: a, ă, â, o, ô, ơ, e, ê.
    if tone_char.is_none() {
        for c in ['a', 'ă', 'â', 'o', 'ô', 'ơ', 'e', 'ê'] {
            if rhyme.contains(c) {
                tone_char = Some(c);
                break;
            }
        }
    }

    // Rule 3: Final fallback for rhymes without any priority vowels (e.g., 'iu', 'ưu').
    // The tone goes on the first vowel.
    if tone_char.is_none() {
        tone_char = rhyme.chars().find(|c| "iuưy".contains(*c));
    }

    // Now, find the byte position of the character to be replaced and build the new string.
    if let Some(char_to_tone) = tone_char {
        // Find the character in the full syllable to get the correct byte index.
        if let Some(pos) = base_syllable.find(char_to_tone) {
            if let Some(toned_vowels) = tone_map.get(&char_to_tone) {
                let toned_vowel_str = toned_vowels[tone_index];
                let mut result = String::with_capacity(base_syllable.len() + 3); // Pre-allocate
                result.push_str(&base_syllable[..pos]);
                result.push_str(toned_vowel_str);
                result.push_str(&base_syllable[pos + char_to_tone.len_utf8()..]);
                return result;
            }
        }
    }

    // Fallback: If no rule matched, return the untoned syllable.
    base_syllable.to_string()
}

// --- Data Definitions ---

fn get_initials() -> Vec<&'static str> {
    vec![
        "", "b", "c", "ch", "d", "đ", "g", "gh", "gi", "h", "k", "kh", "l", "m", "n", "ng", "ngh",
        "nh", "p", "ph", "qu", "r", "s", "t", "th", "tr", "v", "x",
    ]
}

fn get_rhymes() -> Vec<&'static str> {
    // This is an exhaustive list of Vietnamese rhymes (vần).
    vec![
        "a", "ac", "ach", "ai", "am", "an", "ang", "anh", "ao", "ap", "at", "au", "ay", "ă", "ăc",
        "ăm", "ăn", "ăng", "ăp", "ăt", "â", "âc", "âm", "ân", "âng", "âp", "ât", "âu", "ây", "e",
        "ec", "em", "en", "eng", "eo", "ep", "et", "ê", "êch", "êm", "ên", "ênh", "êp", "êt", "êu",
        "i", "ia", "ich", "iêc", "iêm", "iên", "iêng", "iêp", "iêt", "iêu", "im", "in", "inh",
        "ip", "it", "iu", "o", "oa", "oac", "oach", "oai", "oam", "oan", "oang", "oanh", "oap",
        "oat", "oay", "oc", "oe", "oem", "oen", "om", "on", "ong", "op", "ot", "oi", "ô", "ôc",
        "ôi", "ôm", "ôn", "ông", "ôp", "ôt", "ơ", "ơi", "ơm", "ơn", "ơp", "ơt", "u", "ua", "uân",
        "uât", "uc", "uê", "uêch", "uênh", "uêt", "uây", "ui", "uich", "um", "un", "ung", "uôc",
        "uôi", "uôm", "uôn", "uông", "uôt", "up", "ut", "uy", "uyn", "uynh", "uyp", "uyt", "uyên",
        "uyêt", "ư", "ưa", "ưc", "ưi", "ưm", "ưn", "ưng", "ươc", "ươi", "ươm", "ươn", "ương",
        "ươp", "ươt", "ưu", "ưp", "ưt", "y", "ya", "ych", "yêm", "yên", "yêng", "yêp", "yêt",
        "yêu", "ynh",
    ]
}

/// Returns a map from a base vowel to its 6 toned forms.
/// Tones are in order: Ngang, Huyền, Sắc, Hỏi, Ngã, Nặng.
fn get_tone_map() -> HashMap<char, [&'static str; 6]> {
    let mut map = HashMap::new();
    map.insert('a', ["a", "à", "á", "ả", "ã", "ạ"]);
    map.insert('ă', ["ă", "ằ", "ắ", "ẳ", "ẵ", "ặ"]);
    map.insert('â', ["â", "ầ", "ấ", "ẩ", "ẫ", "ậ"]);
    map.insert('e', ["e", "è", "é", "ẻ", "ẽ", "ẹ"]);
    map.insert('ê', ["ê", "ề", "ế", "ể", "ễ", "ệ"]);
    map.insert('i', ["i", "ì", "í", "ỉ", "ĩ", "ị"]);
    map.insert('o', ["o", "ò", "ó", "ỏ", "õ", "ọ"]);
    map.insert('ô', ["ô", "ồ", "ố", "ổ", "ỗ", "ộ"]);
    map.insert('ơ', ["ơ", "ờ", "ớ", "ở", "ỡ", "ợ"]);
    map.insert('u', ["u", "ù", "ú", "ủ", "ũ", "ụ"]);
    map.insert('ư', ["ư", "ừ", "ứ", "ử", "ữ", "ự"]);
    map.insert('y', ["y", "ỳ", "ý", "ỷ", "ỹ", "ỵ"]);
    map
}
