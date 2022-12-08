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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
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

fn part2(input: &str) -> i32 {
    let trees = input.split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as u8
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    trees.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    score(&trees, i, j)
                }).max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
    let center = trees[x][y];

    //DOWN
    let mut score1: i32 = 0;
    for i in x+1..trees.len() {
        let tree = trees[i][y];
        if tree < center {
            score1 += 1;
        } else if tree == center {
            score1 += 1;
            break
        } else {
            break
        }
    }

    //UP
    let mut score2: i32 = 0;
    for i in (0..x).rev() {
        let tree = trees[i][y];
        if tree < center {
            score2 += 1;
        } else if tree == center {
            score2 += 1;
            break
        } else {
            break
        }
    }

    //RIGHT
    let mut score3: i32 = 0;
    for j in y+1..trees[x].len() {
        let tree = trees[x][j];
        // println!("tree {} {} {}", x, j, tree);
        if tree < center {
            score3 += 1;
        } else if tree >= center {
            score3 += 1;
            break
        } else {
            break
        }
    }

    //LEFT
    let mut score4: i32 = 0;
    for j in (0..y).rev() {
        let tree = trees[x][j];
        if tree < center {
            score4 += 1;
        } else if tree == center {
            score4 += 1;
            break
        } else {
            break
        }
    }

    score1 * score2 * score3 * score4
}