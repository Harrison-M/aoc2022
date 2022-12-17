//! Day 16

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use util::*;

struct Room {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn parse_valve_tuple(
    input: &str,
) -> IResult<&str, (String, usize, Vec<String>), VerboseError<&str>> {
    tuple((
        preceded(tag("Valve "), map(alpha1, String::from)),
        preceded(
            tag(" has flow rate="),
            map_res(digit1, |num: &str| num.parse::<usize>()),
        ),
        preceded(
            alt((
                tag("; tunnel leads to valve "),
                tag("; tunnels lead to valves "),
            )),
            separated_list1(tag(", "), map(alpha1, String::from)),
        ),
    ))(input)
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (name, flow_rate, tunnels)) = parse_valve_tuple(s)
            .finish()
            .map_err(|e| anyhow!("{}", e))?;

        Ok(Room {
            name,
            flow_rate,
            tunnels,
        })
    }
}

fn parse_valve_list(input: &str) -> Result<HashMap<String, Room>, Error> {
    input
        .lines()
        .map(|line| {
            let room: Room = line.parse()?;
            Ok((room.name.clone(), room))
        })
        .collect()
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: String,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distances_from(current: String, rooms: &HashMap<String, Room>) -> HashMap<String, usize> {
    let mut distances: HashMap<String, usize> = rooms
        .keys()
        .cloned()
        .map(|room| {
            if room == current {
                (room, 0)
            } else {
                (room, usize::MAX)
            }
        })
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: current,
    });

    // Adapted from the Djikstra example in the std::collections::binary_heap docs
    while let Some(State { cost, position }) = heap.pop() {
        // Important as we may have already found a better way
        if cost > *distances.get(&position).unwrap() {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in rooms.get(&position).unwrap().tunnels.iter() {
            let next = State {
                cost: cost + 1,
                position: edge.clone(),
            };

            // If so, add it to the frontier and continue
            if next.cost < *distances.get(&next.position).unwrap() {
                // Relaxation, we have now found a better way
                distances.insert(next.position.clone(), next.cost);
                heap.push(next);
            }
        }
    }

    distances
}

fn best_case_release(
    rooms: &HashMap<String, Room>,
    distances: &mut HashMap<String, HashMap<String, usize>>,
    current: String,
    targets: HashSet<String>,
    time_remaining: usize,
) -> usize {
    if targets.is_empty() || time_remaining == 0 {
        return 0;
    }

    let current_distances = distances
        .entry(current.clone())
        .or_insert_with(|| distances_from(current.clone(), rooms))
        .clone();

    targets
        .iter()
        .map(|target| {
            // Time to walk to the target and turn it on
            let time_needed = current_distances.get(target).unwrap() + 1;
            if time_needed > time_remaining {
                return 0;
            }

            let new_time_remaining = time_remaining - time_needed;

            let mut new_targets = targets.clone();
            new_targets.remove(target);

            let strength = rooms.get(target).unwrap().flow_rate * new_time_remaining;

            strength
                + best_case_release(
                    rooms,
                    distances,
                    target.clone(),
                    new_targets,
                    new_time_remaining,
                )
        })
        .max()
        .unwrap_or(0)
}

// Warning: slow
fn best_case_release_2(
    rooms: &HashMap<String, Room>,
    distances: &mut HashMap<String, HashMap<String, usize>>,
    targets: HashSet<String>,
    mut actors: Vec<(String, usize)>,
) -> usize {
    if targets.is_empty() || actors.iter().all(|(_, time)| *time == 0) {
        return 0;
    }

    actors.sort_by_key(|a| a.1);
    actors.reverse();

    actors
        .iter()
        .enumerate()
        .find_map(|(actor_index, (current, time_remaining))| {
            let current_distances = distances
                .entry(current.clone())
                .or_insert_with(|| distances_from(current.clone(), rooms))
                .clone();

            targets
                .iter()
                .filter_map(|target| {
                    // Time to walk to the target and turn it on
                    let time_needed = current_distances.get(target).unwrap() + 1;
                    if time_needed > *time_remaining {
                        return None;
                    }

                    let new_time_remaining = time_remaining - time_needed;

                    let mut new_targets = targets.clone();
                    new_targets.remove(target);

                    let strength = rooms.get(target).unwrap().flow_rate * new_time_remaining;

                    let mut new_actors = actors.clone();

                    new_actors[actor_index] = (target.clone(), new_time_remaining);

                    Some(strength + best_case_release_2(rooms, distances, new_targets, new_actors))
                })
                .max()
        })
        .unwrap_or(0)
}

fn solutions(rooms: &HashMap<String, Room>) -> (usize, usize) {
    let targets: HashSet<String> = rooms
        .values()
        .filter_map(|r| (r.flow_rate > 0).then_some(&r.name))
        .cloned()
        .collect();
    // Distance from one node to another doesn't change between part 1 and part 2
    let mut distances = HashMap::new();
    (
        best_case_release(rooms, &mut distances, "AA".to_owned(), targets.clone(), 30),
        best_case_release_2(
            rooms,
            &mut distances,
            targets,
            vec![("AA".to_owned(), 26), ("AA".to_owned(), 26)],
        ),
    )
}

fn main() -> Result<(), Error> {
    let (part1, part2) = solutions(&parse_valve_list(&read_stdin()?)?);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn examples() -> Result<(), Error> {
        let rooms = parse_valve_list(SAMPLE)?;
        assert_eq!(solutions(&rooms), (1651, 1707));
        Ok(())
    }
}
