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

// we could return a [usize; 2] and filter out the second stone if it didn't split
// or we could limit a single map stone to a certain size, if it gets above this sive
// we split resursively into a second call
//
// For now, use a memory inefficient way
fn map_stones(stones: &[usize]) -> Vec<usize> {
    stones
        .iter()
        .flat_map(|stone| match stone {
            0 => vec![1],
            s if num_length(*s) % 2 == 0 => vec![
                take_top(*s, num_length(*s) / 2),
                take_bottom(*s, num_length(*s) / 2),
            ],
            s => vec![*s * 2024], // s * 2024
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones = input.trim().split(" ").usize().collect_vec();

    let result = stones
        .par_iter()
        .map(|stone| {
            let mut stones = vec![*stone];
            for _ in 0..25 {
                stones = map_stones(&stones);
            }

            stones.len()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = input.trim().split(" ").usize().collect_vec();

    let result = stones
        .par_iter()
        .map(|stone| {
            let mut stones = vec![*stone];
            for _ in 0..75 {
                stones = map_stones(&stones);
            }

            stones.len()
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
