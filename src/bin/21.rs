use core::panic;
use std::fs::create_dir;

use advent_of_code::{
    algo::dijkstra::dijkstra,
    components::Point,
    grid::{hash_grid::HashGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(21);

type Pad = HashGrid<'static, char>;
fn numpad() -> Pad {
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

fn direction_keypad() -> Pad {
    let mut grid = HashGrid::new();

    grid.set(&Point::new(1, 0), '^');
    grid.set(&Point::new(2, 0), 'A');

    grid.set(&Point::new(0, 1), '<');
    grid.set(&Point::new(1, 1), 'V');
    grid.set(&Point::new(2, 1), '>');

    grid
}

fn move_pad(pad: &Pad, from: char, to: char) -> Vec<char> {
    let start = pad.find_by_value(&from).unwrap();
    let end = pad.find_by_value(&to).unwrap();
    let path = dijkstra(pad, start, end, |_, _| Some(1)).unwrap();

    let mut path = path
        .iter()
        .zip(path.iter().skip(1))
        .map(|(current, next)| {
            let dir = next.1 - current.1;

            match dir {
                Point::UP => '^',
                Point::DOWN => 'V',
                Point::LEFT => '<',
                Point::RIGHT => '>',

                s => panic!("not a valid dir {}", s),
            }
        })
        .collect_vec();

    //path is the path keypad has to press for each char. it also has to press a after
    path.push('A');
    path
}

fn solve_number(number: &str) {
    let numpad = numpad();
    let keypad = direction_keypad();

    let mut keypad_pos = 'A';
    for c in number.chars() {
        let path = move_pad(&numpad, keypad_pos, c);
        keypad_pos = c;
        dbg!(path);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input.lines().collect_vec();

    let result = numbers
        .into_iter()
        .take(1)
        .map(|number| {
            solve_number(number);
        })
        .collect_vec();

    dbg!(result);
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
