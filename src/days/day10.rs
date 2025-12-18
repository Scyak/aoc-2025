use good_lp::solvers::coin_cbc::coin_cbc as default_solver;
use good_lp::*;
use itertools::Itertools;
use regex::Regex;
use std::{fmt::Display, vec};

use crate::days::Day;

const _EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

struct Button {
    toggles: Vec<bool>,
}

pub struct Day10;

impl Day for Day10 {
    fn part_one(input: &str) -> impl Display {
        //let input = _EXAMPLE_INPUT;
        let machines = parse_input(input);

        machines
            .iter()
            .map(|machine| presses_for_indicator(machine))
            .sum::<usize>()
    }

    fn part_two(input: &str) -> impl Display {
        let machines = parse_input(input);

        machines
            .iter()
            .map(|machine| presses_for_joltage(&machine))
            .sum::<usize>()
    }

    fn get_day_num() -> u8 {
        return 10;
    }
}

fn presses_for_indicator(machine: &Machine) -> usize {
    let mut presses = 1;

    loop {
        if machine
            .buttons
            .iter()
            .permutations(presses)
            .any(|button_combo| {
                button_combo.iter().fold(
                    vec![false; button_combo[0].toggles.len()],
                    |acc, button| {
                        acc.iter()
                            .enumerate()
                            .map(|(i, indicator)| indicator ^ button.toggles[i])
                            .collect()
                    },
                ) == machine.indicators
            })
        {
            break;
        }

        presses += 1;

        if presses == 10 {
            panic!("Too many presses to check");
        }
    }

    presses
}

fn presses_for_joltage(machine: &Machine) -> usize {
    let mut variables = ProblemVariables::new();
    let vars: Vec<Variable> =
        variables.add_vector(variable().min(0).integer(), machine.buttons.len());

    let mut problem = variables
        .minimise({
            let mut presses = Expression::with_capacity(machine.buttons.len());
            for var in &vars {
                presses += var;
            }

            presses
        })
        .using(default_solver);
    problem.set_parameter("loglevel", "0");

    for j_index in 0..machine.joltages.len() {
        problem = problem.with(
            ({
                let mut new_joltage = Expression::with_capacity(machine.joltages.len());
                for (b_index, button) in machine.buttons.iter().enumerate() {
                    if button.toggles[j_index] {
                        new_joltage += vars[b_index];
                    }
                }

                new_joltage
            })
            .eq(machine.joltages[j_index]),
        );
    }

    let solution = problem.solve().unwrap();
    (0..machine.buttons.len())
        .map(|index| solution.value(vars[index]))
        .sum::<f64>() as usize
}

fn parse_input(input: &str) -> Vec<Machine> {
    let indicator_re = Regex::new(r"\[([.#]+)\]").unwrap();
    let button_re = Regex::new(r"\((\d(?:,\d)*)\)").unwrap();
    let joltage_re = Regex::new(r"\{(\d+(?:,\d+)*)\}").unwrap();

    input
        .lines()
        .map(|line| {
            let indicators: Vec<bool> = indicator_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .map(|indicator| match indicator {
                    '#' => true,
                    '.' => false,
                    other => panic!("Unexpected indicator {other}!"),
                })
                .collect();
            let indicator_count = indicators.len();
            Machine {
                indicators,
                buttons: button_re
                    .captures_iter(line)
                    .map(|capture| {
                        let mut button_toggles = vec![false; indicator_count];
                        capture
                            .get(1)
                            .unwrap()
                            .as_str()
                            .split(",")
                            .map(|id_str| id_str.parse::<usize>().unwrap())
                            .for_each(|id| button_toggles[id] = true);
                        Button {
                            toggles: button_toggles,
                        }
                    })
                    .collect(),
                joltages: joltage_re
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|joltage_str| joltage_str.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}
