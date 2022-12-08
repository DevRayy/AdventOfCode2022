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
    let forest = parse(input);

    forest.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, tree)| {
                    is_visible(&forest, i, j)
                })
                .count()
        })
        .sum::<usize>()
}

fn is_visible(forest: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let tree = forest[x][y];

    (x+1..forest.len()) //down
        .find(|&i| forest[i][y] >= tree) == None ||
    ((0..x).rev() //up
        .find(|&i| forest[i][y] >= tree) == None) ||
    ((y + 1..forest[x].len()) //right
        .find(|&i| forest[x][i] >= tree) == None) ||
    ((0..y).rev() //left
        .find(|&i| forest[x][i] >= tree) == None)
}

fn part2(input: &str) -> i32 {
    let forest = parse(input);

    forest.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    score(&forest, i, j)
                }).max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn score(forest: &Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
    if x == 0 || x == forest.len() || y == 0 || y == forest[0].len() {
        return 0
    }

    let center = forest[x][y];

    let score1 = match (x+1..forest.len())
        .position(|i| forest[i][y] >= center)
    {
        None => forest.len() - (x + 1),
        Some(pos) => pos + 1
    } as i32;

    let score2 = match (0..x).rev()
        .position(|i| forest[i][y] >= center)
    {
        None => x,
        Some(pos) => pos + 1
    } as i32;

    let score3 = match (y+1..forest[x].len())
        .position(|i| forest[x][i] >= center)
    {
        None => forest[x].len() - (y + 1),
        Some(pos) => pos + 1
    } as i32;

    let score4 = match (0..y).rev()
        .position(|i| forest[x][i] >= center)
    {
        None => y,
        Some(pos) => pos + 1
    } as i32;

    score1 * score2 * score3 * score4
}