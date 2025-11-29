use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult, Parser,
};
use std::{cmp::min, collections::HashMap};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[u8]) -> u32 {
    let (one, three) = input
        .iter()
        .merge(vec![&0])
        .sorted()
        .tuple_windows::<(_, _)>()
        .fold((0, 0), |(one, three), (lo, hi)| {
            if hi - lo == 1 {
                (one + 1, three)
            } else {
                (one, three + 1)
            }
        });

    one * (three + 1)
}

fn part_two(mut input: Vec<u8>) -> u64 {
    input.push(0);
    input.sort();
    let max = input.last().unwrap() + 3;
    input.push(max);

    let map = create_map(&input);

    calculate(&input, &map)
}

fn create_map(input: &[u8]) -> HashMap<u8, Vec<u8>> {
    input
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            let v = input[idx + 1..min(idx + 4, input.len())]
                .iter()
                .filter(|y| *y - x <= 3)
                .copied()
                .collect::<Vec<u8>>();
            (*x, v)
        })
        .collect::<HashMap<u8, Vec<u8>>>()
}

fn calculate(input: &[u8], map: &HashMap<u8, Vec<u8>>) -> u64 {
    let mut new_map: HashMap<u8, u64> = HashMap::new();
    for node in input.iter().rev() {
        let num = map[node].iter().map(|x| new_map[x]).sum1().unwrap_or(1);
        new_map.insert(*node, num);
    }

    new_map[&0]
}

fn parse(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(line_ending, complete::u8).parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u8>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(35, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one_testdata_2() -> Result<()> {
        assert_eq!(220, part_one(&parse_input(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(2046, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(8, part_two(parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata_2() -> Result<()> {
        assert_eq!(19208, part_two(parse_input(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(1157018619904, part_two(parse_input(DATA)?));

        Ok(())
    }
}
