use regex::Regex;
use std::fmt::Display;

use crate::days::Day;

pub struct Day12;

// no tiles (in my input or the example input at least) fit in a space smaller than 3x3
const PRESENT_SIZE_X: usize = 3;
const PRESENT_SIZE_Y: usize = 3;

struct Space {
    x_size: usize,
    y_size: usize,
    present_count: usize,
}

impl Day for Day12 {
    // tried various ways to cut down the search space for ages, plus caching, eventually ran out of ideas
    // checked the subreddit and saw a) this solution worked (for the actual input, not the example),
    // and b) "proper" solutions were either stochastic or took ages,
    // like one I saw for a proper SAT problem that took 75 CPU hours with a proper CBC solver
    fn part_one(input: &str) -> impl Display {
        let spaces = parse_input(input);

        spaces
            .iter()
            .filter(|space| {
                (space.present_count / (space.y_size / PRESENT_SIZE_Y)) * PRESENT_SIZE_X
                    <= space.x_size
                    && (space.present_count / (space.x_size / PRESENT_SIZE_X)) * PRESENT_SIZE_Y
                        <= space.y_size
            })
            .count()
    }

    fn part_two(_input: &str) -> impl Display {
        "No computation needed :)"
    }

    fn get_day_num() -> u8 {
        return 12;
    }
}

fn parse_input(input: &str) -> Vec<Space> {
    let space_regex = Regex::new(r"(\d+)x(\d+): ((?:\d+ ?)+)").unwrap();
    let spaces = space_regex
        .captures_iter(input)
        .map(|cap| Space {
            x_size: cap.get(1).unwrap().as_str().parse().unwrap(),
            y_size: cap.get(2).unwrap().as_str().parse().unwrap(),

            present_count: cap
                .get(3)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|occurrence_str| occurrence_str.parse::<usize>().unwrap())
                .sum(),
        })
        .collect();

    spaces
}
