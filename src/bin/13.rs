use advent_of_code::components::Point;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Machine {
    fn parse_line(line: &str) -> Point {
        let (_, point) = line.split_once(": ").unwrap();

        let point = point
            .replace("X+", "")
            .replace("Y+", "")
            .replace("X=", "")
            .replace("Y=", "");
        Point::parse_seperated(&point, ", ").unwrap()
    }

    fn from_string(input: &str) -> Machine {
        let mut lines = input.lines();
        let a = Self::parse_line(lines.next().unwrap());
        let b = Self::parse_line(lines.next().unwrap());
        let prize = Self::parse_line(lines.next().unwrap());

        Machine { a, b, prize }
    }

    fn cost(&self) -> Option<isize> {
        // let denom = self.a.x * self.b.y - self.a.y * self.b.x;
        let denom = self.a.x * self.b.y - self.b.x * self.a.y;

        // let numerator = self.prize.y * self.a.x - self.a.y * self.prize.x;
        let numerator = self.prize.y * self.a.x - self.prize.x * self.a.y;

        if numerator % denom != 0 {
            // Non integer solution
            return None;
        }

        let b = numerator / denom;

        let a = (self.prize.x - self.b.x * b) / self.a.x;

        //  ¯\_(ツ)_/¯ only needed for 1 specific point
        if self.a.x * a + self.b.x * b != self.prize.x
            || self.a.y * a + self.b.y * b != self.prize.y
        {
            return None;
        }

        Some(a * 3 + b)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let machines = input.split("\n\n").map(Machine::from_string).collect_vec();

    let result = machines.iter().filter_map(|m| m.cost()).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let machines = input
        .split("\n\n")
        .map(Machine::from_string)
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize: Point::new(m.prize.x + 10000000000000, m.prize.y + 10000000000000),
        })
        .collect_vec();

    let result = machines.par_iter().filter_map(|m| m.cost()).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
