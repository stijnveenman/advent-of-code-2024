use std::usize;

use advent_of_code::AocItertools;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

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

const MAX: usize = 75;

fn insert(
    stone: usize,
    depth: usize,
    max_depth: usize,
    result_set: &mut [usize; MAX],
    cache: &mut FxHashMap<usize, [usize; MAX]>,
) {
    cache.entry(stone).or_insert_with(|| [0; MAX]);

    let entry = cache.get_mut(&stone).unwrap();

    (depth..max_depth).for_each(|i| {
        entry[i] = result_set[i];
    });
}

fn get(
    stone: usize,
    depth: usize,
    max_depth: usize,
    cache: &mut FxHashMap<usize, [usize; MAX]>,
) -> Option<usize> {
    let entry = cache.get(&stone)?;

    dbg!(depth, max_depth, &entry[..max_depth]);

    let value = entry[depth];
    if value == 0 {
        None
    } else {
        Some(value)
    }
}

fn count_stones(
    stone: usize,
    depth: usize,
    max_depth: usize,
    result_set: &mut [usize; MAX],
    cache: &mut FxHashMap<usize, [usize; MAX]>,
) -> usize {
    if let Some(c) = get(stone, depth, max_depth, cache) {
        return c;
    }
    if depth >= max_depth {
        result_set[depth] = 1;
        return 1;
    }

    if stone == 0 {
        let result = count_stones(1, depth + 1, max_depth, result_set, cache);
        result_set[depth] = result;
        insert(stone, depth, max_depth, result_set, cache);
        return result;
    }

    let num_len = num_length(stone);
    let result = if num_len % 2 == 0 {
        count_stones(
            take_top(stone, num_len / 2),
            depth + 1,
            max_depth,
            result_set,
            cache,
        ) + count_stones(
            take_bottom(stone, num_len / 2),
            depth + 1,
            max_depth,
            result_set,
            cache,
        )
    } else {
        count_stones(stone * 2024, depth + 1, max_depth, result_set, cache)
    };

    result_set[depth] = result;
    insert(stone, depth, max_depth, result_set, cache);
    result
}

fn solve(input: &str, max_depth: usize) -> Option<usize> {
    let stones = input.trim().split(" ").usize().collect_vec();

    let result = stones
        .par_iter()
        .map(|stone| {
            let mut result_set = [0; MAX];
            let mut cache = FxHashMap::default();
            count_stones(*stone, 0, max_depth, &mut result_set, &mut cache)
        })
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
