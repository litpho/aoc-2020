use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
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

fn part_one(input: &[Vec<Loc>]) -> usize {
    let mut last_input = input.to_vec();
    loop {
        let next_input = next_round(&last_input);
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

fn next_round(input: &[Vec<Loc>]) -> Vec<Vec<Loc>> {
    let mut output = vec![];
    let max_x = input[0].len();
    let max_y = input.len();
    for (y, vec) in input.iter().enumerate() {
        let mut row = vec![];
        for (x, loc) in vec.iter().enumerate() {
            let loc = match loc {
                Loc::Floor => Loc::Floor,
                _ => determine_loc(input, x, y, loc, max_x, max_y),
            };
            row.push(loc);
        }
        output.push(row);
    }

    output
}

fn determine_loc(
    input: &[Vec<Loc>],
    x: usize,
    y: usize,
    loc: &Loc,
    max_x: usize,
    max_y: usize,
) -> Loc {
    let mut count = 0;
    if y > 0 {
        if x > 0 && input[y - 1][x - 1] == Loc::Occupied {
            count += 1;
        }
        if input[y - 1][x] == Loc::Occupied {
            count += 1;
        }
        if x < max_x - 1 && input[y - 1][x + 1] == Loc::Occupied {
            count += 1;
        }
    }
    if x > 0 && input[y][x - 1] == Loc::Occupied {
        count += 1;
    }
    if x < max_x - 1 && input[y][x + 1] == Loc::Occupied {
        count += 1;
    }
    if y < max_y - 1 {
        if x > 0 && input[y + 1][x - 1] == Loc::Occupied {
            count += 1;
        }
        if input[y + 1][x] == Loc::Occupied {
            count += 1;
        }
        if x < max_x - 1 && input[y + 1][x + 1] == Loc::Occupied {
            count += 1;
        }
    }

    if count == 0 {
        return Loc::Occupied;
    }
    if count >= 4 {
        return Loc::Empty;
    }

    loc.clone()
}

fn part_two(input: &[Vec<Loc>]) -> usize {
    let mut last_input = input.to_vec();
    loop {
        let next_input = next_round_line(&last_input);
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

fn next_round_line(input: &[Vec<Loc>]) -> Vec<Vec<Loc>> {
    let mut output = vec![];
    let max_x = input[0].len();
    let max_y = input.len();
    for (y, vec) in input.iter().enumerate() {
        let mut row = vec![];
        for (x, loc) in vec.iter().enumerate() {
            let loc = match loc {
                Loc::Floor => Loc::Floor,
                _ => determine_loc_line(input, x, y, loc, max_x, max_y),
            };
            row.push(loc);
        }
        output.push(row);
    }

    output
}

fn determine_loc_line(
    input: &[Vec<Loc>],
    x: usize,
    y: usize,
    loc: &Loc,
    max_x: usize,
    max_y: usize,
) -> Loc {
    let mut count = 0;
    // topleft
    if loc_line(input, (0..x).rev(), (0..y).rev()) {
        count += 1;
    }
    // top
    if loc_line(input, [x].into_iter().cycle(), (0..y).rev()) {
        count += 1;
    }
    // topright
    if loc_line(input, (x + 1)..max_x, (0..y).rev()) {
        count += 1;
    }
    // left
    if loc_line(input, (0..x).rev(), [y].into_iter().cycle()) {
        count += 1;
    }
    // right
    if loc_line(input, (x + 1)..max_x, [y].into_iter().cycle()) {
        count += 1;
    }
    if loc_line(input, (0..x).rev(), (y + 1)..max_y) {
        count += 1;
    }
    if loc_line(input, [x].into_iter().cycle(), (y + 1)..max_y) {
        count += 1;
    }
    if loc_line(input, (x + 1)..max_x, (y + 1)..max_y) {
        count += 1;
    }

    if count == 0 {
        return Loc::Occupied;
    }
    if count >= 5 {
        return Loc::Empty;
    }

    loc.clone()
}

fn loc_line(
    input: &[Vec<Loc>],
    x_range: impl Iterator<Item=usize>,
    y_range: impl Iterator<Item=usize>,
) -> bool {
    x_range
        .zip(y_range)
        .find_map(|(x, y)| match input[y][x] {
            Loc::Floor => None,
            Loc::Empty => Some(false),
            Loc::Occupied => Some(true),
        })
        .unwrap_or(false)
}

#[derive(Clone, Debug, PartialEq)]
enum Loc {
    Floor,
    Empty,
    Occupied,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Loc>>> {
    separated_list1(line_ending, parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Loc>> {
    many1(parse_loc)(input)
}

fn parse_loc(input: &str) -> IResult<&str, Loc> {
    alt((parse_floor, parse_empty, parse_occupied))(input)
}

fn parse_floor(input: &str) -> IResult<&str, Loc> {
    value(Loc::Floor, complete::char('.'))(input)
}

fn parse_empty(input: &str) -> IResult<&str, Loc> {
    value(Loc::Empty, complete::char('L'))(input)
}

fn parse_occupied(input: &str) -> IResult<&str, Loc> {
    value(Loc::Occupied, complete::char('#'))(input)
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
        assert_eq!(37, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(2277, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(26, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(2066, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
