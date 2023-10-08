use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::complete::{alpha1, char, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    AsChar, IResult,
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

fn part_one(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_complete()).count()
}

fn part_two(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[derive(Hash, Eq, PartialEq)]
enum Key {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "byr" => Key::Byr,
            "iyr" => Key::Iyr,
            "eyr" => Key::Eyr,
            "hgt" => Key::Hgt,
            "hcl" => Key::Hcl,
            "ecl" => Key::Ecl,
            "pid" => Key::Pid,
            "cid" => Key::Cid,
            _ => panic!("Value not found"),
        }
    }
}

struct Passport {
    data: HashMap<Key, String>,
}

impl Passport {
    pub fn new(pairs: Vec<(&str, &str)>) -> Self {
        let data = pairs
            .iter()
            .map(|(key, value)| {
                let key: Key = (*key).into();
                (key, String::from(*value))
            })
            .collect::<HashMap<Key, String>>();

        Passport { data }
    }

    pub fn is_complete(&self) -> bool {
        [
            Key::Byr,
            Key::Iyr,
            Key::Eyr,
            Key::Hgt,
            Key::Hcl,
            Key::Ecl,
            Key::Pid,
        ]
        .iter()
        .all(|key| self.data.contains_key(key))
    }

    pub fn is_valid(&self) -> bool {
        self.is_complete()
            && self.is_valid_byr()
            && self.is_valid_iyr()
            && self.is_valid_eyr()
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
    }

    pub fn is_valid_byr(&self) -> bool {
        self.data[&Key::Byr]
            .parse::<u16>()
            .map_or(false, |byr| (1920u16..=2002u16).contains(&byr))
    }

    pub fn is_valid_iyr(&self) -> bool {
        self.data[&Key::Iyr]
            .parse::<u16>()
            .map_or(false, |iyr| (2010u16..=2020u16).contains(&iyr))
    }

    pub fn is_valid_eyr(&self) -> bool {
        self.data[&Key::Eyr]
            .parse::<u16>()
            .map_or(false, |eyr| (2020u16..=2030u16).contains(&eyr))
    }

    pub fn is_valid_hgt(&self) -> bool {
        let hgt = self.data[&Key::Hgt].as_str();
        let (height, unit) = hgt.split_at(hgt.len() - 2);

        height.parse::<u16>().map_or(false, |h| {
            if unit == "cm" {
                (150u16..=193u16).contains(&h)
            } else if unit == "in" {
                (59u16..=76u16).contains(&h)
            } else {
                false
            }
        })
    }

    pub fn is_valid_hcl(&self) -> bool {
        self.data[&Key::Hcl]
            .strip_prefix('#')
            .map_or(false, |hcl| u32::from_str_radix(hcl, 16).is_ok())
    }

    pub fn is_valid_ecl(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.data[&Key::Ecl].as_str())
    }

    pub fn is_valid_pid(&self) -> bool {
        let pid = self.data[&Key::Pid].as_str();
        pid.len() == 9 && pid.chars().all(|c| c.is_dec_digit())
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(pair(line_ending, line_ending), parse_passport)(input)
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    map(
        separated_list1(alt((space1, line_ending)), parse_key_value),
        Passport::new,
    )(input)
}

fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, char(':'), parse_value)(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '#')(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Passport>> {
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
        assert_eq!(2, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(235, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(4, part_two(&parse_input(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(194, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
