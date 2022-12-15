//! Day 13

use std::{cmp::Ordering, slice};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    error::VerboseError,
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use util::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    List(Vec<Expr>),
    Int(usize),
}

fn parse_num(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map_res(digit1, |num: &str| num.parse::<usize>().map(Expr::Int))(input)
}

fn parse_list(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map(
        delimited(tag("["), separated_list0(tag(","), parse_expr), tag("]")),
        Expr::List,
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((parse_num, parse_list))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(Expr, Expr)>, VerboseError<&str>> {
    separated_list1(
        count(line_ending, 2),
        separated_pair(parse_expr, line_ending, parse_expr),
    )(input)
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expr::Int(l), Expr::Int(r)) => l.cmp(r),
            (Expr::List(l), Expr::List(r)) => l.cmp(r),
            (l, Expr::List(r)) => slice::from_ref(l).cmp(r),
            (Expr::List(l), r) => l[..].cmp(slice::from_ref(r)),
        }
    }
}

fn part1(pairs: &[(Expr, Expr)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(pairs: &[(Expr, Expr)]) -> usize {
    let mut lists: Vec<&Expr> = pairs.iter().flat_map(|(a, b)| [a, b]).collect();
    let divider1 = Expr::List(vec![Expr::List(vec![Expr::Int(2)])]);
    let divider2 = Expr::List(vec![Expr::List(vec![Expr::Int(6)])]);

    lists.push(&divider1);
    lists.push(&divider2);

    lists.sort();

    let index1 = lists
        .iter()
        .enumerate()
        .find_map(|(i, l)| (l == &&divider1).then_some(i + 1))
        .unwrap();
    let index2 = lists
        .iter()
        .enumerate()
        .find_map(|(i, l)| (l == &&divider2).then_some(i + 1))
        .unwrap();

    index1 * index2
}

fn main() -> Result<(), Error> {
    let pairs = parse(&read_stdin()?).map_err(|e| anyhow!("{}", e))?.1;
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(&parse(SAMPLE).map_err(|e| anyhow!("{}", e))?.1), 13);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(part2(&parse(SAMPLE).map_err(|e| anyhow!("{}", e))?.1), 140);
        Ok(())
    }
}
