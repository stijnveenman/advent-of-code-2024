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

            let words = Point::DIRECTIONS_8
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
    let grid = CharGrid::new(input);

    let result = grid
        .keys()
        .filter_map(|a_point| {
            if grid.get(&a_point).is_none_or(|c| c != 'A') {
                return None;
            }

            let lt = grid.get(&(a_point + Point::UP_LEFT))?;
            if lt != 'M' && lt != 'S' {
                return None;
            }

            let rt = grid.get(&(a_point + Point::UP_RIGHT))?;
            if rt != 'M' && rt != 'S' {
                return None;
            }

            let lb = grid.get(&(a_point + Point::DOWN_LEFT))?;
            if lb != 'M' && lb != 'S' {
                return None;
            }

            let rb = grid.get(&(a_point + Point::DOWN_RIGHT))?;
            if rb != 'M' && rb != 'S' {
                return None;
            }

            if lt == lb && rt == rb && lt != rt {
                return Some(a_point);
            }

            if lt == rt && lb == rb && lt != lb {
                return Some(a_point);
            }

            None
        })
        .collect_vec();

    Some(result.len() as u32)
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
        assert_eq!(result, Some(9));
    }
}
