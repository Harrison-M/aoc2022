//! Day 5

use regex::Regex;
use util::*;

#[derive(Clone, Debug)]
struct Step {
    quantity: usize,
    source: usize,
    destination: usize,
}

#[derive(Clone)]
struct CrateProcedure {
    crates: Vec<Vec<char>>,
    steps: Vec<Step>,
}

const MOVE_RE: &str = r"move (\d+) from (\d) to (\d)";

fn parse(input: &str) -> Result<CrateProcedure, Error> {
    let (crate_input, step_input) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("No crate/step split"))?;

    let mut crate_lines: Vec<_> = crate_input.lines().rev().map(|line| line.chars()).collect();

    let mut crates: Vec<Vec<char>> = vec![];

    loop {
        let mut line_iter = crate_lines.iter_mut();
        let label_opt = line_iter.next().and_then(|label_line| label_line.next());

        if let Some(label) = label_opt {
            if !label.is_ascii_digit() {
                // Advance all character iterators without using result
                for line in line_iter {
                    line.next();
                }
                continue;
            }

            crates.push(
                line_iter
                    .filter_map(|line| line.next().filter(|c| c.is_alphabetic()))
                    .collect(),
            );
        } else {
            break;
        }
    }

    let step_re = Regex::new(MOVE_RE)?;

    let steps: Vec<Step> = step_input
        .lines()
        .map(|line| {
            let caps = step_re
                .captures(line)
                .ok_or_else(|| anyhow!("Failed to match step {}", line))?;

            let quantity = caps
                .get(1)
                .ok_or_else(|| anyhow!("Capture 1 missing in {}", line))
                .and_then(|num| num.as_str().parse().map_err(Error::from))?;
            let source = caps
                .get(2)
                .ok_or_else(|| anyhow!("Capture 2 missing in {}", line))
                .and_then(|num| num.as_str().parse().map_err(Error::from))?;
            let destination = caps
                .get(3)
                .ok_or_else(|| anyhow!("Capture 3 missing in {}", line))
                .and_then(|num| num.as_str().parse().map_err(Error::from))?;

            Ok(Step {
                quantity,
                source,
                destination,
            })
        })
        .collect::<Result<Vec<Step>, Error>>()?;

    Ok(CrateProcedure { crates, steps })
}

fn part1(mut crate_procedure: CrateProcedure) -> Result<String, Error> {
    for step in crate_procedure.steps.iter() {
        for _ in 0..step.quantity {
            let to_move = crate_procedure
                .crates
                .get_mut(step.source - 1)
                .ok_or_else(|| anyhow!("No pile at source index {}", step.source - 1))?
                .pop()
                .ok_or_else(|| anyhow!("Pile at source index {} is empty", step.source - 1))?;

            crate_procedure
                .crates
                .get_mut(step.destination - 1)
                .ok_or_else(|| anyhow!("No pile at destination index {}", step.destination - 1))?
                .push(to_move);
        }
    }

    Ok(crate_procedure
        .crates
        .iter()
        .filter_map(|pile| pile.last())
        .collect())
}

fn part2(mut crate_procedure: CrateProcedure) -> Result<String, Error> {
    for step in crate_procedure.steps.iter() {
        let mut to_move = crate_procedure
            .crates
            .get_mut(step.source - 1)
            .ok_or_else(|| anyhow!("No pile at source index {}", step.source - 1))
            .and_then(|pile| {
                if pile.len() >= step.quantity {
                    Ok(pile.split_off(pile.len() - step.quantity))
                } else {
                    bail!("Pile at source index {} is too small", step.source - 1)
                }
            })?;

        crate_procedure
            .crates
            .get_mut(step.destination - 1)
            .ok_or_else(|| anyhow!("No pile at destination index {}", step.destination - 1))?
            .append(&mut to_move);
    }

    Ok(crate_procedure
        .crates
        .iter()
        .filter_map(|pile| pile.last())
        .collect())
}

fn main() -> Result<(), Error> {
    let crate_proc = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(crate_proc.clone())?);
    println!("Part 2: {}", part2(crate_proc)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn crate_parse() {
        let CrateProcedure { crates, steps: _ } = parse(SAMPLE).unwrap();

        assert_eq!(crates[0], vec!['Z', 'N']);
        assert_eq!(crates[1], vec!['M', 'C', 'D']);
        assert_eq!(crates[2], vec!['P']);
    }

    #[test]
    fn part1_example() {
        assert_eq!(&part1(parse(SAMPLE).unwrap()).unwrap(), "CMZ");
    }

    #[test]
    fn part2_example() {
        assert_eq!(&part2(parse(SAMPLE).unwrap()).unwrap(), "MCD");
    }
}
