use std::collections::HashMap;

use advent_of_code::{
    algo::floodfill::floodfill,
    grid::{char_grid::CharGrid, Grid},
};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = CharGrid::new(input);

    let mut region_index = 0usize;
    let mut region_map = HashMap::new();

    let mut edges_map = HashMap::new();
    let mut size_map = HashMap::new();

    for point in grid.keys() {
        if region_map.contains_key(&point) {
            continue;
        }

        let ch = grid.get(&point).unwrap();
        let result = floodfill(&grid, point, |_p, c| ch == c.unwrap());

        for point in &result {
            region_map.insert(*point, region_index);

            let edge_count = point
                .neighbours()
                .iter()
                .filter(|p| !result.contains(p))
                .count();

            let entry = edges_map.entry(region_index).or_insert(0usize);
            *entry += edge_count;

            size_map.insert(region_index, result.len());
        }

        region_index += 1;
    }

    let result = edges_map
        .keys()
        .map(|key| edges_map.get(key).unwrap() * size_map.get(key).unwrap())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_example() {
        let result = part_one(
            "AAAA
BBCD
BBCC
EEEC",
        );

        assert_eq!(result, Some(140));
    }

    #[test]
    fn containing_example() {
        let result = part_one(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
