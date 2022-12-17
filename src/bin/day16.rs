use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans : {:.2?}", part2_ans);
}

#[derive(Debug)]
struct Valve {
    id: String,
    rate: u64,
    neighbours: Vec<String>,
}

impl Valve {
    fn new(id: &str, rate: u64, neighbours_raw: &str) -> Valve {
        Valve{
            id: String::from(id),
            rate,
            neighbours: neighbours_raw.split(", ").map(|x| String::from(x)).collect(),
        }
    }
}

fn parse(input: &str) -> Vec<Valve> {
    let re = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Valve::new(
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str()
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
    let mut budget: i64 = 30;
    println!("{:?}", graph);
    0
}