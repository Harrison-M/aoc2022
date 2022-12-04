//! Day 3

use std::collections::HashSet;
use util::*;

/// Convert an ascii-range letter character to a priority value
fn to_priority(byte: u8) -> Result<u8, Error> {
    if byte.is_ascii_lowercase() {
        Ok(byte - 96)
    } else if byte.is_ascii_uppercase() {
        Ok(byte - 38)
    } else {
        bail!("Invalid character {} provided", char::from(byte))
    }
}

/// Find common element in two sides of sacks
fn part1(input: &str) -> Result<usize, Error> {
    input.lines().try_fold(0, |acc, sack| {
        let bytes = sack.as_bytes();
        let (left, right) = bytes.split_at(bytes.len() / 2);
        let left_set: HashSet<&u8> = left.iter().collect();
        let right_set: HashSet<&u8> = right.iter().collect();
        let item = left_set
            .intersection(&right_set)
            .next()
            .ok_or_else(|| anyhow!("No common item found in compartments of {sack}"))?;
        Ok(acc + usize::from(to_priority(**item)?))
    })
}

/// Find common element in groups of three sacks
fn part2(input: &str) -> Result<usize, Error> {
    let mut sum: usize = 0;
    let mut lines = input.lines();
    loop {
        let next = (lines.next(), lines.next(), lines.next());

        match next {
            (Some(bag1), Some(bag2), Some(bag3)) => {
                let set1: HashSet<&u8> = bag1.as_bytes().iter().collect();
                let set2: HashSet<&u8> = bag2.as_bytes().iter().collect();
                let set3: HashSet<&u8> = bag3.as_bytes().iter().collect();

                let first_intersection: HashSet<&u8> = set1.intersection(&set2).copied().collect();
                let badge = first_intersection
                    .intersection(&set3)
                    .next()
                    .ok_or_else(|| anyhow!("No badge found in {:?}", next))?;

                sum += usize::from(to_priority(**badge)?)
            }
            _ => break,
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Error> {
    let sacks = read_stdin()?;
    println!("Part 1: {}", part1(&sacks)?);
    println!("Part 2: {}", part2(&sacks)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn calculates_priorities() {
        let test_bytes = b"azAZ5";
        assert_eq!(to_priority(test_bytes[0]).unwrap(), 1);
        assert_eq!(to_priority(test_bytes[1]).unwrap(), 26);
        assert_eq!(to_priority(test_bytes[2]).unwrap(), 27);
        assert_eq!(to_priority(test_bytes[3]).unwrap(), 52);
        assert!(to_priority(test_bytes[4]).is_err());
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE).unwrap(), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE).unwrap(), 70);
    }
}
