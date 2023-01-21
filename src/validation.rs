//! Validation functions for verifying if a word is a valid vietnamese word.
//! 
//! # The structure of a vietnamese word
//! 
//! 1 optional consonant / consonant cluster + 1 compulsory vowel / vowel cluster + 1 optional consonant / consonant cluster
//! 
//! The starting consonant are called initial consonant, while the consonant at the end is called the final consonant.
//! A cluster of consonant can contains 1 -> 3 characters.
//! See: https://en.wikibooks.org/wiki/Vietnamese/Consonants 

use crate::util::extract_consonants;

const SINGLE_INITIAL_CONSONANTS: [char; 17] = [
    'b', 'c', 'd', 'đ', 'g', 'h', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x'
];

const DIGRAPHS_INITIAL_CONSONANTS: [&str; 10] = [
    "ch", "gh", "gi", "kh", "nh", "ng", "ph", "th", "tr", "qu"
];

const FINAL_CONSONANTS: [&str; 8] = [
    "c", "ch", "m", "n", "nh", "ng", "p", "t"
];

/// Verify if a word is a valid vietnamese word.
pub fn is_valid_vietnamese_word(input: &str) -> bool {
    let (initial_consonant, final_consonant) = extract_consonants(input);
    println!("{:?} {:?}", initial_consonant, final_consonant);

    match (initial_consonant, final_consonant) {
        (Some(initial_c), Some(final_c)) => is_valid_initial_consonant(&initial_c) && is_valid_final_consonant(&final_c),
        (Some(initial_c), None) => is_valid_initial_consonant(&initial_c),
        (None, Some(final_c)) => is_valid_final_consonant(&final_c),
        (None, None) => true,
    }
}

pub fn is_valid_initial_consonant(consonant: &str) -> bool {
    let consonant = consonant.to_lowercase().to_string();
    if consonant.len() == 1 {
        if let Some(c) = consonant.chars().next() {
            return SINGLE_INITIAL_CONSONANTS.contains(&c);
        }
    }

    if consonant.len() == 2 {
        return DIGRAPHS_INITIAL_CONSONANTS.contains(&consonant.as_str());
    }

    if consonant.len() == 3 {
        return consonant == "ngh";
    }

    false
}

pub fn is_valid_final_consonant(consonant: &str) -> bool {
    let consonant = consonant.to_lowercase().to_string();
    FINAL_CONSONANTS.contains(&consonant.as_str())
}
