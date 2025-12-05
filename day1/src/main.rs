use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult, Parser,
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

fn part_one(input: &[u32]) -> u32 {
    input
        .iter()
        .permutations(2)
        .find(|x| x[0] + x[1] == 2020)
        .map(|x| x[0] * x[1])
        .unwrap()
}

fn part_two(input: &[u32]) -> u32 {
    input
        .iter()
        .permutations(3)
        .find(|x| x[0] + x[1] + x[2] == 2020)
        .map(|x| x[0] * x[1] * x[2])
        .unwrap()
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, complete::u32).parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u32>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 514579);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 955584);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 241861950);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 287503934);

        Ok(())
    }
}
