use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines().u32().collect_vec();

    dbg!(input);

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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
