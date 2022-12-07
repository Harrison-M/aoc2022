//! Day 6

use std::collections::HashSet;

use util::*;

/// Find the number of characters you need to read before you read a series of n unique characters
fn find_unique(input: &str, count: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(count)
        .enumerate()
        .find_map(|(i, window)| {
            if window.iter().collect::<HashSet<&u8>>().len() == count {
                Some(i + count)
            } else {
                None
            }
        })
}

fn part1(input: &str) -> Option<usize> {
    find_unique(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    find_unique(input, 14)
}

fn main() -> Result<(), Error> {
    let input = read_stdin()?;
    println!(
        "Part 1: {}",
        part1(&input).map_or("Failed".to_string(), |n| n.to_string())
    );
    println!(
        "Part 2: {}",
        part2(&input).map_or("Failed".to_string(), |n| n.to_string())
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn part1_examples() {
        assert_eq!(part1(SAMPLES[0]), Some(7));
        assert_eq!(part1(SAMPLES[1]), Some(5));
        assert_eq!(part1(SAMPLES[2]), Some(6));
        assert_eq!(part1(SAMPLES[3]), Some(10));
        assert_eq!(part1(SAMPLES[4]), Some(11));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(SAMPLES[0]), Some(19));
        assert_eq!(part2(SAMPLES[1]), Some(23));
        assert_eq!(part2(SAMPLES[2]), Some(23));
        assert_eq!(part2(SAMPLES[3]), Some(29));
        assert_eq!(part2(SAMPLES[4]), Some(26));
    }
}
