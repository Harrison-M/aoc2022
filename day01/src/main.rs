//! Day 1

use util::*;

/// Turn lists of numbers separated by blank lines into Vecs
fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|number| number.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect()
}

/// Find max group
fn part1(elves: &[Vec<usize>]) -> usize {
    elves
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .expect("Should be some elves")
}

/// Find sum of top 3 groups
fn part2(elves: &[Vec<usize>]) -> usize {
    let mut sums: Vec<usize> = elves.iter().map(|elf| elf.iter().sum()).collect();

    sums.sort();
    sums.reverse();
    sums.into_iter().take(3).sum()
}

/// Run solver using stdin as puzzle input
fn main() -> Result<(), Error> {
    let elves = parse(&read_stdin()?);
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 45000);
    }
}
