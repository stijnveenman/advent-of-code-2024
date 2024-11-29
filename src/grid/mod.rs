pub mod char_grid;
pub mod hash_grid;

use crate::components::Point;

pub trait Grid<'a> {
    /// The type of Items stored within the grid.
    type Item;

    /// Returns the bounds of the grid
    fn bounds(&self) -> (Point, Point);
    /// Checks wether a point falls within the bounds of the grid. Differs per grid implementation
    fn in_bounds(&self, point: &Point) -> bool;
    /// Gets the item for a certain point, should return None if the point falls outside of the
    /// bounds. Can return None if the point is within the bounds depending on the grid
    fn get(&self, point: &Point) -> Option<Self::Item>;
    /// Sets a value for a certain in the grid. May panic if a point falls out of the bounds and a
    /// grid implementation does not allow for autoscaling.
    fn set(&mut self, point: &Point, value: Self::Item);

    /// Returns all points of the grid. Depending on the grid implementation this may or may not
    /// cover the full bounds of the grid
    fn keys(&self) -> impl Iterator<Item = Point>;
    /// Returns all values stored in the grid
    fn values(&'a self) -> impl Iterator<Item = Self::Item>;
    /// Returns a tuple of the point and value for the entire grid
    fn entries(&'a self) -> impl Iterator<Item = (Point, Self::Item)>;
}
