use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day08.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> usize {
    let trees = input.split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as i8
                })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let mut visible: Vec<(usize, usize)> = Vec::new();

    for i in 0..trees.len() {
    let mut max: i8 = -1;
        for j in 0..trees[0].len() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.push((i, j));
            }
            if max == 9 {
                break
            }
        }
    }

    for i in 0..trees.len() {
    let mut max: i8 = -1;
        for j in (0..trees[0].len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.push((i, j));
            }
            if max == 9 {
                break
            }
        }
    }

    for j in 0..trees.len() {
    let mut max: i8 = -1;
        for i in 0..trees[0].len() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.push((i, j));
            }
            if max == 9 {
                break
            }
        }
    }

    for j in 0..trees.len() {
    let mut max: i8 = -1;
        for i in (0..trees[0].len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.push((i, j));
            }
            if max == 9 {
                break
            }
        }
    }

    visible.iter().collect::<HashSet<&(usize, usize)>>().len()
}