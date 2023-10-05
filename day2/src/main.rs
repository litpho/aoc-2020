use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, anychar, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|line| {
            (line.first_number..=line.second_number)
                .contains(&(line.password.iter().filter(|x| **x == line.letter).count()))
        })
        .count()
}

fn part_two(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|line| {
            let letter = line.letter;
            let first = line.password[line.first_number - 1];
            let second = line.password[line.second_number - 1];
            (first == letter && second != letter) || (first != letter && second == letter)
        })
        .count()
}

#[derive(Debug)]
struct Line {
    first_number: usize,
    second_number: usize,
    letter: char,
    password: Vec<char>,
}

impl Line {
    pub fn new(
        first_number: usize,
        second_number: usize,
        letter: char,
        password: Vec<char>,
    ) -> Self {
        Self {
            first_number,
            second_number,
            letter,
            password,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((parse_numbers, parse_letter, parse_password)),
        |(numbers, letter, password)| Line::new(numbers.0, numbers.1, letter, password),
    )(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        separated_pair(complete::u8, complete::char('-'), complete::u8),
        |(a, b)| (a as usize, b as usize),
    )(input)
}

fn parse_letter(input: &str) -> IResult<&str, char> {
    delimited(space1, anychar, tag(": "))(input)
}

fn parse_password(input: &str) -> IResult<&str, Vec<char>> {
    map(alpha1, |a: &str| a.chars().collect())(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Line>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(2, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(580, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(1, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(611, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
