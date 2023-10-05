use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::take, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::pair, IResult,
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

fn part_one(input: &[Seat]) -> u16 {
    input.iter().max().unwrap().seat_id
}

fn part_two(input: &[Seat]) -> u16 {
    let mut prev = 79u16;
    input
        .iter()
        .map(|x| x.seat_id)
        .sorted()
        .find(|s| {
            let found = prev + 1 != *s;
            prev = *s;
            found
        })
        .unwrap()
        - 1
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Seat {
    row: u8,
    column: u8,
    seat_id: u16,
}

impl Seat {
    pub fn new(row: u8, column: u8) -> Self {
        let seat_id = (8u16 * row as u16) + (column as u16);
        Seat {
            row,
            column,
            seat_id,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Seat>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Seat> {
    map(pair(parse_row, parse_column), |(row, column)| {
        Seat::new(row, column)
    })(input)
}

fn parse_row(input: &str) -> IResult<&str, u8> {
    map(take(7usize), |x: &str| {
        let bin_string = x
            .chars()
            .map(|c| if c == 'F' { '0' } else { '1' })
            .collect::<String>();
        u8::from_str_radix(&bin_string, 2).unwrap()
    })(input)
}

fn parse_column(input: &str) -> IResult<&str, u8> {
    map(take(3usize), |x: &str| {
        let bin_string = x
            .chars()
            .map(|c| if c == 'L' { '0' } else { '1' })
            .collect::<String>();
        u8::from_str_radix(&bin_string, 2).unwrap()
    })(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Seat>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(820, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(919, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(118, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(642, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
