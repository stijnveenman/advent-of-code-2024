use std::collections::{BTreeSet, HashMap};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, Debug)]
// Distance, Location, Direction
// MARKER improve dijkstra with point T
struct SearchPoint(usize, Point, Point);

impl PartialOrd for SearchPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            // Returning equal will have the BTreeSet assume they are equal
            // We don't care which distance comes first if the distance is Equal
            // So pick a random point by returning less. unless the points are actually equal
            std::cmp::Ordering::Equal if self.1 == other.1 => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
            other => other,
        }
    }
}

pub fn dijkstra(grid: &CharGrid, start: Point, end: Point) -> Option<usize> {
    let mut closed = HashMap::new();
    let mut open = BTreeSet::from([SearchPoint(0, start, Point::RIGHT)]);

    while let Some(SearchPoint(distance, current, direction)) = open.pop_first() {
        if closed
            .get(&current)
            .is_some_and(|prev_distance| *prev_distance < distance)
        {
            continue;
        }

        let next = [
            SearchPoint(distance + 1, current + direction, direction),
            SearchPoint(
                distance + 1001, // Rotate and move
                current + direction.rotate_right(),
                direction.rotate_right(),
            ),
            SearchPoint(
                distance + 1001, // Rotate and move
                current + direction.rotate_left(),
                direction.rotate_left(),
            ),
        ]
        .into_iter()
        .filter(|point| grid.in_bounds(&point.1))
        .filter(|point| grid.get(&point.1).is_some_and(|c| c != '#'))
        .filter(|point| !closed.contains_key(&point.1));

        open.extend(next);
        closed.insert(current, distance);

        if current == end {
            return Some(distance);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = CharGrid::new(input);
    // MARKER special grid find points as we do this a lot
    let start = grid.entries().find(|(p, c)| c == &'S').unwrap().0;
    let end = grid.entries().find(|(p, c)| c == &'E').unwrap().0;

    dijkstra(&grid, start, end)
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
