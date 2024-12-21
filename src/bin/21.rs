use advent_of_code::{
    components::Point,
    grid::{hash_grid::HashGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(21);

fn numpad() -> HashGrid<'static, char> {
    let mut grid = HashGrid::new();

    grid.set(&Point::new(0, 0), '7');
    grid.set(&Point::new(1, 0), '8');
    grid.set(&Point::new(2, 0), '9');

    grid.set(&Point::new(0, 1), '4');
    grid.set(&Point::new(1, 1), '5');
    grid.set(&Point::new(2, 1), '6');

    grid.set(&Point::new(0, 2), '1');
    grid.set(&Point::new(1, 2), '2');
    grid.set(&Point::new(2, 2), '3');

    grid.set(&Point::new(1, 3), '0');
    grid.set(&Point::new(2, 3), 'A');

    grid
}

fn direction_keypad() -> HashGrid<'static, char> {
    let mut grid = HashGrid::new();

    grid.set(&Point::new(1, 0), '^');
    grid.set(&Point::new(2, 0), 'A');

    grid.set(&Point::new(0, 1), '<');
    grid.set(&Point::new(1, 1), 'V');
    grid.set(&Point::new(2, 1), '>');

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input.lines().collect_vec();

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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
