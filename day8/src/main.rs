use anyhow::Result;
use bit_vec::BitVec;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

const DATA: &str = include_str!("input.txt");
const DATA2: &str = include_str!("input2.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing data: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA2));
    println!("Time spent parsing data2: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Instruction]) -> i32 {
    match run_program(input) {
        Ending::Visited(acc) => acc,
        Ending::EndOfInstructions(_) => panic!("Should end on visited"),
    }
}

fn part_two(input: &[Instruction]) -> i32 {
    let mut last_changed = 0;
    let mut input2 = input.to_vec();
    loop {
        if let Ending::EndOfInstructions(acc) = run_program(&input2) {
            break acc;
        }

        input2 = input.to_vec();
        let (index, _) = input2
            .iter()
            .enumerate()
            .find(|(index, instruction)| {
                index > &last_changed && matches!(instruction, Instruction::Jmp(_))
            })
            .unwrap();
        last_changed = index;
        input2[index] = Instruction::Nop;
    }
}

fn run_program(input: &[Instruction]) -> Ending {
    let mut visited = BitVec::from_elem(1024, false);
    let mut acc = 0;
    let mut index = 0;
    loop {
        if index >= input.len() {
            break Ending::EndOfInstructions(acc);
        }
        if visited[index] {
            break Ending::Visited(acc);
        }
        visited.set(index, true);
        match input[index] {
            Instruction::Acc(val) => {
                acc += val as i32;
                index += 1;
            }
            Instruction::Jmp(val) => {
                if val < 0 {
                    index -= val.unsigned_abs() as usize
                } else {
                    index += val.unsigned_abs() as usize
                }
            }
            Instruction::Nop => index += 1,
        }
    }
}

enum Ending {
    Visited(i32),
    EndOfInstructions(i32),
}

#[derive(Clone, Debug)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop,
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((parse_acc, parse_jmp, parse_nop)).parse(input)
}

fn parse_acc(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("acc "), complete::i16), |val: i16| {
        Instruction::Acc(val)
    })
    .parse(input)
}

fn parse_jmp(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("jmp "), complete::i16), |val: i16| {
        Instruction::Jmp(val)
    })
    .parse(input)
}

fn parse_nop(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Nop, preceded(tag("nop "), complete::i16)).parse(input)
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
        assert_eq!(part_one(&parse_input(TESTDATA)?), 5);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 1723);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 8);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 846);

        Ok(())
    }
}
