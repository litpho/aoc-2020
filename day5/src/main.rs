use anyhow::Result;
use bit_vec::BitVec;
use nom::{
    bytes::complete::take, character::complete::line_ending, combinator::map,
    multi::separated_list1, IResult,
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

fn part_one(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

fn part_two(input: &[u16]) -> u16 {
    let mut bv: BitVec = BitVec::from_elem(1024, false);
    for seat_id in input {
        bv.set(*seat_id as usize, true);
    }

    bv.iter()
        .enumerate()
        .skip_while(|(_, b)| b == &false)
        .find(|(_, b)| b == &false)
        .map(|(idx, _)| idx)
        .unwrap() as u16
}

fn parse(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, u16> {
    map(take(10usize), |x: &str| {
        let bin_string = x
            .chars()
            .map(|c| if c == 'L' || c == 'F' { '0' } else { '1' })
            .collect::<String>();
        u16::from_str_radix(&bin_string, 2).unwrap()
    })(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u16>> {
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
    fn test_part_two() -> Result<()> {
        assert_eq!(642, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
