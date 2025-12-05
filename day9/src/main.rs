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

    let (took, result) = took::took(|| part_one(&input, 25));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input, 25));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[u32], preamble: usize) -> u32 {
    let (_, result) = find_number(input, preamble);

    *result
}

fn part_two(input: &[u32], preamble: usize) -> u32 {
    let (result_idx, result) = find_number(input, preamble);
    (0..result_idx)
        .rev()
        .find_map(|top_idx| {
            let (bottom_idx, sum) = find_idx_and_sum(input, top_idx, *result);
            if sum == *result {
                Some((bottom_idx, top_idx))
            } else {
                None
            }
        })
        .map(|(bottom_idx, top_idx)| {
            let (min, max) = input[bottom_idx..top_idx]
                .iter()
                .minmax()
                .into_option()
                .unwrap();
            *min + *max
        })
        .unwrap()
}

fn find_idx_and_sum(input: &[u32], top_idx: usize, result: u32) -> (usize, u32) {
    (0..top_idx)
        .rev()
        .try_fold((0, 0), |(_, sum), idx| {
            let sum = sum + input[idx];
            if sum >= result {
                Err((idx, sum))
            } else {
                Ok((idx, sum))
            }
        })
        .unwrap_err()
}

fn find_number(input: &[u32], preamble: usize) -> (usize, &u32) {
    input
        .iter()
        .skip(preamble)
        .enumerate()
        .find(|(idx, x)| {
            !input[*idx..(idx + preamble)]
                .iter()
                .permutations(2)
                .any(|v| **x == v[0] + v[1])
        })
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
        assert_eq!(part_one(&parse_input(TESTDATA)?, 5), 127);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?, 25), 530627549);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?, 5), 62);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?, 25), 77730285);

        Ok(())
    }
}
