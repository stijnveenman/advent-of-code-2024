use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect_vec();

    // MARKER .sum() iterator functions would be nice
    let mut left = input
        .iter()
        .map(|a| a.0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let mut right = input
        .iter()
        .map(|a| a.1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect_vec();

    // MARKER .sum() iterator functions would be nice
    let left = input
        .iter()
        .map(|a| a.0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let right = input
        .iter()
        .map(|a| a.1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    let mut map = HashMap::new();
    for n in right {
        match map.entry(n) {
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
            Entry::Occupied(mut entry) => {
                entry.insert(entry.get() + 1);
            }
        }
    }

    let result = left
        .into_iter()
        .map(|number| {
            let count = map.get(&number).unwrap_or(&0);
            number * count
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
