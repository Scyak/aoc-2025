use std::fmt::Display;

use crate::days::Day;

pub struct Day04;

const _EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn step(&self, x_step: isize, y_step: isize) -> Coordinate {
        Coordinate {
            x: self.x + x_step,
            y: self.y + y_step,
        }
    }

    fn within_bounds(&self, xlen: isize, ylen: isize) -> bool {
        self.x >= 0 && self.x < xlen && self.y >= 0 && self.y < ylen
    }

    fn neighbors(&self, xlen: isize, ylen: isize) -> Vec<Coordinate> {
        (-1..=1)
            .map(|x_step| {
                (-1..=1)
                    .filter_map(|y_step| match self.step(x_step, y_step) {
                        neighbor
                            if neighbor.within_bounds(xlen, ylen)
                                && !(neighbor.x == self.x && neighbor.y == self.y) =>
                        {
                            Some(neighbor)
                        }
                        _ => None,
                    })
                    .collect::<Vec<Coordinate>>()
            })
            .flatten()
            .collect()
    }
}

impl Day for Day04 {
    fn part_one(input: &str) -> impl Display {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        grid.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, element)| match element {
                        '@' => Some(
                            Coordinate {
                                x: x as isize,
                                y: y as isize,
                            }
                            .neighbors(grid[0].len() as isize, grid.len() as isize)
                            .iter()
                            .filter(|coordinate| {
                                grid[coordinate.y as usize][coordinate.x as usize] == '@'
                            })
                            .count(),
                        ),

                        _ => None,
                    })
                    .filter(|&neighboring_rolls| neighboring_rolls < 4)
                    .count()
            })
            .sum::<usize>()
    }

    fn part_two(input: &str) -> impl Display {
        let mut removable_count = 0;

        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut removable = find_removable(&grid);

        while removable.len() != 0 {
            removable_count += removable.len();

            grid = grid
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(x, element)| match (x, y) {
                            (x, y)
                                if removable.contains(&Coordinate {
                                    x: x as isize,
                                    y: y as isize,
                                }) =>
                            {
                                '.'
                            }

                            _ => *element,
                        })
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            removable = find_removable(&grid);
        }

        removable_count
    }

    fn get_day_num() -> u8 {
        return 4;
    }
}

fn find_removable(grid: &Vec<Vec<char>>) -> Vec<Coordinate> {
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, element)| match element {
                    '@' => {
                        let coordinate = Coordinate {
                            x: x as isize,
                            y: y as isize,
                        };
                        Some((
                            coordinate,
                            coordinate
                                .neighbors(grid[0].len() as isize, grid.len() as isize)
                                .iter()
                                .filter(|coordinate| {
                                    grid[coordinate.y as usize][coordinate.x as usize] == '@'
                                })
                                .count(),
                        ))
                    }

                    _ => None,
                })
                .filter_map(|(coordinate, neighboring_rolls)| match neighboring_rolls {
                    rolls if rolls < 4 => Some(coordinate),
                    _ => None,
                })
                .collect::<Vec<Coordinate>>()
        })
        .flatten()
        .collect::<Vec<Coordinate>>()
}
