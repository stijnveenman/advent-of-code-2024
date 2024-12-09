use core::panic;
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

#[derive(Debug, PartialEq, Eq)]
enum Block {
    /// (length of the block)
    Free(u32),
    /// (length of the block, file index)
    Used(u32, u32),
}

fn expand_blocks(input: &str) -> Vec<Block> {
    let mut file_index = 0u32;
    let mut is_file = true;
    let mut v = Vec::new();

    for c in input.chars() {
        let length = c.to_digit(10).unwrap();

        let item = match is_file {
            true => {
                let block = Block::Used(length, file_index);
                file_index += 1;
                block
            }
            false => Block::Free(length),
        };

        is_file = !is_file;

        if length != 0 {
            v.push(item);
        }
    }

    v
}

fn compress_blocks(input: &mut Vec<Block>) {
    let mut tail = input.len() - 1;

    while tail > 0 {
        let (length, idx) = match input[tail] {
            Block::Free(_) => {
                tail -= 1;
                continue;
            }
            Block::Used(length, idx) => (length, idx),
        };

        let Some((free_index, free_block)) = input[..tail].iter().find_position(|i| match i {
            Block::Used(_, _) => false,
            Block::Free(len) => *len >= length,
        }) else {
            tail -= 1;
            continue;
        };

        let Block::Free(free_len) = free_block else {
            panic!("should have free block here");
        };

        input[free_index] = Block::Free(*free_len - length);
        input[tail] = Block::Free(length);

        input.insert(free_index, Block::Used(length, idx));

        unfragment_blocks(input);

        tail -= 1;
    }
}

fn unfragment_blocks(blocks: &mut Vec<Block>) {
    let mut i = 0usize;
    while i < blocks.len() {
        let current_index = i;
        let Block::Free(mut current_len) = blocks[i] else {
            i += 1;
            continue;
        };

        while let Some(Block::Free(free_len)) = blocks.get(i + 1) {
            current_len += free_len;

            blocks[i + 1] = Block::Free(0);
            i += 1;
        }

        blocks[current_index] = Block::Free(current_len);

        i += 1;
    }
}

fn blocks_checksum(blocks: &[Block]) -> usize {
    let mut idx = 0usize;
    let mut checksum = 0usize;

    for block in blocks {
        let (length, block_index) = match block {
            Block::Free(length) => {
                idx += *length as usize;
                continue;
            }
            Block::Used(len, idx) => (len, idx),
        };

        for _ in 0..*length {
            checksum += idx * (*block_index as usize);
            idx += 1;
        }
    }
    checksum
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks = expand_blocks(input.trim());
    compress_blocks(&mut blocks);

    Some(blocks_checksum(&blocks))
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
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn should_unfragment_blocks() {
        let mut blocks = vec![Block::Free(2), Block::Free(0), Block::Free(4)];

        unfragment_blocks(&mut blocks);

        assert_eq!(blocks, vec![Block::Free(6), Block::Free(0), Block::Free(0)])
    }
}
