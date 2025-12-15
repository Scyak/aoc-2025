use std::cmp::Ordering;
use std::collections::VecDeque;
use std::hash::Hash;
use std::{collections::HashSet, fmt::Display};

use crate::days::Day;
use itertools::Itertools;

pub struct Day08;

const _EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

const CONNECTIONS_TO_MAKE: usize = 1000;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: JunctionBox,
    to: JunctionBox,
    len: f64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.total_cmp(&other.len)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    fn new(from: JunctionBox, to: JunctionBox) -> Edge {
        Edge {
            from,
            to,
            len: f64::sqrt(
                ((from.x - to.x).pow(2) + (from.y - to.y).pow(2) + (from.z - to.z).pow(2)) as f64,
            ),
        }
    }
}

impl Day for Day08 {
    fn part_one(input: &str) -> impl Display {
        let (mut circuits, edges) = parse_input(input);

        // Kruskal's algorithm, but cut off at 1000 edges checked
        for i in 0..CONNECTIONS_TO_MAKE {
            let edge = edges[i];
            let from_idx = find_circuit_index(&circuits, edge.from);
            let to_idx = find_circuit_index(&circuits, edge.to);

            if from_idx != to_idx {
                circuits[from_idx] = circuits[from_idx]
                    .union(&circuits[to_idx])
                    .map(|junction_box| *junction_box)
                    .collect();
                circuits.remove(to_idx);
            }
        }

        circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>()
    }

    fn part_two(input: &str) -> impl Display {
        let (mut circuits, edges) = parse_input(input);

        // Kruskal's algorithm properly this time
        let mut last_edge = edges[0];
        for edge in edges {
            let from_idx = find_circuit_index(&circuits, edge.from);
            let to_idx = find_circuit_index(&circuits, edge.to);

            if from_idx != to_idx {
                circuits[from_idx] = circuits[from_idx]
                    .union(&circuits[to_idx])
                    .map(|junction_box| *junction_box)
                    .collect();
                circuits.remove(to_idx);
                last_edge = edge;
            }
        }

        last_edge.from.x * last_edge.to.x
    }

    fn get_day_num() -> u8 {
        return 8;
    }
}

fn parse_input(input: &str) -> (Vec<HashSet<JunctionBox>>, VecDeque<Edge>) {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(",").map(|coord_str| coord_str.parse().unwrap());
            JunctionBox {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
                z: coords.next().unwrap(),
            }
        })
        .collect();

    let circuits: Vec<HashSet<JunctionBox>> = boxes
        .iter()
        .map(|junction_box| {
            let mut circuit = HashSet::new();
            circuit.insert(junction_box.clone());
            circuit
        })
        .collect();

    let edges: VecDeque<Edge> = boxes
        .iter()
        .combinations(2)
        .map(|edge| Edge::new(*edge[0], *edge[1]))
        .sorted()
        .collect();

    (circuits, edges)
}

fn find_circuit_index(circuits: &Vec<HashSet<JunctionBox>>, to_find: JunctionBox) -> usize {
    circuits
        .iter()
        .position(|circuit| circuit.contains(&to_find))
        .unwrap()
}
