use std::collections::HashMap;

use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(19);

fn find_match(
    pattern: &str,
    options: &HashMap<char, Vec<&str>>,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(current) = cache.get(&pattern.len()) {
        return *current;
    }

    let Some(c) = pattern.chars().next() else {
        return 1;
    };

    let Some(option) = options.get(&c) else {
        cache.insert(pattern.len(), 0);
        return 0;
    };

    let value = option
        .iter()
        .filter(|o| pattern.starts_with(*o))
        .map(|o| {
            if pattern.len() == o.len() {
                return 1;
            }

            find_match(&pattern[o.len()..], options, cache)
        })
        .sum();

    cache.insert(pattern.len(), value);
    value
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
        .filter(|p| find_match(p, &options, &mut HashMap::new()) > 0)
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (options, patterns) = input.split_once("\n\n").unwrap();
    let options = options
        .split(",")
        .map(|p| p.trim())
        .grouped_by(|c| c.chars().next().unwrap());

    let patterns = patterns.lines().collect_vec();

    let result = patterns
        .into_iter()
        .map(|p| find_match(p, &options, &mut HashMap::new()))
        .sum();

    Some(result)
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
        assert_eq!(result, Some(16));
    }
}
