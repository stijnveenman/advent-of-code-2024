use cached::proc_macro::cached;
use core::panic;
use std::{collections::HashMap, iter::repeat_n};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, Grid},
};
use itertools::Itertools;
use lazy_static::lazy_static;

advent_of_code::solution!(21);

static NUMPAD: &str = "789
456
123
X0A";

static DIRPAD: &str = "X^A
<v>";

lazy_static! {
    static ref NUMPAD_PATHS: Moveset = calculate_paths(NUMPAD);
    static ref DIRPAD_PATHS: Moveset = calculate_paths(DIRPAD);
}

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

#[cached]
fn solve_length(input: String, depth_remaining: usize) -> usize {
    let pairs = ['A'].into_iter().chain(input.chars()).zip(input.chars());

    if depth_remaining == 1 {
        return pairs
            .map(|pair| DIRPAD_PATHS.get(&pair).unwrap().first().unwrap().len())
            .sum();
    }

    pairs
        .map(|pair| {
            DIRPAD_PATHS
                .get(&pair)
                .unwrap()
                .iter()
                .map(|subseq| solve_length(subseq.clone(), depth_remaining - 1))
                .min()
                .unwrap()
        })
        .sum()
}

fn solve(input: &str, depth: usize) -> Option<usize> {
    let numbers = input.lines().collect_vec();

    let result = numbers
        .into_iter()
        .map(|number| {
            let number_parsed = number[..number.len() - 1].parse::<usize>().unwrap();
            map_number(number, &NUMPAD_PATHS)
                .iter()
                .map(|n| {
                    let length = solve_length(n.clone(), depth);
                    length * number_parsed
                })
                .min()
                .unwrap()
        })
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 25)
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
        assert_eq!(result, Some(154115708116294));
    }
}
