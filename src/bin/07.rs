use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(7);

fn reduce_to_result(target: usize, current: usize, remaining: &[usize]) -> bool {
    if remaining.is_empty() {
        return target == current;
    }

    if current > target {
        return false;
    }

    let next = remaining[0];

    reduce_to_result(target, current + next, &remaining[1..])
        || reduce_to_result(target, current * next, &remaining[1..])
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| {
            let (result, rem) = line.split_once(":").unwrap();

            let result = result.parse::<usize>().unwrap();

            let rem = rem
                .trim()
                .split(" ")
                .u32()
                .map(|v| v as usize)
                .collect_vec();

            (result, rem)
        })
        .collect_vec();

    let result = input
        .iter()
        .filter(|i| reduce_to_result(i.0, *i.1.first().unwrap(), &i.1[1..]))
        .map(|i| i.0)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
