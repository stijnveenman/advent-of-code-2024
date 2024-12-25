use std::collections::HashSet;

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(25);

fn get_set(input: &&str) -> HashSet<Point> {
    let grid = CharGrid::new(input);

    grid.entries()
        .filter(|(_p, c)| *c == '#')
        .map(|a| a.0)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let items = input.split("\n\n").collect_vec();
    let locks = items
        .iter()
        .filter(|i| i.starts_with("#"))
        .map(get_set)
        .collect_vec();
    let keys = items
        .iter()
        .filter(|i| i.starts_with("."))
        .map(get_set)
        .collect_vec();

    let mut total = 0;
    for lock in locks {
        for key in &keys {
            if lock.intersection(key).count() == 0 {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }
}
