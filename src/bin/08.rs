use advent_of_code::{
    grid::{char_grid::CharGrid, Grid},
    AocItertools,
};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let mut groups = grid.entries().grouped_by(|(_p, c)| *c);
    groups.remove(&'.');

    dbg!(groups);

    None
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
