use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::hash::Hash;
use std::time::Instant;
use itertools::max;
use rayon::prelude::*;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day19.txt")
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Storage {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Storage {
    fn new(ore: i64, clay: i64, obsidian: i64, geode: i64) -> Storage {
        Storage{ore, clay, obsidian, geode}
    }

    fn add(&mut self, other: &Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn subtract(&mut self, other: &Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }

    fn contains(&self, other: &Self) -> bool {
        self.ore >= other.ore &&
            self.clay >= other.clay &&
            self.obsidian >= other.obsidian &&
            self.geode >= other.geode
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Blueprint {
    robot_ore_cost: Storage,
    robot_clay_cost: Storage,
    robot_obsidian_cost: Storage,
    robot_geode_cost: Storage,
    max_needed_ore: i64,
    max_needed_clay: i64,
    max_needed_obsidian: i64,
}

impl Blueprint {
    fn new(ore_ore: i64, clay_ore: i64, obsidian_ore: i64, obsidian_clay: i64, geode_ore: i64, geode_obsidian: i64) -> Blueprint {
        Blueprint{
            robot_ore_cost: Storage::new(ore_ore, 0, 0, 0),
            robot_clay_cost: Storage::new(clay_ore, 0, 0, 0),
            robot_obsidian_cost: Storage::new(obsidian_ore, obsidian_clay, 0, 0),
            robot_geode_cost: Storage::new(geode_ore, 0, geode_obsidian, 0),
            max_needed_ore: max([ore_ore, clay_ore, obsidian_ore, geode_ore]).unwrap(),
            max_needed_clay: obsidian_clay,
            max_needed_obsidian: geode_obsidian,
        }
    }
}

fn parse(input: &str) -> HashMap<i64, Blueprint> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let blueprint = Blueprint::new(
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
                caps.get(5).unwrap().as_str().parse().unwrap(),
                caps.get(6).unwrap().as_str().parse().unwrap(),
                caps.get(7).unwrap().as_str().parse().unwrap(),
            );
            (id, blueprint)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let blueprints = parse(input);
    let max_steps: i64 = 24;

    blueprints.par_iter()
        .map(|(id, blueprint)| {
            find_max_geodes(blueprint, max_steps) * id
        })
        .sum::<i64>() as usize
}

fn part2(input: &str) -> usize {
    let blueprints = parse(input);
    let blueprints = [
        *blueprints.get(&1).unwrap(),
        *blueprints.get(&2).unwrap(),
        *blueprints.get(&3).unwrap(),
    ];
    let max_steps: i64 = 32;

    blueprints.par_iter()
        .map(|blueprint| {
            find_max_geodes(blueprint, max_steps)
        })
        .product::<i64>() as usize
}

fn find_max_geodes(blueprint: &Blueprint, max_steps: i64) -> i64 {
    let mut states: Vec<State> = vec![State::new(blueprint)];
    for i in 0..max_steps {
        states = states.iter()
            .map(|s| s.branch())
            .flatten()
            .collect();
        let current_best = states.iter()
            .max_by(|s1, s2| s1.resources.geode.cmp(&s2.resources.geode))
            .unwrap()
            .resources.geode;
        states = states.into_iter()
            .filter(|s| current_best < s.resources.geode + max_steps - i)
            .collect();
    }
    states.iter()
        .max_by(|s1, s2| s1.resources.geode.cmp(&s2.resources.geode))
        .unwrap()
        .resources.geode
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    blueprint: &'a Blueprint,
    robots: Storage,
    resources: Storage,
    forbid_ore: bool,
    forbid_clay: bool,
    forbid_obsidian: bool,
    forbid_geode: bool
}

impl <'a> State<'a> {
    fn new(blueprint: &Blueprint) -> State {
        State{
            blueprint,
            robots: Storage::new(1, 0, 0, 0),
            resources: Storage::new(0, 0, 0, 0),
            forbid_ore: false,
            forbid_clay: false,
            forbid_obsidian: false,
            forbid_geode: false,
        }
    }

    fn reset(&mut self) {
        self.forbid_ore = false;
        self.forbid_clay = false;
        self.forbid_obsidian = false;
        self.forbid_geode = false;
    }

    fn branch(self) -> Vec<State<'a>> {
        let mut new_states: Vec<State> = Vec::new();

        let can_build_ore = self.resources.contains(&self.blueprint.robot_ore_cost);
        let can_build_clay = self.resources.contains(&self.blueprint.robot_clay_cost);
        let can_build_obsidian = self.resources.contains(&self.blueprint.robot_obsidian_cost);
        let can_build_geode = self.resources.contains(&self.blueprint.robot_geode_cost);

        let mut blank_state = self.clone();
        blank_state.resources.add(&self.robots);
        blank_state.forbid_ore = can_build_ore;
        blank_state.forbid_clay = can_build_clay;
        blank_state.forbid_obsidian = can_build_obsidian;
        blank_state.forbid_geode = can_build_geode;
        new_states.push(blank_state);

        if can_build_ore && !self.forbid_ore && self.robots.ore < self.blueprint.max_needed_ore {
            let mut ore_state = self.clone();
            ore_state.robots.ore += 1;
            ore_state.resources.subtract(&self.blueprint.robot_ore_cost);
            ore_state.resources.add(&self.robots);
            ore_state.reset();
            new_states.push(ore_state);
        }
        if can_build_clay && !self.forbid_clay && self.robots.clay < self.blueprint.max_needed_clay {
            let mut clay_state = self.clone();
            clay_state.robots.clay += 1;
            clay_state.resources.subtract(&self.blueprint.robot_clay_cost);
            clay_state.resources.add(&self.robots);
            clay_state.reset();
            new_states.push(clay_state);
        }
        if can_build_obsidian && !self.forbid_obsidian && self.robots.obsidian < self.blueprint.max_needed_obsidian {
            let mut obsidian_state = self.clone();
            obsidian_state.robots.obsidian += 1;
            obsidian_state.resources.subtract(&self.blueprint.robot_obsidian_cost);
            obsidian_state.resources.add(&self.robots);
            obsidian_state.reset();
            new_states.push(obsidian_state);
        }
        if can_build_geode && !self.forbid_geode {
            let mut geode_state = self.clone();
            geode_state.robots.geode += 1;
            geode_state.resources.subtract(&self.blueprint.robot_geode_cost);
            geode_state.resources.add(&self.robots);
            geode_state.reset();
            new_states.push(geode_state);
        }
        new_states
    }
}

impl Debug for State<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "robots: {:?}; resources: {:?}", self.robots, self.resources)
    }
}