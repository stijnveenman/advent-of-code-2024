use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
};

use itertools::Itertools;
use petgraph::{data::Build, dot::Dot, Graph};
use rustc_hash::FxHashMap;

advent_of_code::solution!(24);

type Values<'a> = FxHashMap<&'a str, usize>;
type Connections<'a> = Vec<(&'a str, &'a str, &'a str, &'a str)>;
type MyGraph<'a> = FxHashMap<&'a str, Node<'a>>;

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

fn build_graph<'a>(connections: &'a Connections) -> MyGraph<'a> {
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

fn solve_node<'a>(node: &str, graph: &mut MyGraph<'a>, values: &'a Values) -> usize {
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

fn solve_graph<'a>(graph: &mut MyGraph<'a>, values: &'a Values) -> usize {
    let mut result = 0;
    for i in 0..64 {
        let node = format!("z{:0>2}", i);
        if !graph.contains_key(node.as_str()) {
            break;
        }

        let node_result = solve_node(&node, graph, values);

        result |= node_result << i;
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let (values, connections) = parse(input);

    let mut graph = build_graph(&connections);

    let result = solve_graph(&mut graph, &values);
    Some(result)
}

fn build_num(values: &Values, start: char) -> usize {
    let mut result = 0;
    for i in 0..64 {
        let num = format!("{}{:0>2}", start, i);

        let Some(value) = values.get(num.as_str()) else {
            break;
        };

        result |= value << i;
    }
    result
}

fn connected_nodes<'a>(node: &str, graph: &MyGraph<'a>, set: &mut HashSet<&'a str>) {
    let mut open = vec![node];
    if set.contains(node) {
        return;
    }
    set.insert(graph.get_key_value(node).unwrap().0);

    while let Some(current) = open.pop() {
        if current.starts_with('x') || current.starts_with('y') {
            continue;
        }
        let node = graph.get(current).unwrap();

        if !set.contains(node.left) {
            set.insert(node.left);
            open.push(node.left);
        }

        if !set.contains(node.right) {
            set.insert(node.right);
            open.push(node.right);
        }
    }
}

#[allow(dead_code)]
fn wrong_connected_nodes<'a>(graph: &MyGraph<'a>, diff: usize) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    let len = format!("{diff:b}").len();

    for i in 0..len {
        if ((diff >> i) & 0b1) == 0 {
            continue;
        }

        let node = format!("z{i:0>2}");
        connected_nodes(node.as_str(), graph, &mut set);
    }

    set
}

fn render_graph(graph: &MyGraph) {
    let mut deps = Graph::<&str, &str>::new();

    let mut indexes = HashMap::new();

    for (from, node) in graph.iter() {
        let from_index = *indexes.entry(from).or_insert_with(|| deps.add_node(from));

        let left_index = *indexes
            .entry(&node.left)
            .or_insert_with(|| deps.add_node(node.left));

        let right_index = *indexes
            .entry(&node.right)
            .or_insert_with(|| deps.add_node(node.right));

        deps.add_edge(from_index, left_index, node.gate);
        deps.add_edge(from_index, right_index, node.gate);
    }

    let dot = Dot::with_config(&deps, &[]);

    let mut file = File::create("graph2.dot").unwrap();
    file.write_all(dot.to_string().as_bytes()).unwrap();
}

fn switch<'a>(a: &'a str, b: &'a str, graph: &mut MyGraph<'a>) {
    let a_node = graph.remove(a).unwrap();
    let b_node = graph.remove(b).unwrap();

    graph.insert(a, b_node);
    graph.insert(b, a_node);
}

fn first_one(a: usize) -> Option<usize> {
    (0..64).find(|&i| (a >> i) & 0b1 == 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (values, connections) = parse(input);

    let mut graph = build_graph(&connections);

    switch("z10", "ggn", &mut graph);

    let x = build_num(&values, 'x');
    let y = build_num(&values, 'y');

    let result = solve_graph(&mut graph, &values);

    println!("{:064b}", x + y);
    println!("{:064b}", result);
    println!("{:064b}", result ^ (x + y));

    println!("first mismatch: {:?}", first_one(result ^ (x + y)));

    render_graph(&graph);

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
