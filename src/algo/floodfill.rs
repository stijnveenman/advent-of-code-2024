use std::collections::HashSet;

use crate::{components::Point, grid::Grid};

/// Floodfils a `Grid<T>` starting from the `Point` specified. `check_fn` is a function passed a
/// `Point` and value `T`, and returns wether the point can be visited.
/// Only direct neighbours of each point are visited, diagonal points are not visited so that it
/// it can't skip over diagonal walls.
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
            .filter(|point| grid.in_bounds(point))
            .filter(|point| !visited.contains(point))
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

    #[rstest]
    fn should_go_around_corners_but_not_diagonal() {
        let input = "
-----
-   -
--- -
-   -
- ---
- ---
-- --
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
            HashSet::from([
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(3, 2),
                Point::new(3, 3),
                Point::new(2, 3),
                Point::new(1, 3),
                Point::new(1, 4),
                Point::new(1, 5),
            ])
        )
    }

    #[rstest]
    fn should_work_in_open_spaces() {
        let input = "
-----
-   -
-   -
-   -
-   -
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
            HashSet::from([
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
                Point::new(1, 3),
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(1, 4),
                Point::new(2, 4),
                Point::new(3, 4),
            ])
        )
    }
}
