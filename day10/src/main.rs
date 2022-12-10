//! Day 10

use std::{convert::TryInto, fmt::Write};

use util::*;

fn parse(input: &str) -> Result<Vec<Option<isize>>, Error> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ') // "noop" will return None
                .map(|split| match split {
                    ("addx", val) => val.parse().map_err(Error::from),
                    _ => bail!("Invalid instruction: {}", line),
                })
                .transpose()
        })
        .collect()
}

/// Record the value of X during each cycle and return them in sequence
fn x_over_time(instructions: &[Option<isize>]) -> Vec<isize> {
    let mut x = 1;
    let mut values: Vec<isize> = vec![];

    for instruction in instructions {
        match instruction {
            Some(val) => {
                values.push(x);
                values.push(x);
                x += val;
            }
            None => values.push(x),
        }
    }

    values
}

fn part1(instructions: &[Option<isize>]) -> Result<isize, Error> {
    let values = x_over_time(instructions);

    if values.len() < 220 {
        bail!("Insufficient cycles run");
    }

    Ok(values[19] * 20
        + values[59] * 60
        + values[99] * 100
        + values[139] * 140
        + values[179] * 180
        + values[219] * 220)
}

fn part2(instructions: &[Option<isize>]) -> Result<String, Error> {
    let values = x_over_time(instructions);

    let chars: Vec<char> = values
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            if x.abs_diff(TryInto::<isize>::try_into(i)? % 40) <= 1 {
                Ok('#')
            } else {
                Ok('.')
            }
        })
        .collect::<Result<Vec<char>, Error>>()?;

    let mut output: String = String::new();

    for chunk in chars.chunks_exact(40) {
        writeln!(output, "{}", chunk.iter().collect::<String>())?;
    }
    Ok(output)
}

fn main() -> Result<(), Error> {
    let instructions = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(&instructions)?);
    println!("Part 2:\n{}", part2(&instructions)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");
    const SAMPLE_RESULT: &str = include_str!("./sample_result");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(&parse(SAMPLE)?)?, 13140);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(&part2(&parse(SAMPLE)?)?, SAMPLE_RESULT);
        Ok(())
    }
}
