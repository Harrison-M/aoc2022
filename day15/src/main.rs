//! Day 15

use std::{collections::HashSet, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt, recognize},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use point_2d::Point2D;
use util::*;

/// Parse a signed integer with no separators
fn parse_signed_int(input: &str) -> IResult<&str, isize, VerboseError<&str>> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), |num: &str| {
        num.parse()
    })(input)
}

/// Represents a sensor
struct Sensor {
    /// The position of the sensor
    position: Point2D<isize>,
    /// The distance from the sensor the detected beacon was
    range: isize,
}

/// A field of deployed sensors and the beacons they detected
struct Field {
    /// The furthest left any sensor can reach
    min_x: isize,
    /// The furthest right any sensor can reach
    max_x: isize,
    /// The deployed sensors
    sensors: Vec<Sensor>,
    /// The positions of every beacon a sensor has detected
    beacons: HashSet<Point2D<isize>>,
}

/// Gets the four numbers from a sensor specification
fn parse_line(input: &str) -> IResult<&str, (isize, isize, isize, isize), VerboseError<&str>> {
    tuple((
        preceded(tag("Sensor at x="), parse_signed_int),
        preceded(tag(", y="), parse_signed_int),
        preceded(tag(": closest beacon is at x="), parse_signed_int),
        preceded(tag(", y="), parse_signed_int),
    ))(input)
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = separated_list1(newline, parse_line)(s)
            .finish()
            .map_err(|e| anyhow!("{}", e))?
            .1;

        let mut beacons: HashSet<Point2D<isize>> = HashSet::new();
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;

        let sensors = points
            .into_iter()
            .map(|(sx, sy, bx, by)| {
                let position = Point2D(sx, sy);
                let beacon = Point2D(bx, by);
                let range = position.manhattan_distance(&beacon);

                beacons.insert(beacon);

                let reach_left = sx - range + 1;
                let reach_right = sx + range - 1;

                min_x = min_x.min(reach_left);
                max_x = max_x.max(reach_right);

                Sensor { position, range }
            })
            .collect();

        Ok(Field {
            min_x,
            max_x,
            sensors,
            beacons,
        })
    }
}

impl Sensor {
    fn just_outside(&self) -> impl Iterator<Item = Point2D<isize>> {
        let radius = self.range + 1;
        let Point2D(x, y) = self.position;
        let ne_x = x..=(x + radius);
        let ne_y = (y - radius)..=y;
        let se_x = ne_x.clone().rev();
        let se_y = y..=(y + radius);
        let nw_x = (x - radius)..=x;
        let nw_y = ne_y.clone().rev();
        let sw_x = nw_x.clone();
        let sw_y = se_y.clone();

        // We're going to duplicate the corners but I'm counting that as an acceptable loss
        ne_x.zip(ne_y)
            .chain(se_x.zip(se_y))
            .chain(nw_x.zip(nw_y))
            .chain(sw_x.zip(sw_y))
            .map(|(x, y)| Point2D(x, y))
    }
}

impl Field {
    /// Find how many non-beacon points are within any sensor's range in the given row
    fn part1(&self, row: isize) -> usize {
        (self.min_x..=self.max_x)
            .filter(|x| {
                let point = Point2D(*x, row);
                if self.beacons.contains(&point) {
                    return false;
                }

                self.sensors
                    .iter()
                    .any(|s| s.position.manhattan_distance(&point) <= s.range)
            })
            .count()
    }

    /// Assuming there is only one such point, find the only point out of any sensor's range where
    /// 0 <= x <= max and 0 <= y <= max
    fn part2(&self, max: isize) -> isize {
        self.sensors
            .iter()
            .flat_map(Sensor::just_outside)
            .filter(|&Point2D(x, y)| x >= 0 && y >= 0 && x <= max && y <= max)
            .find(|point| {
                self.sensors
                    .iter()
                    .all(|s| s.position.manhattan_distance(point) > s.range)
            })
            .map_or(0, |Point2D(x, y)| x * 4000000 + y)
    }
}

fn main() -> Result<(), Error> {
    let field: Field = read_stdin()?.parse()?;
    println!("Part 1: {}", field.part1(2000000));
    println!("Part 2: {}", field.part2(4000000));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn examples() -> Result<(), Error> {
        let field: Field = SAMPLE.parse()?;
        assert_eq!(field.part1(10), 26);
        assert_eq!(field.part2(20), 56000011);
        Ok(())
    }
}
