use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
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

fn part_one(input: &HashMap<String, Vec<BagRuleItem>>) -> usize {
    let mut rev_input: HashMap<String, Vec<String>> = HashMap::new();
    for (name, contains) in input {
        for c in contains {
            rev_input
                .entry(c.name.clone())
                .or_insert(vec![])
                .push(name.clone());
        }
    }

    flatten(&rev_input).len() - 1
}

fn flatten(rev_input: &HashMap<String, Vec<String>>) -> Vec<String> {
    traverse(rev_input, vec![], vec![String::from("shiny gold")])
        .iter()
        .flatten()
        .unique()
        .cloned()
        .collect::<Vec<String>>()
}

fn traverse(
    rev_input: &HashMap<String, Vec<String>>,
    current: Vec<String>,
    keys: Vec<String>,
) -> Vec<Vec<String>> {
    println!("Traversing for {:?}", keys);
    if keys.is_empty() {
        return vec![current];
    }

    keys.iter()
        .flat_map(|key| {
            let mut new_current = current.clone();
            new_current.push(key.to_owned());
            match rev_input.get(key) {
                None => vec![new_current],
                Some(new_keys) => traverse(rev_input, new_current, new_keys.to_owned()),
            }
        })
        .collect::<Vec<Vec<String>>>()
}

fn part_two(input: &HashMap<String, Vec<BagRuleItem>>) -> u32 {
    let result = traverse_bags(input, 1, "shiny gold");

    result.calculate() - 1
}

fn traverse_bags(input: &HashMap<String, Vec<BagRuleItem>>, amount: u32, key: &str) -> Bag {
    let bags = input[key]
        .iter()
        .map(|b| traverse_bags(input, b.amount, b.name.as_str()))
        .collect::<Vec<Bag>>();
    Bag::new(amount, bags)
}

#[derive(Debug)]
struct Bag {
    amount: u32,
    contains: Vec<Bag>,
}

impl Bag {
    pub fn new(amount: u32, contains: Vec<Bag>) -> Self {
        Self { amount, contains }
    }

    pub fn calculate(&self) -> u32 {
        self.amount
            + self
                .contains
                .iter()
                .map(|b| self.amount * b.calculate())
                .sum::<u32>()
    }
}

#[derive(Clone, Debug)]
struct BagRuleItem {
    amount: u32,
    name: String,
}

impl BagRuleItem {
    pub fn new<S: Into<String>>(amount: u32, name: S) -> Self {
        BagRuleItem {
            amount,
            name: name.into(),
        }
    }
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Vec<BagRuleItem>>> {
    map(separated_list1(line_ending, parse_bag_rule), |v| {
        v.into_iter().collect::<HashMap<String, Vec<BagRuleItem>>>()
    })(input)
}

fn parse_bag_rule(input: &str) -> IResult<&str, (String, Vec<BagRuleItem>)> {
    map(
        terminated(
            separated_pair(parse_bag_name, tag(" contain "), parse_contains),
            tag("."),
        ),
        |(name, contains)| (name.to_string(), contains),
    )(input)
}

fn parse_bag_name(input: &str) -> IResult<&str, String> {
    map(
        terminated(
            separated_pair(alpha1, tag(" "), alpha1),
            alt((tag(" bags"), tag(" bag"))),
        ),
        |(s, s2)| format!("{s} {s2}"),
    )(input)
}

fn parse_contains(input: &str) -> IResult<&str, Vec<BagRuleItem>> {
    alt((parse_contains_none, parse_contains_some))(input)
}

fn parse_contains_none(input: &str) -> IResult<&str, Vec<BagRuleItem>> {
    value(vec![], tag("no other bags"))(input)
}

fn parse_contains_some(input: &str) -> IResult<&str, Vec<BagRuleItem>> {
    map(
        separated_list1(tag(", "), parse_amount_plus_bag),
        |v: Vec<(u32, String)>| {
            v.iter()
                .map(|(amount, name)| BagRuleItem::new(*amount, name))
                .collect::<Vec<BagRuleItem>>()
        },
    )(input)
}

fn parse_amount_plus_bag(input: &str) -> IResult<&str, (u32, String)> {
    separated_pair(complete::u32, tag(" "), parse_bag_name)(input)
}

fn parse_input(input: &'static str) -> Result<HashMap<String, Vec<BagRuleItem>>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(4, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(233, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(32, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(421550, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
