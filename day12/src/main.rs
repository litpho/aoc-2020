use anyhow::Result;
use nom::{
    character::complete::{self, line_ending, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
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

fn part_one(input: &[Instruction]) -> usize {
    let mut pos = Position::default();
    for i in input {
        pos = new_position_one(pos, i);
        println!("{:?}: {:?}", i, pos);
    }

    pos.x.unsigned_abs() + pos.y.unsigned_abs()
}

fn new_position_one(pos: Position, instruction: &Instruction) -> Position {
    let mut new_x = pos.x;
    let mut new_y = pos.y;
    let mut new_facing = pos.facing;
    match instruction {
        Instruction::North(value) => new_y += *value as isize,
        Instruction::East(value) => new_x += *value as isize,
        Instruction::South(value) => new_y -= *value as isize,
        Instruction::West(value) => new_x -= *value as isize,
        Instruction::Left(degrees) => new_facing = new_facing.new_facing(*degrees, false),
        Instruction::Right(degrees) => new_facing = new_facing.new_facing(*degrees, true),
        Instruction::Forward(value) => match new_facing {
            Facing::North => new_y += *value as isize,
            Facing::East => new_x += *value as isize,
            Facing::South => new_y -= *value as isize,
            Facing::West => new_x -= *value as isize,
        },
    }
    Position::new(new_x, new_y, new_facing)
}

fn part_two(input: &[Instruction]) -> usize {
    let mut ship = (0, 0);
    let mut waypoint = Position::new(10, 1, Facing::North);
    for i in input {
        let result = new_positions_two(ship, waypoint, i);
        ship = result.0;
        waypoint = result.1;
        println!("{:?}: {:?} - {:?}", i, ship, waypoint);
    }

    ship.0.unsigned_abs() + ship.1.unsigned_abs()
}

fn new_positions_two(
    pos: (isize, isize),
    waypoint: Position,
    instruction: &Instruction,
) -> ((isize, isize), Position) {
    let mut ship_x = pos.0;
    let mut ship_y = pos.1;
    let mut waypoint_x = waypoint.x;
    let mut waypoint_y = waypoint.y;
    let mut waypoint_facing = waypoint.facing;
    match instruction {
        Instruction::North(value) => waypoint_y += *value as isize,
        Instruction::East(value) => waypoint_x += *value as isize,
        Instruction::South(value) => waypoint_y -= *value as isize,
        Instruction::West(value) => waypoint_x -= *value as isize,
        Instruction::Left(degrees) => {
            let old_facing = waypoint_facing;
            waypoint_facing = waypoint_facing.new_facing(*degrees, false);
            (waypoint_x, waypoint_y) =
                adjust_waypoint(&old_facing, &waypoint_facing, waypoint_x, waypoint_y);
        }
        Instruction::Right(degrees) => {
            let old_facing = waypoint_facing;
            waypoint_facing = waypoint_facing.new_facing(*degrees, true);
            (waypoint_x, waypoint_y) =
                adjust_waypoint(&old_facing, &waypoint_facing, waypoint_x, waypoint_y);
        }
        Instruction::Forward(value) => {
            ship_x += waypoint_x * (*value as isize);
            ship_y += waypoint_y * (*value as isize);
        }
    }
    let waypoint = adjust_facing(waypoint_x, waypoint_y, waypoint_facing);
    ((ship_x, ship_y), waypoint)
}

fn adjust_waypoint(
    old_facing: &Facing,
    new_facing: &Facing,
    waypoint_x: isize,
    waypoint_y: isize,
) -> (isize, isize) {
    match (old_facing, new_facing) {
        (Facing::North, Facing::East) => (waypoint_y, -waypoint_x),
        (Facing::North, Facing::South) => (-waypoint_x, -waypoint_y),
        (Facing::North, Facing::West) => (-waypoint_y, waypoint_x),
        (Facing::East, Facing::South) => (waypoint_y, -waypoint_x),
        (Facing::East, Facing::West) => (-waypoint_x, -waypoint_y),
        (Facing::East, Facing::North) => (-waypoint_y, waypoint_x),
        (Facing::South, Facing::West) => (waypoint_y, -waypoint_x),
        (Facing::South, Facing::North) => (-waypoint_x, -waypoint_y),
        (Facing::South, Facing::East) => (-waypoint_y, waypoint_x),
        (Facing::West, Facing::North) => (waypoint_y, -waypoint_x),
        (Facing::West, Facing::East) => (-waypoint_x, -waypoint_y),
        (Facing::West, Facing::South) => (-waypoint_y, waypoint_x),
        _ => (waypoint_x, waypoint_y),
    }
}

fn adjust_facing(x: isize, y: isize, facing: Facing) -> Position {
    let new_facing: Facing = match facing {
        Facing::North if y < 0 => Facing::South,
        Facing::East if x < 0 => Facing::West,
        Facing::South if y > 0 => Facing::North,
        Facing::West if x > 0 => Facing::East,
        _ if x == 0 && y > 0 => Facing::North,
        _ if y == 0 && x > 0 => Facing::East,
        _ if x == 0 && y < 0 => Facing::South,
        _ if y == 0 && x < 0 => Facing::West,
        _ => facing,
    };
    Position::new(x, y, new_facing)
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    facing: Facing,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Facing::East,
        }
    }
}

impl Position {
    pub fn new(x: isize, y: isize, facing: Facing) -> Self {
        Self { x, y, facing }
    }
}

impl Facing {
    pub fn new_facing(self, degrees: usize, clockwise: bool) -> Self {
        if clockwise {
            match (self as isize + degrees as isize).rem_euclid(360) {
                0 => Facing::North,
                90 => Facing::East,
                180 => Facing::South,
                270 => Facing::West,
                _ => panic!(),
            }
        } else {
            match (self as isize + 360 - degrees as isize).rem_euclid(360) {
                0 => Facing::North,
                90 => Facing::East,
                180 => Facing::South,
                270 => Facing::West,
                _ => panic!(),
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Facing {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        pair(parse_action, complete::u16),
        |(action, value)| match action {
            'N' => Instruction::North(value as usize),
            'E' => Instruction::East(value as usize),
            'S' => Instruction::South(value as usize),
            'W' => Instruction::West(value as usize),
            'L' => Instruction::Left(value as usize),
            'R' => Instruction::Right(value as usize),
            'F' => Instruction::Forward(value as usize),
            _ => panic!("Illegal value"),
        },
    )(input)
}

fn parse_action(input: &str) -> IResult<&str, char> {
    one_of("NESWLRF")(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    println!("{:?}", input);

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(25, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(319, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(286, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(50157, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
