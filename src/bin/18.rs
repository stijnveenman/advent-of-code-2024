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

    grid.print(|_p, c| match c {
        Some(true) => "#".into(),
        _ => ".".into(),
    });

    dijkstra(&grid, Point::new(0, 0), size, |_p, v| match v {
        Some(true) => None,
        _ => Some(1),
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part1(input, Point::new(70, 70), 1024)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
