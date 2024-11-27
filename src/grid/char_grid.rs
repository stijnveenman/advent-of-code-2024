use crate::components::Point;

use super::Grid;

pub struct CharGrid {
    /// Each vertical line is an entry in the CharGrid
    /// Each horizontal line is a character in the line!()
    /// Each line must be of equal width
    lines: Vec<String>,
}

impl CharGrid {
    /// Creates a new CharGrid from a Vec<String>
    /// returns None if the vec is empty
    /// returns None if any of the lines differs in length
    pub fn new(lines: Vec<String>) -> Option<CharGrid> {
        let line_len = lines.first().map(|f| f.len())?;

        if lines.iter().any(|f| f.len() != line_len) {
            return None;
        }

        Some(CharGrid { lines })
    }
}

impl Grid for CharGrid {
    type Item = char;

    fn bounds(&self) -> (Point, Point) {
        todo!()
    }

    fn in_bounds(&self, point: &Point) -> bool {
        todo!()
    }

    fn get(&self, point: &Point) -> Option<Self::Item> {
        todo!()
    }

    fn set(&mut self, point: &Point, value: Self::Item) {
        todo!()
    }

    fn keys(&self) -> impl Iterator<Item = Point> {
        self.lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, _c)| Point::new(x as isize, y as isize))
        })
    }

    fn values(&self) -> impl Iterator<Item = Self::Item> {
        self.lines.iter().flat_map(|line| line.chars())
    }

    fn entries(&self) -> impl Iterator<Item = (Point, Self::Item)> {
        self.lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as isize, y as isize), c))
        })
    }
}
