//! Day 11

use std::str::FromStr;

use util::*;

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    AddSelf,
    MulSelf,
}

// PART 1

#[derive(Clone)]
struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    operation: Operation,
    test_mod: usize,
    test_true: usize,
    test_false: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        lines.next(); // Skip monkey number

        // Items
        let items: Vec<usize> = lines
            .next()
            .context("Not enough lines")?
            .split_once(':')
            .context("No colon on item line")?
            .1
            .split(',')
            .map(|item| item.trim().parse().map_err(Error::from))
            .collect::<Result<Vec<usize>, Error>>()?;

        let mut op_tokens = lines
            .next()
            .context("Not enough lines")?
            .split_once('=')
            .context("No = on op line")?
            .1
            .trim()
            .split(' ');

        let lhs = op_tokens.next().context("No lhs")?;
        let operator = op_tokens.next().context("No operator")?;
        let rhs = op_tokens.next().context("No rhs")?;

        let operation = match (lhs, operator, rhs) {
            ("old", "+", "old") => Operation::AddSelf,
            ("old", "*", "old") => Operation::MulSelf,
            ("old", "+", num) => Operation::Add(num.parse()?),
            ("old", "*", num) => Operation::Mul(num.parse()?),
            _ => bail!("Invalid operation"),
        };

        let test_mod: usize = lines
            .next()
            .context("Not enough lines")?
            .rsplit_once(' ')
            .context("No space on test mod line")?
            .1
            .parse()?;

        let test_true = lines
            .next()
            .context("Not enough lines")?
            .rsplit_once(' ')
            .context("No space on test true line")?
            .1
            .parse()?;

        let test_false = lines
            .next()
            .context("Not enough lines")?
            .rsplit_once(' ')
            .context("No space on test false line")?
            .1
            .parse()?;

        Ok(Monkey {
            inspections: 0,
            items,
            operation,
            test_mod,
            test_true,
            test_false,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>, Error> {
    input.split("\n\n").map(|block| block.parse()).collect()
}

/// Run a round as indicated in part 1
fn turn(monkeys: &mut [Monkey]) -> Result<(), Error> {
    // Using index instead of iterator to avoid taking an exclusive borrow on monkeys
    for i in 0..monkeys.len() {
        // (item, target)
        let throws: Vec<(usize, usize)> = {
            let monkey = &monkeys[i];
            monkey
                .items
                .iter()
                .map(|item| {
                    let worry = match monkey.operation {
                        Operation::AddSelf => item + item,
                        Operation::MulSelf => item * item,
                        Operation::Add(val) => item + val,
                        Operation::Mul(val) => item * val,
                    } / 3;

                    if worry % monkey.test_mod == 0 {
                        (worry, monkey.test_true)
                    } else {
                        (worry, monkey.test_false)
                    }
                })
                .collect()
        };

        let monkey = monkeys.get_mut(i).unwrap();
        monkey.items.clear();
        monkey.inspections += throws.len();

        for (item, target) in throws {
            monkeys
                .get_mut(target)
                .with_context(|| format!("Invalid target {}", target))?
                .items
                .push(item);
        }
    }

    Ok(())
}

fn part1(mut monkeys: Vec<Monkey>) -> Result<usize, Error> {
    for _ in 0..20 {
        turn(&mut monkeys)?;
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();

    inspections.sort();
    inspections.reverse();

    Ok(inspections.first().context("No first value")?
        * inspections.get(1).context("No second value")?)
}

// PART 2

/// A monkey where each of its held items keeps track of its value modulo the test value of each
/// monkey.
struct ModularMonkey {
    inspections: usize,
    items: Vec<Vec<usize>>,
    operation: Operation,
    test_index: usize,
    test_true: usize,
    test_false: usize,
}

/// A keep away game, tracking the monkeys involved as well as a list of the modulo each monkey
/// checks
struct ModularKeepAway {
    mods: Vec<usize>,
    monkeys: Vec<ModularMonkey>,
}

impl ModularKeepAway {
    /// Run a round as indicated in part 2
    fn round(&mut self) -> Result<(), Error> {
        // Using index instead of iterator to avoid taking an exclusive borrow on monkeys
        for i in 0..self.monkeys.len() {
            // (item, target)
            let throws: Vec<(Vec<usize>, usize)> = {
                let monkey = &self.monkeys[i];
                monkey
                    .items
                    .iter()
                    .map(|item| {
                        let worry: Vec<usize> = item
                            .iter()
                            .zip(self.mods.iter())
                            .map(|(item_mod, m)| {
                                (match monkey.operation {
                                    Operation::AddSelf => item_mod + item_mod,
                                    Operation::MulSelf => item_mod * item_mod,
                                    Operation::Add(val) => item_mod + val,
                                    Operation::Mul(val) => item_mod * val,
                                }) % m
                            })
                            .collect();
                        if worry[monkey.test_index] == 0 {
                            (worry, monkey.test_true)
                        } else {
                            (worry, monkey.test_false)
                        }
                    })
                    .collect()
            };

            let monkey = self.monkeys.get_mut(i).unwrap();
            monkey.items.clear();
            monkey.inspections += throws.len();

            for (item, target) in throws {
                self.monkeys
                    .get_mut(target)
                    .with_context(|| format!("Invalid target {}", target))?
                    .items
                    .push(item);
            }
        }

        Ok(())
    }
}

/// Convert a part 1 monkey list to a part 2 keep away game
fn monkeys_to_modulars(monkeys: Vec<Monkey>) -> ModularKeepAway {
    let mods: Vec<usize> = monkeys.iter().map(|m| m.test_mod).collect();

    let mod_monkeys: Vec<ModularMonkey> = monkeys
        .into_iter()
        .enumerate()
        .map(|(i, monkey)| ModularMonkey {
            inspections: monkey.inspections,
            items: monkey
                .items
                .iter()
                .map(|item| mods.iter().map(|m| item % m).collect())
                .collect(),
            operation: monkey.operation,
            test_index: i,
            test_true: monkey.test_true,
            test_false: monkey.test_false,
        })
        .collect();

    ModularKeepAway {
        mods,
        monkeys: mod_monkeys,
    }
}

fn part2(monkeys: Vec<Monkey>) -> Result<usize, Error> {
    let mut mod_monkeys = monkeys_to_modulars(monkeys);

    for _ in 0..10000 {
        mod_monkeys.round()?;
    }

    let mut inspections: Vec<usize> = mod_monkeys.monkeys.iter().map(|m| m.inspections).collect();

    inspections.sort();
    inspections.reverse();

    Ok(inspections.first().context("No first value")?
        * inspections.get(1).context("No second value")?)
}

fn main() -> Result<(), Error> {
    let monkeys = parse(&read_stdin()?)?;
    println!("Part 1: {}", part1(monkeys.clone())?);
    println!("Part 2: {}", part2(monkeys)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn part1_example() -> Result<(), Error> {
        assert_eq!(part1(parse(SAMPLE)?)?, 10605);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        assert_eq!(part2(parse(SAMPLE)?)?, 2713310158);
        Ok(())
    }
}
