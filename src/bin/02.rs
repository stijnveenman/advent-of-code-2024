use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(2);

fn check_safe(report: &[u32]) -> bool {
    let mut iter = report.iter().zip(report.iter().skip(1)).peekable();
    let order = iter
        .peek()
        .map(|(a, b)| a.cmp(b))
        .expect("failed to get first item");

    for (a, b) in iter {
        if a.abs_diff(*b) > 3 {
            return false;
        }

        match a.cmp(b) {
            Ordering::Equal => return false,
            current_order if current_order != order => return false,
            _ => continue,
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<u32>().expect("failed to parse level"))
                .collect_vec()
        })
        .collect_vec();

    let safe = input.into_iter().filter(|report| check_safe(report));

    Some(safe.count() as u32)
}

fn filter_i(input: &[u32], i: usize) -> Vec<u32> {
    input
        .iter()
        .enumerate()
        .filter(|(index, _v)| *index != i)
        .map(|(_i, v)| v)
        .cloned()
        .collect_vec()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<u32>().expect("failed to parse level"))
                .collect_vec()
        })
        .collect_vec();

    let safe = input.into_iter().filter(|report| {
        if check_safe(report) {
            return true;
        }

        for i in 0..report.len() {
            let report = filter_i(report, i);
            if check_safe(&report) {
                return true;
            }
        }

        false
    });

    Some(safe.count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
