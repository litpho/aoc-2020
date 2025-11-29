use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};
use std::collections::HashMap;

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
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Mask = Mask::new("");
    input.iter().for_each(|instruction| match instruction {
        Instruction::Mask(m) => {
            mask = m.clone();
        }
        Instruction::Mem { idx, value } => {
            let val = (*value | mask.ones) & mask.zeroes;
            mem.insert(*idx, val);
        }
    });

    mem.values().sum()
}

fn part_two(input: &[Instruction]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Mask = Mask::default();
    input.iter().for_each(|instruction| match instruction {
        Instruction::Mask(m) => {
            mask = m.clone();
        }
        Instruction::Mem { idx, value } => mask.calc_addresses(*idx).iter().for_each(|address| {
            mem.insert(*address, *value);
        }),
    });

    mem.values().sum()
}

#[derive(Clone, Debug, Default)]
struct Mask {
    raw: String,
    zeroes: u64,
    ones: u64,
}

impl Mask {
    pub fn new<T: Into<String>>(v: T) -> Self {
        let raw = v.into();
        let (zeroes, ones) = raw
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c != 'X')
            .fold((0, 0), |(zeroes, ones), (idx, c)| {
                let add = 1 << idx;
                if c == '0' {
                    (zeroes + add, ones)
                } else {
                    (zeroes, ones + add)
                }
            });

        Self {
            raw,
            ones,
            zeroes: !zeroes,
        }
    }

    pub fn calc_addresses(&self, value: u64) -> Vec<u64> {
        let val = value | self.ones;
        self.raw
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c == 'X')
            .fold(vec![val], |v, (idx, _)| {
                v.iter()
                    .flat_map(|x| {
                        let pos = 1 << idx;
                        vec![x & !pos, x | pos]
                    })
                    .collect::<Vec<u64>>()
            })
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem { idx: u64, value: u64 },
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mask, parse_mem)).parse(input)
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mask = "), parse_mask_value), |s| {
        Instruction::Mask(Mask::new(s))
    })
    .parse(input)
}

fn parse_mask_value(input: &str) -> IResult<&str, &str> {
    is_a("X01")(input)
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    map(
        (tag("mem["), complete::u64, tag("] = "), complete::u64),
        |(_, idx, _, value)| Instruction::Mem { idx, value },
    )
    .parse(input)
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

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(4355897790573, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
