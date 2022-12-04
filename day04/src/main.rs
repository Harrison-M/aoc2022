//! Day 4

use util::*;

struct Pair {
    elf1_low: usize,
    elf1_high: usize,
    elf2_low: usize,
    elf2_high: usize,
}

fn parse(input: &str) -> Result<Vec<Pair>, Error> {
    input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("No comma in {}", line))?;
            let (elf1_low, elf1_high) = elf1
                .split_once('-')
                .ok_or_else(|| anyhow!("No dash in {}", elf1))?;
            let (elf2_low, elf2_high) = elf2
                .split_once('-')
                .ok_or_else(|| anyhow!("No dash in {}", elf2))?;
            Ok(Pair {
                elf1_low: elf1_low.parse()?,
                elf1_high: elf1_high.parse()?,
                elf2_low: elf2_low.parse()?,
                elf2_high: elf2_high.parse()?,
            })
        })
        .collect()
}

fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|pair| {
            (pair.elf1_low <= pair.elf2_low && pair.elf1_high >= pair.elf2_high)
                || (pair.elf1_low >= pair.elf2_low && pair.elf1_high <= pair.elf2_high)
        })
        .count()
}

fn part2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|pair| {
            (pair.elf1_low >= pair.elf2_low && pair.elf1_low <= pair.elf2_high)
                || (pair.elf1_high >= pair.elf2_low && pair.elf1_high <= pair.elf2_high)
                || (pair.elf2_low >= pair.elf1_low && pair.elf2_low <= pair.elf1_high)
                || (pair.elf2_high >= pair.elf1_low && pair.elf2_high <= pair.elf1_high)
        })
        .count()
}

fn main() -> Result<(), Error> {
    let pairs = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE).unwrap()), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE).unwrap()), 4);
    }
}
