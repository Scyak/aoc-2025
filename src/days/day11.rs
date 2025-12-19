use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::days::Day;

const _EXAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const _EXAMPLE_INPUT_PART_TWO: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

pub struct Day11;

impl Day for Day11 {
    fn part_one(input: &str) -> impl Display {
        let connections = parse_input(input);

        // assumption: paths from you to out all have no cycles
        // otherwise the number of paths would be infinite, which is not a possible AoC answer
        paths("you", "out", &connections)
    }

    fn part_two(input: &str) -> impl Display {
        let connctions = parse_input(input);

        // searching svr->[dac|fft], [dac->fft|fft->dac], [dac|fft]->out and multiplying
        // assumption: paths between the individual stops all have no cycles, and one of dac->fft and fft->dac is 0
        // otherwise the number of valid paths would be infinite, which is not a possible AoC answer

        let paths_dac_fft = paths("dac", "fft", &connctions);

        if paths_dac_fft == 0 {
            // paths from svr to fft, then to dac, then out
            let paths_svr_fft = paths("svr", "fft", &connctions);
            let paths_fft_dac = paths("fft", "dac", &connctions);
            let paths_dac_out = paths("dac", "out", &connctions);
            paths_svr_fft * paths_fft_dac * paths_dac_out
        } else {
            // paths from svr to dac, then to fft, then out
            let paths_svr_dac = paths("svr", "dac", &connctions);
            let paths_fft_out = paths("fft", "out", &connctions);
            paths_svr_dac * paths_dac_fft * paths_fft_out
        }
    }

    fn get_day_num() -> u8 {
        return 11;
    }
}

// returns Hashmap node -> nodes it connects to
fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (from, to_str) = line.split_once(": ").unwrap();
            (from, to_str.split_whitespace().collect())
        })
        .collect()
}

// contains no cycle detection (only counts number of paths, does not record their nodes, so no way to tell)
// -> will not terminate if any paths with cycles lead to node <to>, even if not from node <from>
// my input seems to contain none (even outside the relevant paths), possibly true for all existing inputs?
fn paths(from: &str, to: &str, connections: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut path_count_to: HashMap<&str, u64> = HashMap::new();
    path_count_to.insert(from, 1);

    let mut to_check: VecDeque<&str> = VecDeque::new();
    to_check.push_back(from);
    while !to_check.is_empty() {
        let current_node = to_check.pop_front().unwrap();

        if *path_count_to.get(current_node).unwrap_or(&0) == 0 {
            // no new paths to pass upwards
            continue;
        }

        connections
            .get(current_node)
            .unwrap_or(&vec![])
            .iter()
            .for_each(|next_node| {
                *path_count_to.entry(&next_node).or_insert(0) +=
                    *path_count_to.get(current_node).unwrap_or(&0);
                if *next_node != to {
                    to_check.push_back(&next_node);
                }
            });

        // zero out paths to current node so they don't get double counted later
        *path_count_to.entry(current_node).or_default() = 0;
    }

    *path_count_to.get(to).unwrap_or(&0)
}
