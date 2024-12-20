use advent_of_code::grid::{char_grid::CharGrid, Grid};

advent_of_code::solution!(20);

fn solve_one(input: &str, min_cheat: usize) -> Option<u32> {
    let grid = CharGrid::new(input);
    let start = grid.find_by_value(|v| *v == 'S').unwrap();
    let end = grid.find_by_value(|v| *v == 'E').unwrap();

    dbg!(start, end);

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_one(input, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 20);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
