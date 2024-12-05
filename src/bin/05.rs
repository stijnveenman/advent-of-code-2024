use std::cmp::Ordering;

use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();

            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect_vec();

    let updates = updates
        .lines()
        .map(|l| l.split(",").u32().collect_vec())
        .filter(|update| {
            let iter = update.iter().zip(update.iter().skip(1));

            for (left, right) in iter {
                if !ordering.iter().any(|ord| *ord == (*left, *right)) {
                    return false;
                }
            }

            true
        })
        .collect_vec();

    let result = updates
        .iter()
        .map(|v| {
            let middle = (v.len() - 1) / 2;
            v.get(middle).unwrap()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();

            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect_vec();

    let mut updates = updates
        .lines()
        .map(|l| l.split(",").u32().collect_vec())
        .filter(|update| {
            let iter = update.iter().zip(update.iter().skip(1));

            for (left, right) in iter {
                if !ordering.iter().any(|ord| *ord == (*left, *right)) {
                    return true;
                }
            }

            false
        })
        .collect_vec();

    for update in updates.iter_mut() {
        update.sort_by(|a, b| {
            let s = ordering
                .iter()
                .find(|ord| **ord == (*a, *b) || **ord == (*b, *a))
                .unwrap();

            match s {
                _ord if *s == (*a, *b) => Ordering::Less,
                _ord if *s == (*b, *a) => Ordering::Greater,
                _ => panic!("{}{}, {:?}", a, b, s),
            }
        });
    }

    let result = updates
        .iter()
        .map(|v| {
            let middle = (v.len() - 1) / 2;
            v.get(middle).unwrap()
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
