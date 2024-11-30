use std::collections::{BTreeSet, HashMap};

use crate::{components::Point, grid::Grid};

#[derive(PartialEq, Eq, Debug)]
struct PointWithDistance(usize, Point);

impl PartialOrd for PointWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointWithDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Performs Dijkstra on a `Grid<T>` starting from `start` and trying to reach `end`
///
/// `cost_fn` is a fucntion that gets a `point` and `T` and returns an `Option<usize>`. It should
/// return the cost of visiting the point specified. If `None` is returned, the point cannot be
/// visited
///
/// Algoritm overview
///
/// 1 - Create an empty closed set, and the open ordered queue with the start point
/// 2 - Take the point with the lowest distance so far from the open queue
/// 3 - If the current point is present in the closed set, with a lower distance, skip it
/// 4 - For all unvisited neighbours, calculate the new total distance and add them to the open set
/// 5 - Mark the current node as closed with its distance
/// 6 - Check if the curret node is the end
///
/// @see https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
pub fn dijkstra<'a, T: Grid<'a>, CostFn: Fn(&Point, Option<T::ReturnItem>) -> Option<usize>>(
    grid: &'a T,
    start: Point,
    end: Point,
    cost_fn: CostFn,
) -> Option<usize> {
    // 1
    let mut closed = HashMap::new();
    let mut open = BTreeSet::from([PointWithDistance(0, start)]);

    // 2
    while let Some(PointWithDistance(distance, current)) = open.pop_last() {
        // 3
        if closed
            .get(&current)
            .is_some_and(|prev_distance| *prev_distance < distance)
        {
            continue;
        }

        // 4
        let next = current
            .neighbours()
            .into_iter()
            .filter(|point| grid.in_bounds(point))
            .filter(|point| !closed.contains_key(point))
            .filter_map(|point| {
                Some(PointWithDistance(
                    distance + cost_fn(&point, grid.get(&point))?,
                    point,
                ))
            });

        // 5
        open.extend(next);
        closed.insert(current, distance);

        // 6
        if current == end {
            return Some(distance);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    use crate::{components::Point, grid::char_grid::CharGrid};

    #[rstest]
    fn should_work_on_basic_maze() {
        let input = "
.....
----.
.....
.---.
.....
";
        let grid = CharGrid::new(input).unwrap();

        let result = dijkstra(
            &grid,
            Point::new(0, 0),
            grid.bounds().1,
            |_point, char| match char {
                None => None,
                Some('.') => Some(1),
                Some(_) => None,
            },
        );

        assert_eq!(result, Some(8));
    }
}
