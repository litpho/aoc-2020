use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::value,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::iter;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let (departure, lines) = result?;

    let (took, result) = took::took(|| part_one(departure, &lines));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(100000000000000, &lines));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(departure: u64, lines: &[u64]) -> u64 {
    let (line, next) = lines
        .iter()
        .filter(|line| **line != 0)
        .map(|line| (line, next_multiple(departure, *line)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    line * (next - departure)
}

fn next_multiple(target: u64, factor: u64) -> u64 {
    target / factor * factor + factor
}

fn part_two(start_from: u64, lines: &[u64]) -> u64 {
    let (max_idx, max_line) = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| **line != 0)
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    let other_lines = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| **line != 0 && *line != max_line)
        .collect::<Vec<(usize, &u64)>>();

    let start = next_multiple(start_from, *max_line) - *max_line;

    iter::successors(Some(start), |x| Some(x + *max_line))
        .find(|x| {
            // println!("Finding {x}");
            other_lines
                .iter()
                .all(|(idx, y)| (x - max_idx as u64 + *idx as u64) % **y == 0)
        })
        .unwrap()
        - max_idx as u64
}

fn parse(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(parse_departure, line_ending, parse_buses)(input)
}

fn parse_departure(input: &str) -> IResult<&str, u64> {
    complete::u64(input)
}

fn parse_buses(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::char(','), alt((parse_x, parse_busline_number)))(input)
}

fn parse_x(input: &str) -> IResult<&str, u64> {
    value(0, tag("x"))(input)
}

fn parse_busline_number(input: &str) -> IResult<&str, u64> {
    complete::u64(input)
}

fn parse_input(input: &'static str) -> Result<(u64, Vec<u64>)> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let (departure, lines) = parse_input(TESTDATA)?;
        assert_eq!(295, part_one(departure, &lines));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (departure, lines) = parse_input(DATA)?;
        assert_eq!(3865, part_one(departure, &lines));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (_, lines) = parse_input(TESTDATA)?;
        assert_eq!(1068781, part_two(1, &lines));

        Ok(())
    }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     let (_, lines) = parse_input(DATA)?;
    //     assert_eq!(415579909629976, part_two(100000000000000, &lines));
    //
    //     Ok(())
    // }
}
