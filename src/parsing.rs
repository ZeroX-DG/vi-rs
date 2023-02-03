use nom::{bytes::complete::{take_till, take_while, tag_no_case}, IResult, sequence::tuple, branch::alt};

use crate::util::is_vowel;

pub struct WordComponents<'a> {
    pub initial_consonant: &'a str,
    pub vowel: &'a str,
    pub final_consonant: &'a str
}

fn consonant(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("gi"), tag_no_case("qu"), take_till(is_vowel)))(input)
}

fn vowel(input: &str) -> IResult<&str, &str> {
    take_while(is_vowel)(input)
}

pub fn parse_vowel(input: &str) -> IResult<&str, &str> {
    let (rest, (_, vowel)) = tuple((consonant, vowel))(input)?;
    Ok((rest, vowel))
}

pub fn parse_word(input: &str) -> IResult<&str, WordComponents<'_>> {
    let (rest, (initial_consonant, vowel, final_consonant)) = tuple((consonant, vowel, consonant))(input)?;
    Ok((rest, WordComponents { initial_consonant, vowel, final_consonant }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_vowel_normal() {
        let result = parse_vowel("viet");
        let expected = Ok(("t", "ie"));
        assert_eq!(result, expected);
    }

    #[test]
    fn get_vowel_empty() {
        let result = parse_vowel("vt");
        let expected = Ok(("", ""));
        assert_eq!(result, expected);
    }

    #[test]
    fn get_vowel_double_start_tone() {
        let result = parse_vowel("quai");
        let expected = Ok(("", "ai"));
        assert_eq!(result, expected);
    }

    #[test]
    fn get_vowel_double_start_tone_2() {
        let result = parse_vowel("gia");
        let expected = Ok(("", "a"));
        assert_eq!(result, expected);
    }
}