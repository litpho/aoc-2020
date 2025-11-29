use anyhow::Result;
use nom::{
    character::{complete::line_ending, complete::one_of},
    combinator::map,
    multi::{many1, separated_list1},
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

fn part_one(input: &[Vec<bool>]) -> usize {
    traverse(input, 3, 1)
}

fn part_two(input: &[Vec<bool>]) -> usize {
    let count = traverse(input, 1, 1);
    let count_2 = traverse(input, 3, 1);
    let count_3 = traverse(input, 5, 1);
    let count_4 = traverse(input, 7, 1);
    let count_5 = traverse(input, 1, 2);

    count * count_2 * count_3 * count_4 * count_5
}

fn traverse(input: &[Vec<bool>], x_step: usize, y_step: usize) -> usize {
    let line_length = input[0].len();

    let (_, count) = input
        .iter()
        .skip(y_step)
        .step_by(y_step)
        .fold((0, 0), |(x, count), line| {
            let x = (x + x_step) % line_length;
            let count = if line[x] { count + 1 } else { count };
            (x, count)
        });

    count
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<bool>> {
    many1(parse_position).parse(input)
}

fn parse_position(input: &str) -> IResult<&str, bool> {
    map(one_of(".#"), |x| x == '#').parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<bool>>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(7, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(151, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(336, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(7540141059, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
