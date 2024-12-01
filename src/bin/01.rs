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
    None
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
        assert_eq!(result, None);
    }
}
