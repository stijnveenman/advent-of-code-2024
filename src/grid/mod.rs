pub mod char_grid;
pub mod hash_grid;

use crate::components::Point;

pub trait Grid<'a> {
    /// The type of Items when returned from the grid
    type ReturnItem;
    /// The type of Item used when setting a value
    type SetItem;

    /// Returns the bounds of the grid
    fn bounds(&self) -> (Point, Point);
    /// Checks wether a point falls within the bounds of the grid. Differs per grid implementation
    fn in_bounds(&self, point: &Point) -> bool {
        let bounds = self.bounds();
        point.is_within(&bounds.0, &bounds.1)
    }

    /// Gets the item for a certain point, should return None if the point falls outside of the
    /// bounds. Can return None if the point is within the bounds depending on the grid
    fn get(&'a self, point: &Point) -> Option<Self::ReturnItem>;
    /// Sets a value for a certain in the grid. May panic if a point falls out of the bounds and a
    /// grid implementation does not allow for autoscaling.
    fn set(&mut self, point: &Point, value: Self::SetItem);

    /// Returns all set points of the grid. Depending on the grid implementation this may or may not
    /// cover the full bounds of the grid
    fn keys(&self) -> impl Iterator<Item = Point>;
    /// Returns all values stored in the grid
    fn values(&'a self) -> impl Iterator<Item = Self::ReturnItem>;
    /// Returns a tuple of the point and value for the entire grid
    fn entries(&'a self) -> impl Iterator<Item = (Point, Self::ReturnItem)>;

    /// Draw a visual representation of the grid
    fn draw<DrawFn: Fn(&Point, Option<Self::ReturnItem>) -> String>(
        &'a self,
        draw_fn: DrawFn,
    ) -> String {
        let mut s = String::new();
        let (lower, upper) = self.bounds();

        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let point = Point::new(x, y);
                s += &draw_fn(&point, self.get(&point));
            }

            if y != upper.y {
                s += "\n";
            }
        }

        s
    }
}
