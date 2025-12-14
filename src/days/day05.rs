use itertools::Itertools;
use std::fmt::Display;
use std::ops::Range;

use crate::days::Day;

const _EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

pub struct Day05;

impl Day for Day05 {
    fn part_one(input: &str) -> impl Display {
        let (ranges, ingredients) = input
            .split_once("\n\n")
            .map(|(range_str, ingredient_str)| {
                (parse_ranges(range_str), parse_ingredients(ingredient_str))
            })
            .unwrap();

        ingredients
            .iter()
            .filter_map(
                |ingredient| match ranges.iter().any(|range| range.contains(ingredient)) {
                    true => Some(ingredient),
                    _ => None,
                },
            )
            .count()
    }

    fn part_two(input: &str) -> impl Display {
        let ranges: Vec<Range<i64>> = input
            .split_once("\n\n")
            .map(|(range_str, _)| {
                parse_ranges(range_str)
                    .iter()
                    .map(|range| (range.start, range.end))
                    .sorted() // sweep line algorithm requires list of ranges sorted by start of range
                    .collect::<Vec<(i64, i64)>>()
            })
            .unwrap()
            .iter()
            .map(|(start, end)| *start..*end)
            .collect();

        // sweep line algorithm for finding union of intervals
        let mut union_of_intervals = vec![];
        union_of_intervals.push(ranges[0].clone());

        for range in ranges {
            if union_of_intervals.last().unwrap().end < range.start {
                union_of_intervals.push(range);
            } else if union_of_intervals.last().unwrap().end < range.end {
                union_of_intervals.last_mut().unwrap().end = range.end;
            }
        }

        union_of_intervals
            .iter()
            .map(|range| range.end - range.start)
            .sum::<i64>()
    }

    fn get_day_num() -> u8 {
        return 5;
    }
}

fn parse_ranges(ranges: &str) -> Vec<Range<i64>> {
    ranges
        .lines()
        .map(|line| {
            line.trim()
                .split_once("-")
                .map(|(from, to)| from.parse().unwrap()..(to.parse::<i64>().unwrap() + 1))
                .unwrap()
        })
        .collect()
}

fn parse_ingredients(ingredients: &str) -> Vec<i64> {
    ingredients
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
