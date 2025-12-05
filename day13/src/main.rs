use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::value,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let (departure, lines) = result?;

    let (took, result) = took::took(|| part_one(departure, &lines));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&lines));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(departure: u64, lines: &[u64]) -> u64 {
    let (line, next) = lines
        .iter()
        .filter(|line| **line != 0)
        .map(|line| (line, departure.next_multiple_of(*line)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    line * (next - departure)
}

fn part_two(bus_ids: &[u64]) -> u64 {
    let (current_solution, _) = bus_ids
        .iter()
        .enumerate()
        .filter(|(_, bus_id)| **bus_id != 0)
        .fold(
            (0u64, 1u64),
            |(current_solution, step_size), (offset, bus_id)| {
                (current_solution..u64::MAX)
                    .step_by(step_size as usize)
                    .find_map(|timestamp| {
                        if (timestamp + offset as u64).is_multiple_of(*bus_id) {
                            Some((timestamp, step_size * bus_id))
                        } else {
                            None
                        }
                    })
                    .unwrap()
            },
        );

    current_solution
}

fn parse(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(parse_departure, line_ending, parse_buses).parse(input)
}

fn parse_departure(input: &str) -> IResult<&str, u64> {
    complete::u64(input)
}

fn parse_buses(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::char(','), alt((parse_x, parse_busline_number))).parse(input)
}

fn parse_x(input: &str) -> IResult<&str, u64> {
    value(0, tag("x")).parse(input)
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
        assert_eq!(part_one(departure, &lines), 295);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (departure, lines) = parse_input(DATA)?;
        assert_eq!(part_one(departure, &lines), 3865);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (_, lines) = parse_input(TESTDATA)?;
        assert_eq!(part_two(&lines), 1068781);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (_, lines) = parse_input(DATA)?;
        assert_eq!(part_two(&lines), 415579909629976);

        Ok(())
    }
}
