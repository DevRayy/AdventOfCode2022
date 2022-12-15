use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day15.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
}

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    range: i64,
}

impl Sensor {
    fn new(pos: (i64, i64), beacon: (i64, i64)) -> Sensor {
        Sensor{
            pos,
            beacon,
            range: manhattan(pos, beacon),
        }
    }
}

fn manhattan(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn parse(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Sensor::new(
                (caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                 caps.get(2).unwrap().as_str().parse::<i64>().unwrap()),
                (caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                 caps.get(4).unwrap().as_str().parse::<i64>().unwrap())
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let sensors = parse(input);
    let target_y: i64 = 10;

    let mut nobeacons = HashSet::<i64>::new();

    for s in sensors {
        if s.range < (s.pos.1 - target_y).abs() {
            continue
        }
        let target_range = s.range - (s.pos.1 - target_y).abs();
        let target_min = s.pos.0 - target_range;
        let target_max = s.pos.0 + target_range;

        for x in target_min..target_max {
            nobeacons.insert(x);
        }
    }

    nobeacons.len()
}

fn part2(input: &str) -> usize {
    let sensors = parse(input);
    let max_x = 4000000;
    let max_y = 4000000;

    let point = sensors.iter()
        .map(|s| {
            let mut points = Vec::<(i64, i64)>::with_capacity(4*(s.range as usize +1));
            for a in 0..s.range+1 {
                points.push((s.pos.0+a, s.pos.1+(s.range+1)-a));
                points.push((s.pos.0+a, s.pos.1-((s.range+1)-a)));
                points.push((s.pos.0-a, s.pos.1+(s.range+1)-a));
                points.push((s.pos.0-a, s.pos.1-((s.range+1)-a)));
            }
            points
        })
        .flatten()
        .filter(|&p| {
            p.0 >= 0 &&
            p.0 <= max_x &&
            p.1 >= 0 &&
            p.1 <= max_y &&
            sensors.iter()
                .filter(|s| {
                    manhattan(p, s.pos) <= s.range
                })
                .count() == 0
        })
        .next()
        .unwrap();

    return (point.0 * 4000000 + point.1) as usize
}

