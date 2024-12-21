use core::panic;
use std::fs::create_dir;

use advent_of_code::{
    algo::dijkstra::dijkstra,
    components::Point,
    grid::{hash_grid::HashGrid, Grid},
    AocItertools,
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
    if from == to {
        // i think sometimes we might need A here
        return vec!['A'];
    }
    let mut current = pad.find_by_value(&from).unwrap();
    let end = pad.find_by_value(&to).unwrap();
    let mut path = vec![];

    while current != end {
        let distance = current.distance(&end);

        let dir = Point::DIRECTIONS_4
            .into_iter()
            .filter(|d| pad.contains(&(current + *d)))
            .find(|d| {
                let next = current + *d;
                next.distance(&end) < distance
            })
            .unwrap();

        current += dir;

        path.push(match dir {
            Point::UP => '^',
            Point::DOWN => 'V',
            Point::RIGHT => '>',
            Point::LEFT => '<',
            _ => panic!(),
        });
    }

    path.push('A');
    path
}

struct Robot {
    pos: char,
    pad: Pad,
}

impl Robot {
    fn new(pad: Pad) -> Robot {
        Robot { pos: 'A', pad }
    }

    fn press(&mut self, to: char) -> Vec<char> {
        let path = move_pad(&self.pad, self.pos, to);
        self.pos = to;
        path
    }
}

fn solve_number(number: &str) -> usize {
    let mut r1 = Robot::new(numpad());
    let mut r2 = Robot::new(direction_keypad());
    let mut r3 = Robot::new(direction_keypad());

    let path = number
        .chars()
        .flat_map(|c| {
            let path = r1.press(c);

            let path = path.into_iter().flat_map(|c| r2.press(c)).collect_vec();

            let path = path.into_iter().flat_map(|c| r3.press(c)).collect_vec();
            // dbg!(&path);
            path
        })
        .collect_vec();

    let a = path.iter().join("");
    println!("{}: {}", number, a);

    path.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = input.lines().collect_vec();

    let result = numbers
        .into_iter()
        .map(|number| (solve_number(number), number))
        .map(|(length, number)| number[..number.len() - 1].parse::<usize>().unwrap() * length)
        .sum();

    dbg!(result);

    Some(result)
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
