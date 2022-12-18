use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day18.txt")
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


fn parse(input: &str) -> Vec<(i64, i64, i64)> {
    input.split("\n")
        .map(|line| {
            let mut s = line.split(",")
                .map(|x| x.parse::<i64>().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let cubes = parse(input);

    cubes.par_iter()
        .map(|cube| {
            6 - cubes.iter()
                .map(|cube2| manhattan(cube, cube2))
                .filter(|dist| *dist == 1)
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let cubes = parse(input).into_iter().collect::<HashSet<(i64, i64, i64)>>();

    let (&min_x, &max_x) = cubes.iter()
        .map(|(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let min_x = min_x - 1;
    let max_x = max_x + 1;

    let (&min_y, &max_y) = cubes.iter()
        .map(|(_, x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let min_y = min_y - 1;
    let max_y = max_y + 1;

    let (&min_z, &max_z) = cubes.iter()
        .map(|(_, _, x)| x)
        .minmax()
        .into_option()
        .unwrap();
    let min_z = min_z - 1;
    let max_z = max_z + 1;

    let mut q: VecDeque<(i64, i64, i64)> = VecDeque::new();
    q.push_front((min_x, min_y, min_z));

    let mut water: HashSet<(i64, i64, i64)> = HashSet::new();
    let dirs: Vec<(i64, i64, i64)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    while let Some(n) = q.pop_back() {
        if n.0 > max_x || n.0 < min_x || n.1 > max_y || n.1 < min_y || n.2 > max_z || n.2 < min_z {
            continue
        }
        if cubes.contains(&n) {
            continue
        }
        if water.contains(&n) {
            continue
        }
        water.insert(n);

        for dir in &dirs {
            let next = (n.0 + dir.0, n.1 + dir.1, n.2 + dir.2);
            q.push_front(next);
        }
    }

    water.par_iter()
        .map(|cube| {
            cubes.iter()
                .map(|cube2| manhattan(cube, cube2))
                .filter(|dist| *dist == 1)
                .count()
        })
        .sum()
}

fn manhattan(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}