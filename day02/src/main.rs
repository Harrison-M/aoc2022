//! Day 2

use util::*;

/// Parse games. A/X = 1, B/Y = 2, C/Z = 3
fn parse(input: &str) -> Result<Vec<(isize, isize)>, Error> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let them = match chars.next() {
                Some('A') => 1,
                Some('B') => 2,
                Some('C') => 3,
                _ => bail!("Invalid player move"),
            };
            let me = match chars.next_back() {
                Some('X') => 1,
                Some('Y') => 2,
                Some('Z') => 3,
                _ => bail!("Invalid opponent move"),
            };
            Ok((them, me))
        })
        .collect()
}

/// Tally up game scores
fn part1(guide: &[(isize, isize)]) -> isize {
    guide
        .iter()
        .map(|(them, me)| {
            let difference = me - them;

            (match difference {
                1 | -2 => 6,
                0 => 3,
                _ => 0,
            }) + me
        })
        .sum()
}

/// Figure out moves from expected game results
fn part2(guide: &[(isize, isize)]) -> isize {
    guide
        .iter()
        .map(|game| match game {
            (3, 3) => 7,
            (1, 1) => 3,
            (m, 1) => m - 1,
            (m, 2) => m + 3,
            (m, 3) => m + 7,
            _ => {
                println!("Invalid game {:?} found", guide);
                0
            }
        })
        .sum()
}

/// Run solver using stdin as puzzle input
fn main() -> Result<(), Error> {
    let guide = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(&guide));
    println!("Part 2: {}", part2(&guide));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE).unwrap()), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE).unwrap()), 12);
    }
}
