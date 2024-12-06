use rayon::prelude::*;
use std::collections::HashSet;

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;

advent_of_code::solution!(@impl 6, [part_one, 1] [part_two, 2] [part_twob, 2]);

fn find_loop(grid: &CharGrid, mut current: Point, mut direction: Point, obstacle: &Point) -> bool {
    if !grid.in_bounds(obstacle) {
        return false;
    }

    let mut visited = HashSet::new();

    while grid.in_bounds(&current) {
        if visited.contains(&(current, direction)) {
            return true;
        }

        visited.insert((current, direction));

        if grid.get(&(current + direction)).is_some_and(|c| c == '#')
            || (current + direction) == *obstacle
        {
            direction = direction.rotate_right();
            continue;
        }

        current += direction;
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let mut current = grid
        .entries()
        .find(|(_, c)| *c == '^')
        .map(|(p, _)| p)
        .unwrap();

    let mut visited = HashSet::new();
    let mut direction = Point::UP;

    while grid.in_bounds(&current) {
        visited.insert(current);

        if grid.get(&(current + direction)).is_some_and(|c| c == '#') {
            direction = direction.rotate_right();
            continue;
        }

        current += direction;
    }

    Some(visited.len() as u32)
}

pub fn part_twob(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let mut current = grid
        .entries()
        .find(|(_, c)| *c == '^')
        .map(|(p, _)| p)
        .unwrap();

    let mut visited = HashSet::from([current]);
    let mut direction = Point::UP;

    let mut candidates = vec![];

    while grid.in_bounds(&current) {
        if grid.get(&(current + direction)).is_some_and(|c| c == '#') {
            direction = direction.rotate_right();
            continue;
        }

        visited.insert(current);

        if !visited.contains(&(current + direction)) {
            candidates.push((current, direction));
        }

        current += direction;
    }

    let result = candidates
        .par_iter()
        .filter(|(current, direction)| {
            find_loop(&grid, *current, *direction, &(*current + *direction))
        })
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let start = grid
        .entries()
        .find(|(_, c)| *c == '^')
        .map(|(p, _)| p)
        .unwrap();

    let result = grid
        .entries()
        .filter(|(_point, c)| *c != '^' && *c != '#')
        .collect_vec();

    let result = result
        .par_iter()
        .filter(|(point, _c)| find_loop(&grid, start, Point::UP, point))
        .count();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
