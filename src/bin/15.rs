use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, hash_grid::HashGrid, Grid},
};
use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
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

    dbg!(moves);

    // grid.print(|_p, c| match c {
    //     Some(c) => c.to_string(),
    //     None => " ".to_string(),
    // });

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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
