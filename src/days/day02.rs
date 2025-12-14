use std::fmt::Display;

use crate::days::Day;

pub struct Day02;

const _EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

impl Day for Day02 {
    fn part_one(input: &str) -> impl Display {
        input
            .split(",")
            .map(|range| parse_range(range))
            .map(|(lower_bound, upper_bound)| sum_invalid_in_range(lower_bound, upper_bound, false))
            .sum::<u64>()
    }

    fn part_two(input: &str) -> impl Display {
        input
            .split(",")
            .map(|range| parse_range(range))
            .map(|(lower_bound, upper_bound)| sum_invalid_in_range(lower_bound, upper_bound, true))
            .sum::<u64>()
    }

    fn get_day_num() -> u8 {
        return 2;
    }
}

fn parse_range(range_str: &str) -> (u64, u64) {
    let split: Vec<u64> = range_str
        .split("-")
        .map(|bound| bound.trim().parse::<u64>().unwrap())
        .collect();
    (split[0], split[1])
}

fn sum_invalid_in_range(lower_bound: u64, upper_bound: u64, part_two: bool) -> u64 {
    let mut sum = 0;

    for id in lower_bound..=upper_bound {
        if !part_two && check_id_part_one(id) || part_two && check_id_part_two(id) {
            sum += id;
        }
    }

    sum
}

fn check_id_part_one(id: u64) -> bool {
    let digits = id.checked_ilog10().unwrap_or(0) + 1;

    if digits % 2 == 1 {
        return false;
    }

    check_repeats(id, digits, 2)
}

fn check_id_part_two(id: u64) -> bool {
    let digits = id.checked_ilog10().unwrap_or(0) + 1;
    for potential_occurrences in 2..=digits {
        if (digits % potential_occurrences) == 0 && check_repeats(id, digits, potential_occurrences)
        {
            return true;
        }
    }

    false
}

fn check_repeats(id: u64, digits: u32, occurrences: u32) -> bool {
    let repeat_len = digits / occurrences;

    (1..=occurrences)
        .map(|repeat| {
            (id % 10_u64.pow(repeat * repeat_len)) / 10_u64.pow((repeat - 1) * repeat_len)
        })
        .collect::<Vec<u64>>()
        .windows(2)
        .all(|parts| parts[0] == parts[1])
}
