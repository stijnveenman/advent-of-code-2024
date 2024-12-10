use std::collections::HashSet;

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(10);

fn score_trailhead(grid: &CharGrid, start: &Point) -> usize {
    let mut open = Vec::from([(*start, 0)]);
    let mut target_set = HashSet::new();

    while let Some((current, height)) = open.pop() {
        let next = current
            .neighbours()
            .into_iter()
            .filter(|n| {
                grid.get(n)
                    .is_some_and(|c| c.to_digit(10).is_some_and(|c| c == height + 1))
            })
            .map(|p| (p, height + 1));

        if height + 1 == 9 {
            target_set.extend(next.into_iter().map(|n| n.0));
        } else {
            open.extend(next);
        }
    }

    target_set.len()
}

fn score_distinct(grid: &CharGrid, start: &Point) -> usize {
    let mut open = Vec::from([(*start, 0)]);
    let mut score = 0usize;

    while let Some((current, height)) = open.pop() {
        let next = current
            .neighbours()
            .into_iter()
            .filter(|n| {
                grid.get(n)
                    .is_some_and(|c| c.to_digit(10).is_some_and(|c| c == height + 1))
            })
            .map(|p| (p, height + 1));

        if height + 1 == 9 {
            score += next.count();
        } else {
            open.extend(next);
        }
    }

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let starting_points = grid
        .entries()
        .filter_map(|(p, c)| if c == '0' { Some(p) } else { None })
        .collect_vec();

    let result = starting_points
        .iter()
        .map(|start| score_trailhead(&grid, start) as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let starting_points = grid
        .entries()
        .filter_map(|(p, c)| if c == '0' { Some(p) } else { None })
        .collect_vec();

    let result = starting_points
        .iter()
        .map(|start| score_distinct(&grid, start) as u32)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
