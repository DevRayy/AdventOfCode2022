use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day23.txt")
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

const DIR_N:  (i64, i64) = (-1, 0);
const DIR_NE: (i64, i64) = (-1, 1);
const DIR_E:  (i64, i64) = (0, 1);
const DIR_SE: (i64, i64) = (1, 1);
const DIR_S:  (i64, i64) = (1, 0);
const DIR_SW: (i64, i64) = (1, -1);
const DIR_W:  (i64, i64) = (0, -1);
const DIR_NW: (i64, i64) = (-1, -1);

const DIRS: [[(i64, i64); 3]; 4] = [
    [DIR_N, DIR_NE, DIR_NW],
    [DIR_S, DIR_SE, DIR_SW],
    [DIR_W, DIR_NW, DIR_SW],
    [DIR_E, DIR_NE, DIR_SE]
];

const DIRS_ALL: [(i64, i64); 8] = [DIR_N, DIR_NE, DIR_E, DIR_SE, DIR_S, DIR_SW, DIR_W, DIR_NW];

fn parse(input: &str) -> HashSet<(i64, i64)> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| {
                    ((i as i64, j as i64), c)
                })
        })
        .flatten()
        .filter(|(_, tile)| *tile == '#')
        .map(|(x, _)| x)
        .collect()
}


fn part1(input: &str) -> i64 {
    let mut set = parse(input);

    for i in 0..10 {
        let mut propositions: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
        for elf in &set {
            let proposition = propose(&set, &elf, i);
            propositions.insert(*elf, proposition);
        }

        let mut new_set: HashSet<(i64, i64)> = HashSet::new();
        for (elf, proposition) in &propositions {
            if propositions.iter()
                .filter(|(_, p)| *p == proposition)
                .count() == 1 {
                new_set.insert(*proposition);
            } else {
                new_set.insert(*elf);
            }
        }

        set = new_set;
    }

    score(&set)
}


fn part2(input: &str) -> usize {
    let mut set = parse(input);

    let mut i = 0;
    loop {
        let mut propositions: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
        for elf in &set {
            let proposition = propose(&set, &elf, i);
            propositions.insert(*elf, proposition);
        }

        let mut new_set: HashSet<(i64, i64)> = HashSet::new();
        let mut should_break = true;
        for (elf, proposition) in &propositions {
            if propositions.iter()
                .filter(|(_, p)| *p == proposition)
                .count() == 1 {
                new_set.insert(*proposition);
                if proposition != elf {
                    should_break = false;
                }
            } else {
                new_set.insert(*elf);
            }
        }

        set = new_set;
        i += 1;
        if should_break {
            break i;
        }
    }
}

fn propose(set: &HashSet<(i64, i64)>, elf: &(i64, i64), it: usize) -> (i64, i64) {
    if DIRS_ALL.iter()
        .filter(|d| set.contains(&add(&elf, d)))
        .count() == 0 {
        return *elf
    }
    for i in 0..DIRS.len() {
        let dir = DIRS[(it + i) % DIRS.len()];
        if dir.iter()
            .filter(|d| set.contains(&add(&elf, d)))
            .count() == 0 {
            return add(&elf, &dir[0])
        }
    }
    return *elf
}

fn add(p1: &(i64, i64), p2: &(i64, i64)) -> (i64, i64) {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn score(set: &HashSet<(i64, i64)>) -> i64 {
    let min_x = set.iter().map(|x| x.0).min().unwrap();
    let max_x = set.iter().map(|x| x.0).max().unwrap();
    let min_y = set.iter().map(|x| x.1).min().unwrap();
    let max_y = set.iter().map(|x| x.1).max().unwrap();
    ((max_x - min_x + 1).abs() * (max_y - min_y + 1).abs()) - set.len() as i64
}

fn print(set: &HashSet<(i64, i64)>) {
    let min_x = set.iter().map(|x| x.0).min().unwrap();
    let max_x = set.iter().map(|x| x.0).max().unwrap();
    let min_y = set.iter().map(|x| x.1).min().unwrap();
    let max_y = set.iter().map(|x| x.1).max().unwrap();

    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {
            print!("{}", if set.contains(&(x, y)) {"#"} else {"."});
        }
        print!("\n")
    }
}