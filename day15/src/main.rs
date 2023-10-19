#![feature(new_uninit)]

use anyhow::Result;
use nom::{
    character::complete::{self},
    multi::separated_list1,
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

fn part_one(input: &[u32]) -> u32 {
    let mine = MyGenerator::new(input);

    mine.into_iter().take(2020).last().unwrap()
}

fn part_two(input: &[u32]) -> u32 {
    let mine = MyGenerator::new(input);

    mine.into_iter().take(30000000).last().unwrap()
}

#[derive(Debug)]
struct MyGenerator {
    start: Vec<u32>,
    idx: usize,
    last: Option<u32>,
    // map: HashMap<u32, u32>,
    map: Box<[u32]>,
}

impl Iterator for MyGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.get_next_item();

        if let Some(last) = self.last {
            self.map[last as usize] = self.idx as u32;
        }

        self.last = Some(next_item);
        self.idx += 1;

        Some(next_item)
    }
}

impl MyGenerator {
    pub fn new<T: Into<Vec<u32>>>(start: T) -> Self {
        // 30000000
        let map = unsafe { Box::<[u32]>::new_zeroed_slice(30000000).assume_init() };
        Self {
            start: start.into(),
            map,
            idx: 0,
            last: None,
        }
    }

    fn get_next_item(&mut self) -> u32 {
        if self.idx < self.start.len() {
            return self.start[self.idx];
        }

        match self.map[self.last.unwrap() as usize] {
            0 => 0,
            prev => self.idx as u32 - prev,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(complete::char(','), complete::u32)(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u32>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(436, part_one(&parse_input("0,3,6")?));
        assert_eq!(1, part_one(&parse_input("1,3,2")?));
        assert_eq!(10, part_one(&parse_input("2,1,3")?));
        assert_eq!(27, part_one(&parse_input("1,2,3")?));
        assert_eq!(78, part_one(&parse_input("2,3,1")?));
        assert_eq!(438, part_one(&parse_input("3,2,1")?));
        assert_eq!(1836, part_one(&parse_input("3,1,2")?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(1238, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(175594, part_two(&parse_input("0,3,6")?));
        assert_eq!(2578, part_two(&parse_input("1,3,2")?));
        assert_eq!(3544142, part_two(&parse_input("2,1,3")?));
        assert_eq!(261214, part_two(&parse_input("1,2,3")?));
        assert_eq!(6895259, part_two(&parse_input("2,3,1")?));
        assert_eq!(18, part_two(&parse_input("3,2,1")?));
        assert_eq!(362, part_two(&parse_input("3,1,2")?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(3745954, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
