//! Day 14

use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    error::VerboseError,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use point_2d::Point2D;
use util::*;

#[derive(Clone)]
struct Cave {
    occupied: HashSet<Point2D<isize>>,
    bottom: isize,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Point2D<isize>>>, VerboseError<&str>> {
    separated_list1(
        line_ending,
        separated_list1(
            tag(" -> "),
            map_res(
                separated_pair(
                    digit1::<&str, VerboseError<&str>>,
                    tag(","),
                    digit1::<&str, VerboseError<&str>>,
                ),
                |(x, y)| -> Result<Point2D<isize>, ParseIntError> {
                    Ok(Point2D(x.parse()?, y.parse()?))
                },
            ),
        ),
    )(input)
}

fn draw_cave(instructions: &[Vec<Point2D<isize>>]) -> Cave {
    let mut occupied: HashSet<Point2D<isize>> = HashSet::new();
    let mut bottom = 0;
    for path in instructions.iter() {
        let mut iter = path.iter();
        let mut prev = iter.next().unwrap();
        occupied.insert(*prev);

        for next in iter {
            if prev.0 == next.0 {
                let range = if prev.1 < next.1 {
                    prev.1..=next.1
                } else {
                    next.1..=prev.1
                };

                for y in range {
                    occupied.insert(Point2D(prev.0, y));
                    bottom = bottom.max(y);
                }
            } else {
                let range = if prev.0 < next.0 {
                    prev.0..=next.0
                } else {
                    next.0..=prev.0
                };

                for x in range {
                    occupied.insert(Point2D(x, prev.1));
                }
                bottom = bottom.max(prev.1);
            }

            prev = next;
        }
    }
    Cave { occupied, bottom }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
            .map(|instructions| draw_cave(&instructions.1))
            .map_err(|e| anyhow!("{}", e))
    }
}

impl Cave {
    /// Drops a grain of sand and returns true if it came to rest
    fn drop_sand(&mut self) -> bool {
        let mut position: Point2D<isize> = Point2D(500, 0);

        let candidate_moves: Vec<Point2D<isize>> =
            vec![Point2D(0, 1), Point2D(-1, 1), Point2D(1, 1)];

        while let Some(next) = candidate_moves.iter().find_map(|m| {
            let candidate = &position + m;
            (!self.occupied.contains(&candidate)).then_some(candidate)
        }) {
            position = next;

            if position.1 > self.bottom {
                return false;
            }
        }

        self.occupied.insert(position);

        true
    }

    /// Drops a grain of sand and returns true if it came to rest
    fn drop_sand_with_floor(&mut self) -> bool {
        let mut position: Point2D<isize> = Point2D(500, 0);

        if self.occupied.contains(&position) {
            return false;
        }

        let candidate_moves: Vec<Point2D<isize>> =
            vec![Point2D(0, 1), Point2D(-1, 1), Point2D(1, 1)];

        while let Some(next) = candidate_moves.iter().find_map(|m| {
            let candidate = &position + m;
            (!self.occupied.contains(&candidate)).then_some(candidate)
        }) {
            position = next;

            if position.1 == self.bottom + 1 {
                break;
            }
        }

        self.occupied.insert(position);

        true
    }
}

fn part1(cave: &mut Cave) -> usize {
    let mut grains = 0;
    loop {
        if !cave.drop_sand() {
            break;
        }
        grains += 1;
    }
    grains
}

fn part2(cave: &mut Cave) -> usize {
    let mut grains = 0;
    loop {
        if !cave.drop_sand_with_floor() {
            break;
        }
        grains += 1;
    }
    grains
}

fn main() -> Result<(), Error> {
    let mut cave: Cave = read_stdin()?.parse()?;
    println!("Part 1: {}", part1(&mut cave.clone()));
    println!("Part 2: {}", part2(&mut cave));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(&mut SAMPLE.parse()?), 24);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(part2(&mut SAMPLE.parse()?), 93);
        Ok(())
    }
}
