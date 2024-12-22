use core::panic;

use advent_of_code::{
    components::Point,
    grid::{hash_grid::HashGrid, Grid},
};
use itertools::{repeat_n, Itertools};

advent_of_code::solution!(21);

type Pad = HashGrid<'static, char>;
fn numpad(c: char) -> Point {
    match c {
        '7' => Point::new(0, 0),
        '8' => Point::new(1, 0),
        '9' => Point::new(2, 0),

        '4' => Point::new(0, 1),
        '5' => Point::new(1, 1),
        '6' => Point::new(2, 1),

        '1' => Point::new(0, 2),
        '2' => Point::new(1, 2),
        '3' => Point::new(2, 2),

        '0' => Point::new(1, 3),
        'A' => Point::new(2, 3),

        c => panic!("not handled: {}", c),
    }
}

fn direction_keypad(c: char) -> Point {
    match c {
        '^' => Point::new(1, 0),
        'A' => Point::new(2, 0),

        '<' => Point::new(0, 1),
        'v' => Point::new(1, 1),
        '>' => Point::new(2, 1),

        c => panic!("not handled: {}", c),
    }
}

fn map_pad(c: char, index: usize) -> Point {
    if index == 0 {
        numpad(c)
    } else {
        direction_keypad(c)
    }
}

const ROBOT_COUNT: usize = 3;
fn move_pad(robots: &mut [char; ROBOT_COUNT], index: usize, to: char) -> Vec<char> {
    let from = robots[index];
    if from == to {
        return vec!['A'];
    }

    let current = map_pad(from, index);
    let end = map_pad(to, index);

    let diff = end - current;

    let up = diff.y.min(0);
    let down = diff.y.max(0);
    let left = diff.x.min(0);
    let right = diff.x.max(0);

    let mut path = repeat_n('>', right.unsigned_abs())
        .chain(repeat_n('^', up.unsigned_abs()))
        .chain(repeat_n('v', down.unsigned_abs()))
        .chain(repeat_n('<', left.unsigned_abs()))
        .collect_vec();

    path.push('A');
    robots[index] = to;
    path
}

fn solve_number(number: &str) -> usize {
    let mut robots = ['A'; ROBOT_COUNT];

    let path = number
        .chars()
        .flat_map(|c| {
            let path = move_pad(&mut robots, 0, c);

            let path = path
                .into_iter()
                .flat_map(|c| move_pad(&mut robots, 1, c))
                .collect_vec();

            let path = path
                .into_iter()
                .flat_map(|c| move_pad(&mut robots, 2, c))
                .collect_vec();
            // dbg!(&path);
            path
        })
        .collect_vec();

    let a = path.iter().join("");
    println!("{}: {} - {}", number, a, a.len());

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
