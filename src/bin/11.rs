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

#[allow(dead_code)]
fn insert(
    stone: usize,
    depth: usize,
    max_depth: usize,
    result_set: &mut [usize; MAX],
    cache: &mut FxHashMap<usize, ([usize; MAX], usize)>,
) {
    let cache_len = max_depth - depth;
    if cache.get(&stone).is_some_and(|(_, len)| *len > cache_len) {
        return;
    }

    let mut entry = [0; MAX];
    let mut counter = 1;
    (0..cache_len).for_each(|i| {
        counter += result_set[depth + i];
        entry[i] = counter;
    });

    cache.insert(stone, (entry, cache_len));
}

#[allow(dead_code)]
fn get(
    stone: usize,
    depth: usize,
    max_depth: usize,
    cache: &mut FxHashMap<usize, ([usize; MAX], usize)>,
) -> Option<usize> {
    let (entry, cache_len) = cache.get(&stone)?;
    let steps_remaining = max_depth - depth;

    if steps_remaining > *cache_len {
        return None;
    }

    dbg!(
        "hit",
        &entry[..*cache_len],
        depth,
        max_depth,
        steps_remaining
    );

    dbg!(stone);

    Some(entry[steps_remaining - 1])
}

fn count_stones(
    stone: usize,
    depth: usize,
    max_depth: usize,
    result_set: &mut [usize; MAX],
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if depth >= max_depth {
        return 1;
    }

    if depth + 1 < MAX {
        result_set[depth + 1] = 0;
    }

    if let Some(c) = cache.get(&(stone, depth)) {
        return *c;
    }

    if stone == 0 {
        let result = count_stones(1, depth + 1, max_depth, result_set, cache);
        cache.insert((stone, depth), result);
        return result;
    }

    let num_len = num_length(stone);
    let value = if num_len % 2 == 0 {
        result_set[depth] += 1;
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
    cache.insert((stone, depth), value);
    value
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
    solve(input, 75)
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
        assert_eq!(result, Some(65601038650482));
    }

    #[test]
    fn extra_example() {
        let result = solve("125 17", 6);
        assert_eq!(result, Some(22));
    }
}
