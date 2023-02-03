use nom::{bytes::complete::{take_till, take_while}, IResult, sequence::tuple};

use crate::util::is_vowel;

pub struct WordComponents<'a> {
    pub initial_consonant: &'a str,
    pub vowel: &'a str,
    pub final_consonant: &'a str
}

fn parse_consonant(input: &str) -> IResult<&str, &str> {
    take_till(is_vowel)(input)
}

fn parse_vowel(input: &str) -> IResult<&str, &str> {
    take_while(is_vowel)(input)
}

pub fn parse_word(input: &str) -> IResult<&str, WordComponents<'_>> {
    let (rest, (initial_consonant, vowel, final_consonant)) = tuple((parse_consonant, parse_vowel, parse_consonant))(input)?;
    Ok((rest, WordComponents { initial_consonant, vowel, final_consonant }))
}