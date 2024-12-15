use core::panic;
use std::usize;

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, hash_grid::HashGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(15);

fn parse(input: &str) -> (Point, HashGrid<'_, char>, Vec<Point>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = CharGrid::new(grid);

    let start = grid
        .entries()
        .find(|(_p, v)| *v == '@')
        .map(|(p, _v)| p)
        .unwrap();

    let grid = HashGrid::from_chargrid(grid, |c| match c {
        'O' => Some('O'),
        '#' => Some('#'),
        _ => None,
    });

    let moves = moves
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '<' => Point::LEFT,
            '>' => Point::RIGHT,
            'v' => Point::DOWN,
            '^' => Point::UP,
            c => panic!("unknown {}", c),
        })
        .collect_vec();

    (start, grid, moves)
}

fn try_move(point: &Point, dir: &Point, grid: &mut HashGrid<'_, char>) -> bool {
    if !grid.get(point).is_some_and(|c| *c == 'O') {
        return false;
    }

    let next = *point + *dir;

    if grid.contains(&next) && !try_move(&next, dir, grid) {
        return false;
    }

    let c = grid.remove(point).unwrap();
    grid.set(&next, c);

    true
}

fn try_move_2w(point: &Point, dir: &Point, grid: &mut HashGrid<'_, char>) -> bool {
    if grid.get(point).is_some_and(|c| *c == '#') {
        dbg!("wall", point);
        return false;
    }

    if grid.get(point).is_none() {
        dbg!("isnone");
        return true;
    }

    // let b = grid.get(point).unwrap();

    if dir == &Point::RIGHT || dir == &Point::LEFT {
        // horizontal move

        let next = *point + (*dir * 2);

        if !try_move_2w(&next, dir, grid) {
            return false;
        }

        let c = grid.remove(&(*point + (*dir))).unwrap();
        grid.set(&(*point + *dir * 2), c);

        let c = grid.remove(point).unwrap();
        grid.set(&(*point + *dir), c);
    } else {
        // vertical move

        todo!()
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut pos, mut grid, moves) = parse(input);

    for dir in moves {
        let next = pos + dir;

        if grid.contains(&next) {
            if try_move(&next, &dir, &mut grid) {
                pos = next;
            }
        } else {
            pos = next;
        }
    }

    // grid.print(|_p, c| match c {
    //     Some(c) => c.to_string(),
    //     None => " ".to_string(),
    // });

    let result = grid
        .entries()
        .filter_map(|(p, c)| match c {
            'O' => Some(p),
            _ => None,
        })
        .map(|p| (p.y * 100 + p.x) as usize)
        .sum();

    Some(result)
}

fn expand(grid: HashGrid<'_, char>) -> HashGrid<'_, char> {
    let mut new_grid = HashGrid::new();

    for (point, c) in grid.entries() {
        match c {
            'O' => {
                new_grid.set(&Point::new(point.x * 2, point.y), '[');
                new_grid.set(&Point::new(point.x * 2 + 1, point.y), ']');
            }
            '#' => {
                new_grid.set(&Point::new(point.x * 2, point.y), '#');
                new_grid.set(&Point::new(point.x * 2 + 1, point.y), '#');
            }
            c => panic!("{} not handled", c),
        }
    }

    new_grid
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pos, grid, moves) = parse(input);
    let mut grid = expand(grid);
    let mut pos = Point::new(pos.x * 2, pos.y);

    grid.print(|p, c| {
        if *p == pos {
            return "@".into();
        }

        match c {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        }
    });

    for dir in moves {
        let next = pos + dir;

        if grid.contains(&next) {
            if try_move_2w(&next, &dir, &mut grid) {
                pos = next;
            }
        } else {
            pos = next;
        }

        grid.print(|p, c| {
            if *p == pos {
                return "@".into();
            }

            match c {
                Some(c) => c.to_string(),
                None => " ".to_string(),
            }
        });
        println!();
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_example() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let result = part_two(input);

        // Incorrect
        assert_eq!(result, Some(105));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
