use advent_of_code::components::Point;
use itertools::Itertools;

advent_of_code::solution!(14);

fn solve_part_one(input: &str, size: Point) -> Option<u32> {
    let robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (_, v) = v.split_once("=").unwrap();

            (
                Point::parse_seperated(p, ","),
                Point::parse_seperated(v, ","),
            )
        })
        .collect_vec();

    dbg!(robots);

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input, Point::new(101, 103))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(
            &advent_of_code::template::read_file("examples", DAY),
            Point::new(11, 7),
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
