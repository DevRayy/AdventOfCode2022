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
    assert_eq!(part1_ans, 1798);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
    assert_eq!(part2_ans, 259308);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as u8
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn part1(input: &str) -> usize {
    let trees = parse(input);

    trees.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, tree)| {
                    is_visible(&trees, i, j)
                })
                .count()
        })
        .sum::<usize>()
}

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let tree = trees[x][y];

    (x+1..trees.len()) //down
        .find(|&i| trees[i][y] >= tree) == None ||
    ((0..x).rev() //up
        .find(|&i| trees[i][y] >= tree) == None) ||
    ((y + 1..trees[x].len()) //right
        .find(|&i| trees[x][i] >= tree) == None) ||
    ((0..y).rev() //left
        .find(|&i| trees[x][i] >= tree) == None)
}

fn part2(input: &str) -> i32 {
    let trees = parse(input);

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