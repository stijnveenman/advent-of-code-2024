use std::collections::{HashMap, HashSet};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};

advent_of_code::solution!(6);

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
