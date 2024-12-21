use std::collections::HashMap;

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};

advent_of_code::solution!(20);

fn find_path(grid: &CharGrid) -> HashMap<Point, usize> {
    let mut current = grid.find(|v| *v == 'S').unwrap();
    let end = grid.find(|v| *v == 'E').unwrap();

    let mut index = 1;
    let mut m = HashMap::new();
    m.insert(current, 0);

    while current != end {
        let next = current
            .neighbours()
            .into_iter()
            .filter(|m| grid.get(m).is_some_and(|c| c != '#'))
            .find(|p| !m.contains_key(p))
            .unwrap();

        m.insert(next, index);
        current = next;
        index += 1;
    }

    m
}

fn solve_one(input: &str, min_cheat: usize) -> Option<usize> {
    let grid = CharGrid::new(input);
    let path = find_path(&grid);

    let cheats = path
        .iter()
        .flat_map(|(point, current_pos)| {
            Point::DIRECTIONS_4
                .iter()
                .filter_map(|dir| path.get(&(*point + (*dir * 2))))
                .filter(|cheat_pos| *cheat_pos > current_pos)
                .map(|cheat_pos| *cheat_pos - *current_pos - 2)
        })
        .filter(|cheat| *cheat >= min_cheat)
        .count();

    Some(cheats)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_one(input, 100)
}

fn solve_two(input: &str, min_cheat: usize) -> Option<usize> {
    let grid = CharGrid::new(input);
    let path = find_path(&grid);

    let cheats = path
        .iter()
        .flat_map(|(start, start_pos)| {
            path.iter()
                .filter(|(end, _)| start.distance(end) <= 20)
                .filter(move |(_, end_pos)| end_pos > &start_pos)
                .map(move |(end, end_pos)| end_pos - start_pos - start.distance(end))
        })
        .filter(|cheat| *cheat >= min_cheat)
        .count();

    Some(cheats)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_two(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 20);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = solve_two(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(285));
    }
}
