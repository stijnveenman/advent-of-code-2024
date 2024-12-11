use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let stones = input.trim().split(" ").usize().collect_vec();

    dbg!(stones);

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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
