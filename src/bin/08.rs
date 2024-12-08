use std::collections::HashSet;

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

    let mut set = HashSet::new();

    for (_, v) in groups {
        for v in v.iter().permutations(2) {
            let (p1, _) = v.first().unwrap();
            let (p2, _) = v.get(1).unwrap();

            let b1 = *p1 + (*p1 - *p2);

            if grid.in_bounds(&b1) {
                set.insert(b1);
            }
        }
    }

    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let mut groups = grid.entries().grouped_by(|(_p, c)| *c);
    groups.remove(&'.');

    let mut set = HashSet::new();

    for (_, v) in groups {
        for v in v.iter().permutations(2) {
            let (p1, _) = v.first().unwrap();
            let (p2, _) = v.get(1).unwrap();

            // let b1 = *p1 + (*p1 - *p2);
            let dir = *p1 - *p2;
            set.insert(*p2);

            let mut b = *p1 + dir;
            while grid.in_bounds(&b) {
                set.insert(b);

                b += dir;
            }
        }
    }

    // let g = grid.draw(|p, c| {
    //     if set.contains(p) {
    //         return "#".into();
    //     }
    //     c.unwrap().into()
    // });
    // println!("{}", g);

    Some(set.len() as u32)
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
        assert_eq!(result, Some(34));
    }
}
