use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|result| {
            let (_, [left, right]) = result.extract();
            let left = left.parse::<u32>().expect("failed to parse left");
            let right = right.parse::<u32>().expect("failed to parse left");
            left * right
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul|don't|do)(\(\d*,?\d*\))").unwrap();

    let mut enabled = true;
    let mut sum = 0u32;
    re.captures_iter(input).for_each(|result| {
        let (_, [action, args]) = result.extract();
        match action {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" if enabled => {
                if !args.contains(",") {
                    return;
                }
                let args = &args[1..args.len() - 1];
                let (left, right) = args.split_once(",").unwrap();
                let left = left.parse::<u32>().expect("failed to parse left");
                let right = right.parse::<u32>().expect("failed to parse left");
                sum += left * right;
            }
            "mul" => {}
            _ => panic!("unknown function {}{}", action, args),
        }
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
