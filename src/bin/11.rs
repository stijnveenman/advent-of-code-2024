use advent_of_code::AocItertools;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, MaxLen, ParallelIterator};
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
    cache: &mut FxHashMap<usize, ([usize; MAX], usize)>,
) -> usize {
    if depth >= max_depth {
        return 1;
    }

    if stone == 0 {
        let result = count_stones(1, depth + 1, max_depth, result_set, cache);
        insert(stone, depth, max_depth, result_set, cache);
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
    insert(stone, depth, max_depth, result_set, cache);
    value
}

fn solve(input: &str, max_depth: usize) -> Option<usize> {
    let stones = input.trim().split(" ").usize().collect_vec();

    let result = stones
        .par_iter()
        .map(|stone| {
            let mut result_set = [0; MAX];
            let mut cache = FxHashMap::default();

            let v = count_stones(*stone, 0, max_depth, &mut result_set, &mut cache);
            dbg!(stone, v, cache.get(stone));
            v
        })
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 75)
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

    #[test]
    fn extra_example() {
        let result = solve("125 17", 6);
        assert_eq!(result, Some(22));
    }
}
