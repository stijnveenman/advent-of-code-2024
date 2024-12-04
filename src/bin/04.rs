use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = CharGrid::new(input);

    let result = grid
        .keys()
        .flat_map(|x_point| {
            if grid.get(&x_point).is_none_or(|c| c != 'X') {
                return None;
            }

            let words = Point::DIRECTIONS
                .iter()
                .filter_map(|direction| {
                    if grid
                        .get(&(x_point + (*direction * 1isize)))
                        .is_none_or(|c| c != 'M')
                    {
                        return None;
                    }
                    if grid
                        .get(&(x_point + (*direction * 2isize)))
                        .is_none_or(|c| c != 'A')
                    {
                        return None;
                    }
                    if grid
                        .get(&(x_point + (*direction * 3isize)))
                        .is_none_or(|c| c != 'S')
                    {
                        return None;
                    }

                    Some((x_point, direction))
                })
                .collect_vec();

            Some(words)
        })
        .flatten()
        .collect_vec();

    Some(result.len() as u32)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
