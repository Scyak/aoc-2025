use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    iter::repeat,
    vec,
};

use crate::days::Day;

const _EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    row: usize,
    column: usize,
    leaf: bool,
}

pub struct Day07;

impl Day for Day07 {
    fn part_one(input: &str) -> impl Display {
        let mut beam_columns: HashSet<usize> = HashSet::new();
        beam_columns.insert(input.find('S').unwrap());
        let mut splits = 0;

        input.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '^' && beam_columns.contains(&idx) {
                    splits += 1;
                    beam_columns.remove(&idx);
                    beam_columns.insert(idx + 1);
                    beam_columns.insert(idx - 1);
                }
            })
        });

        splits
    }

    fn part_two(input: &str) -> impl Display {
        let nodes = parse_input(input);
        let node_count = nodes.len();
        let mut paths: Vec<Vec<u64>> = vec![vec![0; node_count]; node_count];

        // initialize path matrix with edges of directed graph
        let mut beams: HashMap<usize, Vec<usize>> = HashMap::new();
        beams.insert(nodes[0].column - 1, vec![0]);
        beams.insert(nodes[0].column + 1, vec![0]);
        nodes
            .iter()
            .skip(1)
            .enumerate()
            .map(|(index, node)| (index + 1, node))
            .for_each(|(index, node)| {
                if beams.contains_key(&node.column) && !beams.get(&node.column).unwrap().is_empty()
                {
                    beams
                        .get(&node.column)
                        .unwrap()
                        .iter()
                        .for_each(|parent_idx| paths[*parent_idx][index] += 1);
                    beams.remove(&node.column);

                    if !node.leaf {
                        beams.entry(node.column + 1).or_default().push(index);
                        beams.entry(node.column - 1).or_default().push(index);
                    }
                }
            });

        // based on algorithm from https://stackoverflow.com/questions/1642139/algorithm-to-find-the-number-of-distinct-paths-in-a-directed-graph
        // note: answer is slightly inaccurate (not a modified Dijkstra, and doesn't work for cycles,
        // which the OP requested but I don't have anyways) but works for this case
        // modification: removed outer loop since only paths starting at node 0 are relevant for this problem
        for to_node in 0..node_count {
            for via_node in 0..node_count {
                paths[0][to_node] += paths[0][via_node] * paths[via_node][to_node];
            }
        }

        paths[0]
            .iter()
            .enumerate()
            .filter_map(|(index, path_count)| {
                if nodes[index].leaf {
                    Some(path_count)
                } else {
                    None
                }
            })
            .sum::<u64>()
    }

    fn get_day_num() -> u8 {
        return 7;
    }
}

fn parse_input(input: &str) -> Vec<Node> {
    let last_row = input.lines().count() - 1;
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, c)| match c {
                    '^' => Some(Node {
                        row,
                        column,
                        leaf: false,
                    }),
                    _ => None,
                })
        })
        .flatten()
        .chain(
            input
                .lines()
                .last()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(column, _)| Node {
                    row: last_row,
                    column,
                    leaf: true,
                }),
        )
        .collect()
}

fn _print_path_matrix(paths: &Vec<Vec<u32>>, nodes: &Vec<Node>) {
    let node_count = nodes.len();
    print!("   | ");
    for index in 0..node_count {
        if nodes[index].leaf {
            print!("\x1b[1m{index: >2}  \x1b[0m");
        } else {
            print!("{index: >2}  ");
        }
    }
    println!();
    print!("---|-");
    println!("{}", repeat('-').take(node_count * 4).collect::<String>());
    for (from_node_idx, row) in paths.iter().enumerate() {
        if nodes[from_node_idx].leaf {
            print!("\x1b[1m{from_node_idx: >2}\x1b[0m | ");
        } else {
            print!("{from_node_idx: >2} | ");
        }
        for path_count in row {
            print!("{path_count: >2}  ");
        }
        println!();
    }
    println!();
}
