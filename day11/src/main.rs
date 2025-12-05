use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::iter;

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

fn part_one(input: &[Vec<Loc>]) -> usize {
    calculate(input, 4, true)
}

fn part_two(input: &[Vec<Loc>]) -> usize {
    calculate(input, 5, false)
}

fn calculate(input: &[Vec<Loc>], empty_limit: usize, limit_to_one: bool) -> usize {
    let mut last_input = input.to_vec();
    loop {
        let next_input = next_round(&last_input, empty_limit, limit_to_one);
        if next_input.eq(&last_input) {
            break;
        }
        last_input = next_input;
    }

    last_input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|l| l == &&Loc::Occupied)
        .count()
}

fn next_round(input: &[Vec<Loc>], empty_limit: usize, limit_to_one: bool) -> Vec<Vec<Loc>> {
    let limits = Limits::new(input[0].len(), input.len(), empty_limit, limit_to_one);
    input
        .iter()
        .enumerate()
        .map(|(y, vec)| {
            vec.iter()
                .enumerate()
                .map(|(x, loc)| match loc {
                    Loc::Floor => Loc::Floor,
                    _ => limits.determine_loc(input, x, y, loc),
                })
                .collect::<Vec<Loc>>()
        })
        .collect::<Vec<Vec<Loc>>>()
}

struct Limits {
    max_x: usize,
    max_y: usize,
    empty_limit: usize,
    limit_to_one: bool,
}

impl Limits {
    pub fn new(max_x: usize, max_y: usize, empty_limit: usize, limit_to_one: bool) -> Self {
        Self {
            max_x,
            max_y,
            empty_limit,
            limit_to_one,
        }
    }

    pub fn determine_loc(&self, input: &[Vec<Loc>], x: usize, y: usize, loc: &Loc) -> Loc {
        let mut count = 0;
        // topleft
        if self.loc_line(input, (0..x).rev(), (0..y).rev()) {
            count += 1;
        }
        // top
        if self.loc_line(input, iter::repeat(x), (0..y).rev()) {
            count += 1;
        }
        // topright
        if self.loc_line(input, (x + 1)..self.max_x, (0..y).rev()) {
            count += 1;
        }
        // left
        if self.loc_line(input, (0..x).rev(), iter::repeat(y)) {
            count += 1;
        }
        // right
        if self.loc_line(input, (x + 1)..self.max_x, iter::repeat(y)) {
            count += 1;
        }
        if self.loc_line(input, (0..x).rev(), (y + 1)..self.max_y) {
            count += 1;
        }
        if self.loc_line(input, iter::repeat(x), (y + 1)..self.max_y) {
            count += 1;
        }
        if self.loc_line(input, (x + 1)..self.max_x, (y + 1)..self.max_y) {
            count += 1;
        }

        match count {
            0 => Loc::Occupied,
            count if count >= self.empty_limit => Loc::Empty,
            _ => loc.to_owned(),
        }
    }

    fn loc_line(
        &self,
        input: &[Vec<Loc>],
        x_range: impl Iterator<Item = usize>,
        y_range: impl Iterator<Item = usize>,
    ) -> bool {
        if self.limit_to_one {
            x_range
                .zip(y_range)
                .take(1)
                .find_map(|(x, y)| Self::find_occupied(input, x, y))
                .unwrap_or(false)
        } else {
            x_range
                .zip(y_range)
                .find_map(|(x, y)| Self::find_occupied(input, x, y))
                .unwrap_or(false)
        }
    }

    fn find_occupied(input: &[Vec<Loc>], x: usize, y: usize) -> Option<bool> {
        match input[y][x] {
            Loc::Floor => None,
            Loc::Empty => Some(false),
            Loc::Occupied => Some(true),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Loc {
    Floor,
    Empty,
    Occupied,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Loc>>> {
    separated_list1(line_ending, parse_row).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Loc>> {
    many1(parse_loc).parse(input)
}

fn parse_loc(input: &str) -> IResult<&str, Loc> {
    alt((parse_floor, parse_empty, parse_occupied)).parse(input)
}

fn parse_floor(input: &str) -> IResult<&str, Loc> {
    value(Loc::Floor, complete::char('.')).parse(input)
}

fn parse_empty(input: &str) -> IResult<&str, Loc> {
    value(Loc::Empty, complete::char('L')).parse(input)
}

fn parse_occupied(input: &str) -> IResult<&str, Loc> {
    value(Loc::Occupied, complete::char('#')).parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<Loc>>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 37);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 2277);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 26);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 2066);

        Ok(())
    }
}
