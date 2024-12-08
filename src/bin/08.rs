use std::{collections::HashSet, usize};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
    AocItertools,
};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let mut groups = grid.entries().grouped_by(|(_p, c)| *c);
    groups.remove(&'.');

    let mut set = HashSet::new();

    for (_, v) in groups {
        for v in v.iter().combinations(2) {
            let (p1, _) = v.first().unwrap();
            let (p2, _) = v.get(1).unwrap();

            let b1 = *p1 + (*p1 - *p2);
            let b2 = *p2 + (*p2 - *p1);

            if grid.in_bounds(&b1) {
                set.insert(b1);
            }
            if grid.in_bounds(&b2) {
                set.insert(b2);
            }
        }
    }

    Some(set.len() as u32)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
