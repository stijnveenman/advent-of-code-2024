use std::{collections::HashMap, marker::PhantomData};

use crate::components::Point;

use super::Grid;

#[derive(PartialEq, Eq)]
pub struct HashGrid<'a, T> {
    grid: HashMap<Point, T>,
    lower: Point,
    upper: Point,

    /// Required to specify the lifetime of a HashGrid, which is required to implement the grid
    /// trait and specify a lifetime of the return Type
    phantom: PhantomData<&'a T>,
}

impl<'a, T> HashGrid<'a, T> {
    pub fn new() -> HashGrid<'a, T> {
        HashGrid {
            grid: HashMap::new(),
            lower: Point::new(0, 0),
            upper: Point::new(0, 0),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Default for HashGrid<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Grid<'a> for HashGrid<'a, T> {
    type Item = &'a T;

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
        self.grid.keys().cloned()
    }

    fn values(&'a self) -> impl Iterator<Item = Self::Item> {
        self.grid.values()
    }

    fn entries(&'a self) -> impl Iterator<Item = (Point, Self::Item)> {
        self.grid.iter().map(|(k, v)| (k.to_owned(), v))
    }
}
