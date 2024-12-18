use core::panic;

use advent_of_code::AocItertools;
use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Clone)]
struct Computer {
    opcodes: Vec<usize>,
    pc: usize,
    a: usize,
    b: usize,
    c: usize,
    out: Vec<usize>,
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let (registers, program) = value.split_once("\n\n").unwrap();

        let program = program
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .usize()
            .collect_vec();

        let mut registers = registers
            .lines()
            .map(|l| l.split_once(": ").unwrap().1.parse::<usize>().unwrap());

        Computer {
            opcodes: program,
            pc: 0,
            a: registers.next().unwrap(),
            b: registers.next().unwrap(),
            c: registers.next().unwrap(),
            out: vec![],
        }
    }
}

impl Computer {
    fn output(&self) -> String {
        self.out.iter().map(|u| u.to_string()).join(",")
    }

    fn run(&mut self) {
        while self.pc < self.opcodes.len() {
            let opcode = self.opcodes[self.pc];

            match opcode {
                0 => {
                    self.a /= 2usize.pow(self.combo() as u32);
                    self.pc += 2;
                }
                1 => {
                    self.b ^= self.literal();
                    self.pc += 2;
                }
                2 => {
                    self.b = self.combo() % 8;
                    self.pc += 2;
                }
                3 => {
                    if self.a == 0 {
                        self.pc += 2;
                    } else {
                        self.pc = self.literal();
                    }
                }
                4 => {
                    self.b ^= self.c;
                    self.pc += 2;
                }
                5 => {
                    self.out.push(self.combo() % 8);
                    self.pc += 2;
                }
                6 => {
                    self.b = self.a / 2usize.pow(self.combo() as u32);
                    self.pc += 2;
                }
                7 => {
                    self.c = self.a / 2usize.pow(self.combo() as u32);
                    self.pc += 2;
                }
                a => panic!("unknown opcode {}", a),
            }
        }
    }

    fn combo(&self) -> usize {
        let operand = self.opcodes[self.pc + 1];
        match operand {
            a if a <= 3 => a,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            a => panic!("unknown operand {}", a),
        }
    }

    fn literal(&self) -> usize {
        self.opcodes[self.pc + 1]
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer: Computer = input.into();

    computer.run();

    Some(computer.output())
}

fn compare_correct(actual: &[usize], expected: &[usize]) -> usize {
    assert_eq!(actual.len(), expected.len());

    for i in 0..expected.len() {
        if expected[expected.len() - 1 - i] != actual[actual.len() - 1 - i] {
            return i;
        }
    }

    actual.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let computer: Computer = input.into();

    let mut i = 0;
    let mut prev_length = 0;

    loop {
        let mut computer = computer.clone();
        computer.a = i;

        computer.run();

        // println!("{}", computer.output());
        if computer.out == computer.opcodes {
            return Some(i);
        }

        if prev_length == 0 {
            prev_length = computer.out.len();
        }

        if computer.out.len() != computer.opcodes.len() {
            i *= 2;
        } else {
            let correct = compare_correct(&computer.opcodes, &computer.out);
            let remaining = computer.opcodes.len() - correct;
            // keep taking smaller steps towards the output
            if remaining > 5 {
                i += 10usize.pow(remaining as u32 - 5);
            }
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
",
        );
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_1() {
        let mut computer = Computer {
            a: 0,
            b: 0,
            c: 9,
            opcodes: vec![2, 6],
            out: vec![],
            pc: 0,
        };

        computer.run();

        assert_eq!(computer.b, 1)
    }

    #[test]
    fn test_2() {
        let mut computer = Computer {
            a: 10,
            b: 0,
            c: 0,
            opcodes: vec![5, 0, 5, 1, 5, 4],
            out: vec![],
            pc: 0,
        };

        computer.run();

        assert_eq!(computer.out, vec![0, 1, 2])
    }

    #[test]
    fn test_3() {
        let mut computer = Computer {
            a: 2024,
            b: 0,
            c: 0,
            opcodes: vec![0, 1, 5, 4, 3, 0],
            out: vec![],
            pc: 0,
        };

        computer.run();

        assert_eq!(computer.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0)
    }

    #[test]
    fn test_4() {
        let mut computer = Computer {
            a: 0,
            b: 29,
            c: 0,
            opcodes: vec![1, 7],
            out: vec![],
            pc: 0,
        };

        computer.run();

        assert_eq!(computer.b, 26)
    }

    #[test]
    fn test_5() {
        let mut computer = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            opcodes: vec![4, 0],
            out: vec![],
            pc: 0,
        };

        computer.run();

        assert_eq!(computer.b, 44354)
    }
}
