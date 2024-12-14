use advent_of_code::{components::Point, AocItertools};
use itertools::Itertools;

advent_of_code::solution!(14);

fn solve_part_one(input: &str, size: Point) -> Option<usize> {
    let robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (_, v) = v.split_once("=").unwrap();

            (
                Point::parse_seperated(p, ",").unwrap(),
                Point::parse_seperated(v, ",").unwrap(),
            )
        })
        .collect_vec();

    let x_split = size.x / 2;
    let y_split = size.y / 2;

    let value = robots
        .iter()
        .map(|(p, v)| {
            let pos = *p + (*v * 100);

            Point::new(pos.x % size.x, pos.y % size.y)
        })
        .map(|p| {
            let x = if p.x < 0 { size.x + p.x } else { p.x };
            let y = if p.y < 0 { size.y + p.y } else { p.y };

            Point::new(x, y)
        })
        .filter(|p| p.x != x_split && p.y != y_split)
        .map(|p| (p.x < x_split, p.y < y_split))
        .grouped_by(|x| *x)
        .values()
        .map(|v| v.len())
        .product();

    Some(value)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(input, Point::new(101, 103))
}

#[allow(dead_code)]
fn draw(upper: Point, v: &[(Point, Point)]) -> String {
    let mut s = String::new();

    for y in 0..=upper.y {
        for x in 0..=upper.x {
            let point = Point::new(x, y);
            let count = v.iter().filter(|(a, _)| a == &point).count();

            if count == 0 {
                s += " ";
            } else {
                s += "█";
            };
        }

        if y != upper.y {
            s += "\n";
        }
    }

    println!("{}\n", s);
    s
}

fn has(v: &[(Point, Point)], p: &Point) -> bool {
    v.iter().any(|(r, _)| r == p)
}

pub fn part_two(input: &str) -> Option<u32> {
    let size = Point::new(101, 103);

    let mut robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (_, v) = v.split_once("=").unwrap();

            (
                Point::parse_seperated(p, ",").unwrap(),
                Point::parse_seperated(v, ",").unwrap(),
            )
        })
        .collect_vec();

    for i in 1..100_000 {
        robots = robots
            .into_iter()
            .map(|(p, v)| {
                let pos = p + (v);

                (Point::new(pos.x % size.x, pos.y % size.y), v)
            })
            .map(|(p, v)| {
                let x = if p.x < 0 { size.x + p.x } else { p.x };
                let y = if p.y < 0 { size.y + p.y } else { p.y };

                (Point::new(x, y), v)
            })
            .collect_vec();

        let has_tree = robots.iter().any(|(p, _)| {
            for y in 1..5 {
                if !(-y..y).all(|x| has(&robots, &Point::new(p.x + x, p.y + y))) {
                    return false;
                }
            }

            true
        });

        if has_tree {
            // draw(size, &robots);
            return Some(i as u32);
        }
    }

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
