use std::fmt::Display;

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_one(input: &str) -> impl Display {
        let mut position: i32 = 50;
        let mut result = 0;

        for line in input.lines() {
            match line.split_at(1) {
                ("R", distance) => position += distance.parse::<i32>().unwrap(),
                ("L", distance) => position -= distance.parse::<i32>().unwrap(),
                (_, _) => println!("Encountered unknown direction"),
            }

            while position < 0 {
                position += 100;
            }

            while position > 99 {
                position -= 100;
            }

            if position == 0 {
                result += 1;
            }
        }

        result
    }

    fn part_two(input: &str) -> impl Display {
        let mut position: i32 = 50;
        let mut result = 0;

        for line in input.lines() {
            if position == 0 && line.starts_with("L") {
                position += 100;
            }

            match line.split_at(1) {
                ("R", distance) => position += distance.parse::<i32>().unwrap(),
                ("L", distance) => position -= distance.parse::<i32>().unwrap(),
                (_, _) => println!("Encountered unknown direction"),
            }

            while position < 0 {
                position += 100;
                result += 1;
            }

            if position == 0 {
                result += 1;
            }

            while position > 99 {
                position -= 100;
                result += 1;
            }
        }

        result
    }
    fn get_day_num() -> u8 {
        return 1;
    }
}
