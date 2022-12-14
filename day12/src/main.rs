//! Day 12

use std::collections::HashMap;

use point_2d::Point2D;
use util::*;

struct HeightMap {
    start: Point2D<isize>,
    end: Point2D<isize>,
    map: HashMap<Point2D<isize>, u8>,
    edges: Vec<(Point2D<isize>, Point2D<isize>)>,
}

fn parse(input: &str) -> Result<HeightMap, Error> {
    let mut start: Point2D<isize> = Point2D(0, 0);
    let mut end: Point2D<isize> = Point2D(0, 0);
    let pairs: Vec<Vec<(Point2D<isize>, u8)>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut row: Vec<(Point2D<isize>, u8)> = vec![];
            for (x, pt) in line.bytes().enumerate() {
                let ix: isize = x.try_into()?;
                let iy: isize = y.try_into()?;
                row.push((
                    Point2D(ix, iy),
                    match pt {
                        b'S' => {
                            start = Point2D(ix, iy);
                            b'a'
                        }
                        b'E' => {
                            end = Point2D(ix, iy);
                            b'z'
                        }
                        other => other,
                    },
                ));
            }
            Ok(row)
        })
        .collect::<Result<_, Error>>()?;

    let map: HashMap<Point2D<isize>, u8> = pairs.into_iter().flatten().collect();

    let directions: Vec<Point2D<isize>> =
        vec![Point2D(1, 0), Point2D(-1, 0), Point2D(0, -1), Point2D(0, 1)];

    let edges: Vec<(Point2D<isize>, Point2D<isize>)> = map
        .iter()
        .flat_map(|(src, height)| {
            directions
                .iter()
                .map(|d| *src + *d)
                .filter(|neighbor| map.get(neighbor).filter(|&&nh| nh <= *height + 1).is_some())
                .map(|neighbor| (neighbor, *src))
        })
        .collect();

    Ok(HeightMap {
        start,
        end,
        map,
        edges,
    })
}

fn solutions(map: &HeightMap) -> (Option<usize>, Option<usize>) {
    // Simplified Bellman-Ford (no negative weights)

    let mut distances: HashMap<Point2D<isize>, usize> = HashMap::new();
    distances.insert(map.end, 0);

    for _ in 0..map.map.len() {
        for (src, dest) in map.edges.iter() {
            if let Some(src_dist) = distances.get(src) {
                let next_dist = src_dist + 1;
                if distances
                    .get(dest)
                    .map_or(true, |&dest_dist| next_dist < dest_dist)
                {
                    distances.insert(*dest, next_dist);
                }
            }
        }
    }

    let part1 = distances.get(&map.start).copied();

    // Bellman-Ford finds the distance to a source from every vertex, so we can look up
    // distances if we use the end as the source
    let part2 = map
        .map
        .iter()
        .filter(|(_, h)| **h == b'a')
        .filter_map(|(pt, _)| distances.get(pt))
        .min()
        .copied();

    (part1, part2)
}

fn main() -> Result<(), Error> {
    let map = parse(&read_stdin()?)?;
    let (result1, result2) = solutions(&map);
    println!("Part 1: {}", result1.context("No path found")?);
    println!("Part 2: {}", result2.context("No path found")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn examples() -> Result<(), Error> {
        let (result1, result2) = solutions(&parse(SAMPLE)?);
        assert_eq!(result1, Some(31));
        assert_eq!(result2, Some(29));
        Ok(())
    }
}
