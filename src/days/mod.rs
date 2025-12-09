use std::{fmt::Display, fs::read_to_string, time::Instant};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub trait Day {
    fn part_one(input: &str) -> impl Display;
    fn part_two(input: &str) -> impl Display;
    fn get_day_num() -> u8;
    fn run() {
        let day = Self::get_day_num();

        println!("--- Day {} ---", day);
        let path_str = format!("input/day{:0>2}.txt", day);
        let input = match read_to_string(&path_str) {
            Ok(input) => input,
            Err(_) => {
                println!(
                    "Couldn't read input file for day {}. Not running this day.",
                    day
                );
                return;
            }
        };

        println!();

        let start_time_one = Instant::now();
        println!("- Starting part one -");
        let result_one = Self::part_one(&input);
        println!("Result (part one): {result_one}");
        println!("Elapsed time: {:?}", start_time_one.elapsed());

        println!();

        let start_time_two = Instant::now();
        println!("- Starting part two -");
        let result_two = Self::part_two(&input);
        println!("Result (part two): {result_two}");
        println!("Elapsed time: {:?}", start_time_two.elapsed());

        println!();
    }
}
