use std::usize;

use itertools::{repeat_n, Itertools};

advent_of_code::solution!(9);

// This is a super inefficient way
fn expand(input: &str) -> Vec<u32> {
    let mut file_index = 0u32;
    let mut is_file = true;
    // Assume each file is on average 5 items wide.
    let mut v = Vec::with_capacity(input.len() * 5);

    for c in input.chars() {
        let count = c.to_digit(10).unwrap();

        let element = match is_file {
            false => {
                is_file = true;
                u32::MAX
            }
            true => {
                is_file = false;
                let current_index = file_index;
                file_index += 1;
                current_index
            }
        };

        v.extend(repeat_n(element, count as usize));
    }

    v
}

// Returns the new size
fn compress(input: &mut [u32]) -> usize {
    let mut head = 0usize;
    let mut tail = input.len() - 1;

    while head < tail {
        if input[head] != u32::MAX {
            head += 1;
            continue;
        }

        while input[tail] == u32::MAX {
            tail -= 1;
        }

        input[head] = input[tail];

        head += 1;
        tail -= 1;
    }

    head + 1
}

fn checksum(input: &[u32]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(a, b)| a * (*b as usize))
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = expand(input.trim());
    let len = compress(&mut input);

    // dbg!(&input[..len].iter().map(|c| format!("{}", c)).join(""));

    Some(checksum(&input[..len]))
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
