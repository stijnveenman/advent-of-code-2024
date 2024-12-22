use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(22);

fn mix(value: usize, number: usize) -> usize {
    value ^ number
}

fn prune(number: usize) -> usize {
    number % 16777216
}

fn evolve(mut number: usize) -> usize {
    number = mix(number * 64, number);
    number = prune(number);

    number = mix(number / 32, number);
    number = prune(number);

    number = mix(number * 2048, number);
    number = prune(number);

    number
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.lines().usize().collect_vec();

    let result = input
        .into_iter()
        .map(|secret| {
            let mut current = secret;
            for _ in 0..2000 {
                current = evolve(current);
            }

            current
        })
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_mix() {
        let result = mix(42, 15);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_prune() {
        let result = prune(100000000);
        assert_eq!(result, 16113920);
    }
}
