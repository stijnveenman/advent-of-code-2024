use core::panic;
use std::{collections::HashMap, iter::repeat_n};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
    AocItertools,
};
use itertools::Itertools;

advent_of_code::solution!(21);

static NUMPAD: &str = "789
456
123
X0A";

static DIRPAD: &str = "X^A
<v>";

type Moveset = HashMap<(char, char), Vec<String>>;

fn map_path(path: &[Point]) -> String {
    path.iter()
        .zip(path.iter().skip(1))
        .map(|(from, to)| {
            let dir = *to - *from;
            match dir {
                Point::RIGHT => '>',
                Point::LEFT => '<',
                Point::UP => '^',
                Point::DOWN => 'v',

                a => panic!("unhandled: {}", a),
            }
        })
        .chain(repeat_n('A', 1))
        .join("")
}

fn calculate_paths(grid: &str) -> Moveset {
    let grid = CharGrid::new(grid);
    let mut map = HashMap::new();

    for (from_point, from_char) in grid.entries() {
        for (to_point, to_char) in grid.entries() {
            if to_char == 'X' || from_char == 'X' {
                continue;
            }

            if from_char == to_char {
                map.insert((from_char, to_char), vec!["A".into()]);
                continue;
            }

            let mut open = vec![vec![from_point]];
            while let Some(current) = open.pop() {
                let last_point = current.last().unwrap();
                if last_point == &to_point {
                    // map.entry((from_char, to_char), map_path(&current));

                    map.entry((from_char, to_char))
                        .or_default()
                        .push(map_path(&current));
                    continue;
                }

                let distance = last_point.distance(&to_point);

                let next = last_point
                    .neighbours()
                    .into_iter()
                    .filter(|p| grid.get(p).is_some_and(|c| c != 'X'))
                    .filter(|p| p.distance(&to_point) < distance)
                    .map(|n| {
                        let mut v = current.clone();
                        v.push(n);
                        v
                    });

                open.extend(next);
            }
        }
    }

    map
}

fn map_number(input: &str, moveset: &Moveset) -> Vec<String> {
    // We start at A

    repeat_n('A', 1)
        .chain(input.chars())
        .zip(input.chars())
        .map(|m| moveset.get(&m).unwrap())
        .multi_cartesian_product()
        .map(|i| i.iter().join(""))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = input.lines().collect_vec();

    let numpad = calculate_paths(NUMPAD);
    let dirpad = calculate_paths(DIRPAD);

    numbers
        .iter()
        .map(|n| map_number(n, &numpad))
        .dbg()
        .collect_vec();

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
