use advent_of_code::AocItertools;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(11);

fn num_length(mut num: usize) -> usize {
    let mut len = 0;

    while num > 0 {
        num /= 10;
        len += 1;
    }

    len
}

fn take_top(mut num: usize, len: usize) -> usize {
    for _ in 0..len {
        num /= 10;
    }

    num
}

fn take_bottom(num: usize, len: usize) -> usize {
    let modulo = 10usize.pow(len as u32);

    num % modulo
}

fn count_stones(stone: usize, depth: usize, max_depth: usize) -> usize {
    if depth >= max_depth {
        return 1;
    }

    if stone == 0 {
        return count_stones(1, depth + 1, max_depth);
    }

    let num_len = num_length(stone);
    if num_len % 2 == 0 {
        count_stones(take_top(stone, num_len / 2), depth + 1, max_depth)
            + count_stones(take_bottom(stone, num_len / 2), depth + 1, max_depth)
    } else {
        count_stones(stone * 2024, depth + 1, max_depth)
    }
}

fn solve(input: &str, max_depth: usize) -> Option<usize> {
    let stones = input.trim().split(" ").usize().collect_vec();

    let result = stones
        .par_iter()
        .map(|stone| count_stones(*stone, 0, max_depth))
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 25)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
