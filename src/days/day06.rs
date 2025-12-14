use std::{fmt::Display, vec};

use crate::days::Day;

const _EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

pub struct Day06;

impl Day for Day06 {
    fn part_one(input: &str) -> impl Display {
        let (args, operations) = parse_input(input);

        operations
            .iter()
            .enumerate()
            .map(|(index, operation)| -> u64 {
                match *operation {
                    "+" => args.iter().map(|arg_vec| arg_vec[index]).sum(),
                    "*" => args.iter().map(|arg_vec| arg_vec[index]).product(),
                    _ => panic!("Unexpected operation"),
                }
            })
            .sum::<u64>()
    }

    fn part_two(input: &str) -> impl Display {
        let (args, operations) = parse_input_part_two(input);

        args.iter()
            .enumerate()
            .map(|(index, op_args)| -> u64 {
                match operations[index] {
                    '+' => op_args.iter().sum(),
                    '*' => op_args.iter().product(),
                    _ => panic!("Unexpected operation"),
                }
            })
            .sum::<u64>()
    }

    fn get_day_num() -> u8 {
        return 6;
    }
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut lines = input.lines().rev();
    let operations = lines.next().unwrap().split_whitespace().collect();
    let args = lines
        .map(|line| {
            line.split_whitespace()
                .map(|arg| arg.parse().unwrap())
                .collect()
        })
        .collect();

    (args, operations)
}

fn parse_input_part_two(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let input_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let argument_input = &input_grid[0..input_grid.len() - 1];
    let mut args = vec![];
    let mut operations = vec![];

    let mut operation_args = vec![];
    for column in 0..input_grid[0].len() {
        let operation = input_grid.last().unwrap()[column];

        if operation != ' ' {
            operations.push(operation);
            if !operation_args.is_empty() {
                args.push(operation_args);
            }
            operation_args = vec![];
        }

        let arg = argument_input
            .iter()
            .map(|row| row[column])
            .collect::<String>()
            .trim()
            .parse::<u64>();

        match arg {
            Ok(arg) => operation_args.push(arg),
            _ => (),
        }
    }
    args.push(operation_args);
    (args, operations)
}
