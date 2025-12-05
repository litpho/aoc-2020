use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
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

fn part_one(input: &[Vec<Vec<char>>]) -> usize {
    input
        .iter()
        .map(|x| x.iter().flatten().unique().count())
        .sum()
}

fn part_two(input: &[Vec<Vec<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            count_letters_per_groupline(group)
                .iter()
                .filter(|c| **c == group.len())
                .count()
        })
        .sum()
}

fn count_letters_per_groupline(group: &[Vec<char>]) -> [usize; 26] {
    let mut arr: [usize; 26] = [0; 26];
    group.iter().flatten().for_each(|c| {
        arr[((*c as u8) - b'a') as usize] += 1;
    });

    arr
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<char>>>> {
    separated_list1(pair(line_ending, line_ending), parse_group).parse(input)
}

fn parse_group(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(separated_list1(line_ending, alpha1), |v| {
        v.iter()
            .map(|x: &&str| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    })
    .parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<Vec<char>>>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 11);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 6775);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 6);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 3356);

        Ok(())
    }
}
