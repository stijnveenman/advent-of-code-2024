use std::collections::{HashMap, HashSet};

use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(19);

fn find_match(
    pattern: &str,
    options: &HashMap<char, Vec<&str>>,
    cache: &mut HashSet<usize>,
) -> bool {
    if cache.contains(&pattern.len()) {
        return false;
    }

    let Some(c) = pattern.chars().next() else {
        return true;
    };

    let Some(option) = options.get(&c) else {
        cache.insert(pattern.len());
        return false;
    };

    for o in option.iter().filter(|o| pattern.starts_with(*o)) {
        if pattern.len() == o.len() {
            return true;
        }

        if find_match(&pattern[o.len()..], options, cache) {
            return true;
        }
    }

    cache.insert(pattern.len());
    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (options, patterns) = input.split_once("\n\n").unwrap();
    let options = options
        .split(",")
        .map(|p| p.trim())
        .grouped_by(|c| c.chars().next().unwrap());

    let patterns = patterns.lines().collect_vec();

    let result = patterns
        .into_iter()
        .filter(|p| find_match(p, &options, &mut HashSet::new()))
        .count();

    Some(result)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
