use std::collections::HashSet;

use crate::{components::Point, grid::Grid};

pub fn floodfill<'a, T: Grid<'a>, F: Fn(&Point, Option<T::ReturnItem>) -> bool>(
    grid: &'a T,
    start: Point,
    check_fn: F,
) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut remaining: Vec<Point> = vec![start];

    while let Some(current) = remaining.pop() {
        visited.insert(current);

        let next = current
            .neighbours()
            .into_iter()
            .filter(|point| !visited.contains(point))
            .filter(|point| grid.in_bounds(point))
            .filter(|point| check_fn(point, grid.get(point)));

        remaining.extend(next);
    }

    visited
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    use crate::{components::Point, grid::char_grid::CharGrid};

    #[rstest]
    fn should_do_a_basic_floodfill() {
        let input = "
-----
-   -
-----
";
        let grid = CharGrid::new(input).unwrap();

        let result = floodfill(&grid, Point::new(1, 1), |_point, char| match char {
            None => true,
            Some('-') => false,
            Some(' ') => true,
            Some(_) => false,
        });

        assert_eq!(
            result,
            HashSet::from([Point::new(1, 1), Point::new(2, 1), Point::new(3, 1)])
        )
    }
}
