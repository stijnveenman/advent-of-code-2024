use advent_of_code::components::Point;
use itertools::Itertools;

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

    fn cost(&self) -> Option<usize> {
        //You estimate that each button would need to be pressed no more than 100 times to win a prize
        for i in 0..=100 {
            let a_total = self.a * i;
            let remaining = self.prize - a_total;

            let x_rem = remaining.x % self.b.x;
            let y_rem = remaining.y % self.b.y;
            if x_rem != 0 || y_rem != 0 {
                continue;
            }
            let x_div = remaining.x / self.b.x;
            let y_div = remaining.y / self.b.y;

            if x_div == y_div {
                return Some(((i * 3) + x_div) as usize);
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = input.split("\n\n").map(Machine::from_string).collect_vec();

    dbg!(&machines);

    let result = machines.iter().filter_map(|m| m.cost()).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
