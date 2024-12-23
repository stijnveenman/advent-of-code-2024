use std::collections::HashSet;

use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(23);

fn find_set<'a>(
    entry: &'a str,
    connections: &FxHashMap<&'a str, Vec<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let entries = connections.get(entry).unwrap();

    let mut set = HashSet::new();
    for next in entries {
        let next_entry = connections.get(next).unwrap();

        let sets = next_entry
            .iter()
            .filter(|c| **c != entry)
            .filter(|c| entries.contains(c))
            .map(|c| {
                let mut v = vec![entry, next, c];
                v.sort();
                v
            });

        set.extend(sets);
    }

    set
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect_vec();

    let mut connections: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    input.into_iter().for_each(|(left, right)| {
        connections.entry(left).or_default().push(right);
        connections.entry(right).or_default().push(left);
    });

    let mut set = HashSet::new();
    for entry in connections.keys().unique() {
        set.extend(find_set(entry, &connections));
    }

    let result = set
        .iter()
        .filter(|v| v.iter().any(|s| s.starts_with("t")))
        .count();

    Some(result)
}

fn is_fully_connected(set: &[&str], connections: &FxHashMap<&str, Vec<&str>>) -> bool {
    set.iter().all(|from| {
        let connections = connections.get(*from).unwrap();

        set.iter().all(|s| s == from || connections.contains(s))
    })
}

fn find_largest_fully_connected_sets<'a>(
    entry: &'a str,
    connections: &FxHashMap<&'a str, Vec<&'a str>>,
    min_length: usize,
) -> Option<Vec<&'a str>> {
    let check_set = connections.get(entry).unwrap();

    (min_length..check_set.len() + 1).rev().find_map(|i| {
        check_set
            .clone()
            .into_iter()
            .chain([entry])
            .combinations(i)
            .find(|p| is_fully_connected(p, connections))
    })
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect_vec();

    let mut connections: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    input.into_iter().for_each(|(left, right)| {
        connections.entry(left).or_default().push(right);
        connections.entry(right).or_default().push(left);
    });

    let mut keys = connections.keys().unique();
    let mut largest =
        find_largest_fully_connected_sets(keys.next().unwrap(), &connections, 3).unwrap();

    for key in keys {
        if let Some(current) = find_largest_fully_connected_sets(key, &connections, largest.len()) {
            if current.len() > largest.len() {
                largest = current;
            }
        }
    }

    largest.sort();

    Some(largest.iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".into()));
    }
}
