use core::panic;

use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(24);

type Values<'a> = FxHashMap<&'a str, usize>;
type Connections<'a> = Vec<(&'a str, &'a str, &'a str, &'a str)>;
type Graph<'a> = FxHashMap<&'a str, Node<'a>>;

fn parse(input: &str) -> (Values, Connections) {
    let (values, connections) = input.split_once("\n\n").unwrap();
    let values = values
        .lines()
        .map(|n| {
            let (input, n) = n.split_once(": ").unwrap();
            (input, n.parse::<usize>().unwrap())
        })
        .collect::<FxHashMap<&str, usize>>();

    let connections = connections
        .lines()
        .map(|n| {
            let mut iter = n.split(" ");

            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            let c = iter.next().unwrap();
            iter.next().unwrap();
            let d = iter.next().unwrap();

            (a, b, c, d)
        })
        .collect_vec();

    (values, connections)
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
    gate: &'a str,
    result: Option<usize>,
}

fn build_graph<'a>(connections: &'a Connections) -> Graph<'a> {
    let mut graph: FxHashMap<&str, Node> = FxHashMap::default();

    for c in connections {
        graph.insert(
            c.3,
            Node {
                left: c.0,
                right: c.2,
                gate: c.1,
                result: None,
            },
        );
    }

    graph
}

fn solve_node<'a>(node: &'a str, graph: &mut Graph<'a>, values: &'a Values) -> usize {
    if node.starts_with("x") || node.starts_with("y") {
        return *values.get(node).unwrap();
    }

    let current = *graph.get(node).unwrap();
    if let Some(result) = current.result {
        return result;
    }

    let left = solve_node(current.left, graph, values);
    let right = solve_node(current.right, graph, values);

    let result = match current.gate {
        "OR" => {
            if left == 1 || right == 1 {
                1
            } else {
                0
            }
        }
        "AND" => {
            if left == 1 && right == 1 {
                1
            } else {
                0
            }
        }
        "XOR" => {
            if left != right {
                1
            } else {
                0
            }
        }
        s => panic!("gate {} unknown", s),
    };

    let current = graph.get_mut(node).unwrap();
    current.result = Some(result);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (values, connections) = parse(input);

    let mut graph = build_graph(&connections);

    let result = solve_node("z07", &mut graph, &values);
    dbg!(result);

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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
