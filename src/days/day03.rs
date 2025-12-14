use std::fmt::Display;

use crate::days::Day;

pub struct Day03;

const _EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

impl Day for Day03 {
    fn part_one(input: &str) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            sum += find_max_joltage(line, 2);
        }

        sum
    }

    fn part_two(input: &str) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            sum += find_max_joltage(line, 12);
        }

        sum
    }

    fn get_day_num() -> u8 {
        return 3;
    }
}

fn find_max_joltage(batteries: &str, digits: usize) -> u64 {
    let mut total_joltage = 0;
    let mut start_idx = 0;

    for digit in (0..digits).rev() {
        for joltage in (1..=9).rev() {
            match batteries[start_idx..batteries.len() - digit].find(&joltage.to_string()) {
                Some(index) => {
                    start_idx = start_idx + index + 1;
                    total_joltage += joltage * 10_u64.pow(digit as u32)
                }
                None => continue,
            }
            break;
        }
    }

    total_joltage
}
