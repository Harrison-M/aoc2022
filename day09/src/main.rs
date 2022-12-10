//! Day 9

use std::{cmp::Ordering, collections::HashSet};

use point_2d::Point2D;
use util::*;

fn parse(input: &str) -> Result<Vec<(Point2D<isize>, isize)>, Error> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .with_context(|| format!("Missing space in line: {line}"))
                .and_then(|(dir, distance)| {
                    Ok((
                        match dir {
                            "L" => Point2D(-1, 0),
                            "R" => Point2D(1, 0),
                            "D" => Point2D(0, -1),
                            "U" => Point2D(0, 1),
                            d => bail!("Invalid direction {}", d),
                        },
                        distance.parse()?,
                    ))
                })
        })
        .collect()
}

/// According to the provided rules, have a trailing segment of rope follow a leading segment
fn move_follower(head: &mut Point2D<isize>, tail: &mut Point2D<isize>) {
    if head.0 - tail.0 >= 2 {
        tail.0 += 1;
        tail.1 += match tail.1.cmp(&head.1) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0,
        }
    }

    if head.0 - tail.0 <= -2 {
        tail.0 -= 1;
        tail.1 += match tail.1.cmp(&head.1) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0,
        }
    }

    if head.1 - tail.1 >= 2 {
        tail.1 += 1;
        tail.0 += match tail.0.cmp(&head.0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0,
        }
    }

    if head.1 - tail.1 <= -2 {
        tail.1 -= 1;
        tail.0 += match tail.0.cmp(&head.0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0,
        }
    }
}

fn part1(moves: &[(Point2D<isize>, isize)]) -> usize {
    let mut points: HashSet<Point2D<isize>> = HashSet::new();

    let mut head: Point2D<isize> = Point2D(0, 0);
    let mut tail = head;

    for (dir, distance) in moves.iter().copied() {
        for _ in 0..distance {
            head += dir;

            move_follower(&mut head, &mut tail);

            points.insert(tail);
        }
    }

    points.len()
}

fn part2(moves: &[(Point2D<isize>, isize)]) -> usize {
    let mut rope: Vec<Point2D<isize>> = (0..10).map(|_| Point2D::default()).collect();
    let mut points: HashSet<Point2D<isize>> = HashSet::new();

    for (dir, distance) in moves.iter().copied() {
        for _ in 0..distance {
            rope[0] += dir;

            for back_idx in 1..=9 {
                let front_idx = back_idx - 1;
                let mut pair = rope[front_idx..=back_idx].iter_mut();
                let front = pair.next().unwrap();
                let back = pair.next().unwrap();
                move_follower(front, back);
            }

            points.insert(rope[9]);
        }
    }

    points.len()
}

fn main() -> Result<(), Error> {
    let moves = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");
    const SAMPLE2: &str = include_str!("./sample2");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(&parse(SAMPLE)?), 13);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(part2(&parse(SAMPLE)?), 1);
        assert_eq!(part2(&parse(SAMPLE2)?), 36);
        Ok(())
    }
}
