use std::fmt::Debug;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::components::Point;

use super::Grid;

#[derive(PartialEq, Eq)]
pub struct CharGrid {
    lines: Vec<String>,
}

impl CharGrid {
    /// Creates a new CharGrid from a string, splitting and trimming the lines
    /// Errors if the input is empty
    /// Errors if any of the lines differs in length
    pub fn new(input: &str) -> Result<CharGrid> {
        let lines: Vec<String> = input
            .trim()
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        let line_len = lines
            .first()
            .map(|f| f.len())
            .ok_or(anyhow!("empty vector"))?;

        if line_len == 0 {
            return Err(anyhow!("line length cannot be empty"));
        }

        if lines.iter().any(|f| f.len() != line_len) {
            return Err(anyhow!("all lines must be of equal width"));
        }

        Ok(CharGrid { lines })
    }
}

impl<'a> Grid<'a> for CharGrid {
    type ReturnItem = char;
    type SetItem = char;

    fn bounds(&self) -> (Point, Point) {
        let line_len = self.lines.first().map(|f| f.len()).unwrap() - 1;
        let height = self.lines.len() - 1;

        (
            Point::new(0, 0),
            Point::new(line_len as isize, height as isize),
        )
    }

    fn get(&'a self, point: &Point) -> Option<Self::ReturnItem> {
        self.lines
            .get(usize::try_from(point.y).unwrap())
            .and_then(|line| line.chars().nth(usize::try_from(point.x).unwrap()))
    }

    fn set(&mut self, point: &Point, value: Self::SetItem) {
        let line = self.lines.get(usize::try_from(point.y).unwrap()).unwrap();
        let line = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if x as isize == point.x {
                    return value;
                }

                c
            })
            .collect::<String>();

        self.lines[usize::try_from(point.y).unwrap()] = line;
    }

    fn keys(&self) -> impl Iterator<Item = Point> {
        self.lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, _c)| Point::new(x as isize, y as isize))
        })
    }

    fn values(&'a self) -> impl Iterator<Item = Self::ReturnItem> {
        self.lines.iter().flat_map(|line| line.chars())
    }

    fn entries(&'a self) -> impl Iterator<Item = (Point, Self::ReturnItem)> {
        self.lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as isize, y as isize), c))
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn new_should_return_none_when_empty() {
        let input = "";

        assert!(CharGrid::new(input).is_err());
    }

    #[rstest]
    fn new_should_fail_when_lines_are_not_equal_len() {
        let input = "|...|
||..|
|||
|||.|
|||||";

        assert!(CharGrid::new(input).is_err());
    }

    #[rstest]
    fn should_trim_correctly() {
        let input = "  |...|
||..|
|||.|
|||.|
|||||

";

        let grid = CharGrid::new(input).unwrap();

        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(4, 4)));
    }

    #[rstest]
    fn basic_chargrid_happy_flow() {
        let input = "|...|
||..|
|||.|
|||.|
|||||";

        let mut grid = CharGrid::new(input).unwrap();

        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(4, 4)));

        assert!(grid.in_bounds(&Point::new(3, 3)));
        // A 5x5 grid starts at 0,0, so 5,5 itself it out of bounds
        assert!(!grid.in_bounds(&Point::new(5, 5)));
        assert!(!grid.in_bounds(&Point::new(6, 3)));

        assert_eq!(grid.get(&Point::new(0, 0)), Some('|'));
        assert_eq!(grid.get(&Point::new(0, 2)), Some('|'));
        assert_eq!(grid.get(&Point::new(0, 6)), None);
        assert_eq!(grid.get(&Point::new(1, 0)), Some('.'));
        assert_eq!(grid.get(&Point::new(2, 2)), Some('|'));
        assert_eq!(grid.get(&Point::new(4, 4)), Some('|'));

        assert_eq!(grid.get(&Point::new(1, 2)), Some('|'));
        assert_eq!(grid.get(&Point::new(2, 2)), Some('|'));
        assert_eq!(grid.get(&Point::new(3, 2)), Some('.'));
        grid.set(&Point::new(2, 2), 'a');
        assert_eq!(grid.get(&Point::new(1, 2)), Some('|'));
        assert_eq!(grid.get(&Point::new(2, 2)), Some('a'));
        assert_eq!(grid.get(&Point::new(3, 2)), Some('.'));
    }

    #[rstest]
    fn char_grid_can_debug() {
        let input = "|...|
||..|
|||.|
|||.|
|||||";

        let grid = CharGrid::new(input).unwrap();

        let result = format!("{:?}", grid);

        assert_eq!(input, result);
    }
}

impl Debug for CharGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.iter().join("\n"))
    }
}
