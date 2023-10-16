use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::one_of,
        complete::{self, line_ending},
    },
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
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

fn part_one(input: &[Instruction]) -> u64 {
    let mut mem: HashMap<u32, u64> = HashMap::new();
    let mut mask: Mask = Mask::new(&[]);
    input.iter().for_each(|instruction| match instruction {
        Instruction::Mask(m) => {
            mask = m.clone();
        }
        Instruction::Mem { idx, value } => {
            let val = (*value | mask.or) & mask.and;
            mem.insert(*idx, val);
        }
    });

    mem.values().sum()
}

fn part_two(_input: &[Instruction]) -> u64 {
    todo!()
}

#[derive(Clone, Debug)]
struct Mask {
    and: u64,
    or: u64,
}

impl Mask {
    pub fn new(v: &[char]) -> Self {
        let (or, and) = v.iter().rev().enumerate().filter(|(_, c)| **c != 'X').fold(
            (0, 0),
            |(or, and), (idx, c)| {
                let add = 1 << idx;
                if *c == '0' {
                    (or, and + add)
                } else {
                    (or + add, and)
                }
            },
        );

        Self { or, and: !and }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem { idx: u32, value: u64 },
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mask, parse_mem))(input)
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mask = "), parse_mask_value), |s| {
        Instruction::Mask(Mask::new(&s))
    })(input)
}

fn parse_mask_value(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("X01"))(input)
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((tag("mem["), complete::u32, tag("] = "), complete::u64)),
        |(_, idx, _, value)| Instruction::Mem { idx, value },
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(165, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(9967721333886, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     let (_, lines) = parse_input(TESTDATA)?;
    //     assert_eq!(1068781, part_two(&lines));
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part_two() -> Result<()> {
    //     let (_, lines) = parse_input(DATA)?;
    //     assert_eq!(415579909629976, part_two(&lines));
    //
    //     Ok(())
    // }
}
