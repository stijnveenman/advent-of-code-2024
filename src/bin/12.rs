use std::collections::{HashMap, HashSet};

use advent_of_code::{
    algo::floodfill::floodfill,
    components::Point,
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

fn count_edges(grid: &CharGrid, fill: &HashSet<Point>) -> usize {
    let mut edges = 0;

    let bounds = grid.bounds().1;
    for y in 0..=(bounds.y + 1) {
        let mut previous = (false, false);
        for x in 0..=bounds.x {
            let upper = fill.contains(&Point::new(x, y - 1));
            let lower = fill.contains(&Point::new(x, y));
            let edge = (upper, lower);

            let is_edge = upper != lower;

            if (previous != edge) && is_edge {
                edges += 1;
            }

            previous = edge;
        }
    }

    for x in 0..=(bounds.x + 1) {
        let mut previous = (false, false);
        for y in 0..=bounds.y {
            let left = fill.contains(&Point::new(x - 1, y));
            let right = fill.contains(&Point::new(x, y));
            let edge = (left, right);

            let is_edge = left != right;

            if (previous != edge) && is_edge {
                edges += 1;
            }

            previous = edge;
        }
    }

    edges
}

pub fn part_two(input: &str) -> Option<usize> {
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
        }

        size_map.insert(region_index, result.len());
        edges_map.insert(region_index, count_edges(&grid, &result));
        region_index += 1;
    }

    let result = edges_map
        .keys()
        .map(|key| edges_map.get(key).unwrap() * size_map.get(key).unwrap())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn edges() {
        let grid = CharGrid::new(
            "XX
XX",
        );

        let result = HashSet::from([
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 0),
            Point::new(0, 1),
        ]);

        assert_eq!(count_edges(&grid, &result), 4);
    }

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
    fn small_example_part2() {
        let result = part_two(
            "AAAA
BBCD
BBCC
EEEC",
        );

        assert_eq!(result, Some(80));
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
    fn containing_example_part2() {
        let result = part_two(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );

        assert_eq!(result, Some(436))
    }

    #[test]
    fn part2_e() {
        let result = part_two(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        );

        assert_eq!(result, Some(236))
    }

    #[test]
    fn part2_double_region() {
        let result = part_two(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );

        assert_eq!(result, Some(368))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
