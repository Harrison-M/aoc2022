//! Day 8

use std::collections::HashSet;

use util::*;

fn parse(input: &str) -> Result<Vec<Vec<u32>>, Error> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).context("Non-digit input"))
                .collect()
        })
        .collect()
}

fn part1(trees: &[Vec<u32>]) -> Result<usize, Error> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    // Rows
    for (y, row) in trees.iter().enumerate() {
        let mut max: Option<u32> = None;
        for (x, tree) in row.iter().enumerate() {
            if max.map_or(true, |m| *tree > m) {
                seen.insert((x, y));
                max = Some(*tree);
            }
        }
        max = None;
        for (x, tree) in row.iter().enumerate().rev() {
            if max.map_or(true, |m| *tree > m) {
                seen.insert((x, y));
                max = Some(*tree);
            }
        }
    }

    // Columns
    for x in 0..trees.first().context("Trees empty")?.len() {
        let mut max: Option<u32> = None;
        for (y, row) in trees.iter().enumerate() {
            let tree = row.get(x).context("Grid not rectangular")?;
            if max.map_or(true, |m| *tree > m) {
                seen.insert((x, y));
                max = Some(*tree);
            }
        }
        max = None;
        for (y, row) in trees.iter().enumerate().rev() {
            let tree = row.get(x).context("Grid still not rectangular")?;
            if max.map_or(true, |m| *tree > m) {
                seen.insert((x, y));
                max = Some(*tree);
            }
        }
    }

    Ok(seen.len())
}

fn part2(trees: &[Vec<u32>]) -> Result<usize, Error> {
    let mut max = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, treehouse) in row.iter().enumerate() {
            let mut score = 1;
            let mut looking = x;
            let mut distance = 0;

            // Left
            while looking > 0 {
                looking -= 1;
                distance += 1;

                let tree = row.get(looking).context("Out of bounds to left")?;
                if tree >= treehouse {
                    break;
                }
            }

            score *= distance;
            looking = x;
            distance = 0;

            // Right
            while looking < row.len() - 1 {
                looking += 1;
                distance += 1;

                let tree = row.get(looking).context("Out of bounds to right")?;
                if tree >= treehouse {
                    break;
                }
            }

            score *= distance;
            looking = y;
            distance = 0;

            // Up
            while looking > 0 {
                looking -= 1;
                distance += 1;

                let tree = trees
                    .get(looking)
                    .context("Out of bounds to top")?
                    .get(x)
                    .context("Hole in grid")?;

                if tree >= treehouse {
                    break;
                }
            }

            score *= distance;
            looking = y;
            distance = 0;

            // Down
            while looking < trees.len() - 1 {
                looking += 1;
                distance += 1;

                let tree = trees
                    .get(looking)
                    .context("Out of bounds to bottom")?
                    .get(x)
                    .context("Hole in grid")?;

                if tree >= treehouse {
                    break;
                }
            }

            score *= distance;

            max = max.max(score);
        }
    }

    Ok(max)
}

fn main() -> Result<(), Error> {
    let trees = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(&trees)?);
    println!("Part 2: {}", part2(&trees)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(&parse(SAMPLE)?)?, 21);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(part2(&parse(SAMPLE)?)?, 8);
        Ok(())
    }
}
