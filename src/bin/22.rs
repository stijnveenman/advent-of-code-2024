use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(22);

fn mix(value: isize, number: isize) -> isize {
    value ^ number
}

fn prune(number: isize) -> isize {
    number % 16777216
}

fn evolve(mut number: isize) -> isize {
    number = mix(number * 64, number);
    number = prune(number);

    number = mix(number / 32, number);
    number = prune(number);

    number = mix(number * 2048, number);
    number = prune(number);

    number
}

pub fn part_one(input: &str) -> Option<isize> {
    let input = input.lines().usize().map(|v| v as isize).collect_vec();

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

fn price(number: isize) -> isize {
    number % 10
}

pub fn part_two(input: &str) -> Option<isize> {
    let input = input.lines().usize().map(|v| v as isize).collect_vec();

    let result = input
        .into_iter()
        .map(|secret| {
            let mut changes = [0isize; 2000];

            let mut current = secret;
            let mut previous_price = price(secret);
            (0..2000).for_each(|i| {
                current = evolve(current);

                let price = price(current);

                changes[i] = price - previous_price;
                previous_price = price;

                if i < 4 {
                    return;
                }

                let sequence = &changes[i - 4..i];
                dbg!(sequence);
            });

            current
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
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
