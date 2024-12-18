use advent_of_code::{
    algo::dijkstra::dijkstra,
    components::Point,
    grid::{hash_grid::HashGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(18);

fn solve_part1(input: &str, size: Point, take: usize) -> Option<usize> {
    let input = input
        .lines()
        .map(|l| Point::parse_seperated(l, ",").unwrap())
        .take(take)
        .collect_vec();

    let mut grid = HashGrid::with_bounds(Point::new(0, 0), size);

    for p in &input {
        grid.set(p, true);
    }

    // grid.print(|_p, c| match c {
    //     Some(true) => "#".into(),
    //     _ => ".".into(),
    // });

    dijkstra(&grid, Point::new(0, 0), size, |_p, v| match v {
        Some(true) => None,
        _ => Some(1),
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part1(input, Point::new(70, 70), 1024)
}

fn try_for(points: &[Point], take: usize) -> bool {
    let mut grid = HashGrid::with_bounds(Point::new(0, 0), Point::new(70, 70));

    for p in points.iter().take(take) {
        grid.set(p, true);
    }

    dijkstra(&grid, Point::new(0, 0), grid.bounds().1, |_p, v| match v {
        Some(true) => None,
        _ => Some(1),
    })
    .is_some()
}

pub fn part_two(input: &str) -> Option<String> {
    let points = input
        .lines()
        .map(|l| Point::parse_seperated(l, ",").unwrap())
        .collect_vec();

    // at the beginning, we know 1024 (input) works, and the end shouldn't work
    let mut min = 1024;
    let mut max = points.len();

    // Binary search the first point that fails
    while min + 1 < max {
        let current = min + (max - min) / 2;

        if try_for(&points, current) {
            // if it succeeds, take the right
            min = current;
        } else {
            // if it fails, take the left
            max = current;
        }
    }

    Some(points[min].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part1(
            &advent_of_code::template::read_file("examples", DAY),
            Point::new(6, 6),
            12,
        );
        assert_eq!(result, Some(22));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
