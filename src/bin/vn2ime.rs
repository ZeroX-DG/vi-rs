use std::collections::HashMap;
use std::env;
use std::io;
use std::process;

/// Convert Vietnamese words into its ime presentation, supports Telex/VNI.
/// Main function to run the converter as a command-line tool.
/// It parses the command-line arguments, selects the appropriate conversion map,
/// and processes input from stdin line by line.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for the correct number of command-line arguments.
    if args.len() != 2 {
        eprintln!("Usage: {} <type>", args[0]);
        eprintln!("  type: 'vni' or 'telex'");
        process::exit(1);
    }

    // Determine which conversion map to use based on the argument.
    // The program exits if the argument is invalid.
    let conversion_map = match args[1].as_str() {
        "telex" => generate_telex_map(),
        "vni" => generate_vni_map(),
        _ => {
            eprintln!(
                "Error: Invalid type '{}'. Please use 'vni' or 'telex'.",
                args[1]
            );
            process::exit(1);
        }
    };

    // Read from standard input line by line until EOF.
    for line in io::stdin().lines() {
        match line {
            Ok(syllable) => {
                let trimmed = syllable.trim();
                if !trimmed.is_empty() {
                    let result = convert_syllable(trimmed, &conversion_map);
                    println!("{result}");
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {e}");
                process::exit(1);
            }
        }
    }
}

/// Converts an input string into a sequence of keystrokes using a provided mapping table.
/// Non-Vietnamese characters are passed through unchanged.
fn convert_syllable(syllable: &str, map: &HashMap<char, String>) -> String {
    let mut result = String::new();
    for c in syllable.chars() {
        if let Some(keystrokes) = map.get(&c) {
            result.push_str(keystrokes);
        } else {
            // If the character is not in the map (e.g., a consonant, punctuation, space), append it directly.
            result.push(c);
        }
    }
    result
}

// --- Map Generation Functions (Unchanged from previous version) ---

/// Generates the complete character-to-keystroke map for the Telex input method.
fn generate_telex_map() -> HashMap<char, String> {
    let mut map = HashMap::new();
    let base_map: HashMap<char, &str> = [
        ('ă', "aw"),
        ('â', "aa"),
        ('đ', "dd"),
        ('ê', "ee"),
        ('ô', "oo"),
        ('ơ', "ow"),
        ('ư', "uw"),
    ]
    .iter()
    .cloned()
    .collect();

    let tone_keys = ["", "f", "s", "r", "x", "j"]; // Ngang, Huyền, Sắc, Hỏi, Ngã, Nặng
    let toned_vowels = get_toned_vowels();

    for vowel_set in toned_vowels.iter() {
        let base_char = vowel_set[0];
        let base_stroke = base_map.get(&base_char).unwrap_or(&"").to_string();

        let final_base_stroke = if base_stroke.is_empty() {
            base_char.to_string()
        } else {
            base_stroke
        };

        for (tone_index, &toned_char) in vowel_set.iter().enumerate() {
            let tone_stroke = tone_keys[tone_index];
            map.insert(toned_char, format!("{final_base_stroke}{tone_stroke}"));
        }
    }

    map.insert('đ', "dd".to_string());
    map
}

/// Generates the complete character-to-keystroke map for the VNI input method.
fn generate_vni_map() -> HashMap<char, String> {
    let mut map = HashMap::new();
    let base_map: HashMap<char, &str> = [
        ('ă', "a8"),
        ('â', "a6"),
        ('đ', "d9"),
        ('ê', "e6"),
        ('ô', "o6"),
        ('ơ', "o7"),
        ('ư', "u7"),
    ]
    .iter()
    .cloned()
    .collect();

    let tone_keys = ["", "2", "1", "3", "4", "5"]; // Ngang, Huyền, Sắc, Hỏi, Ngã, Nặng
    let toned_vowels = get_toned_vowels();

    for vowel_set in toned_vowels.iter() {
        let base_char = vowel_set[0];
        let base_stroke = base_map.get(&base_char).unwrap_or(&"").to_string();

        let final_base_stroke = if base_stroke.is_empty() {
            base_char.to_string()
        } else {
            base_stroke
        };

        for (tone_index, &toned_char) in vowel_set.iter().enumerate() {
            let tone_stroke = tone_keys[tone_index];
            map.insert(toned_char, format!("{final_base_stroke}{tone_stroke}"));
        }
    }

    map.insert('đ', "d9".to_string());
    map
}

/// Returns a matrix of all Vietnamese vowels, organized by tone.
fn get_toned_vowels() -> Vec<[char; 6]> {
    vec![
        ['a', 'à', 'á', 'ả', 'ã', 'ạ'],
        ['ă', 'ằ', 'ắ', 'ẳ', 'ẵ', 'ặ'],
        ['â', 'ầ', 'ấ', 'ẩ', 'ẫ', 'ậ'],
        ['e', 'è', 'é', 'ẻ', 'ẽ', 'ẹ'],
        ['ê', 'ề', 'ế', 'ể', 'ễ', 'ệ'],
        ['i', 'ì', 'í', 'ỉ', 'ĩ', 'ị'],
        ['o', 'ò', 'ó', 'ỏ', 'õ', 'ọ'],
        ['ô', 'ồ', 'ố', 'ổ', 'ỗ', 'ộ'],
        ['ơ', 'ờ', 'ớ', 'ở', 'ỡ', 'ợ'],
        ['u', 'ù', 'ú', 'ủ', 'ũ', 'ụ'],
        ['ư', 'ừ', 'ứ', 'ử', 'ữ', 'ự'],
        ['y', 'ỳ', 'ý', 'ỷ', 'ỹ', 'ỵ'],
    ]
}
