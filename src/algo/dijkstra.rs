use std::collections::HashSet;

use crate::{components::Point, grid::Grid};

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
pub fn floodfill<'a, T: Grid<'a>, CostFn: Fn(&Point, Option<T::ReturnItem>) -> Option<usize>>(
    grid: &'a T,
    start: Point,
    end: Point,
    cost_fn: CostFn,
) -> HashSet<Point> {
    todo!()
}
