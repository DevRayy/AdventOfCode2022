use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::time::Instant;

use itertools::*;

fn main() {
    let input = fs::read_to_string("data/day03.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> i32 {
    input.split("\n")
        .into_iter()
        .map(|x| {
            let halflen = x.len()/2;
            let mut comp1 = x.chars()
                .take(halflen)
                .collect::<Vec<char>>();
            let comp2 = x.chars()
                .rev()
                .take(halflen)
                .collect::<Vec<char>>();
            comp1.retain(|e| comp2.contains(e));
            comp1.iter().nth(0).unwrap().clone()
        })
        .map(|c| {
            if c.is_ascii_uppercase() {
                c as i32 - 38
            } else {
                c as i32 - 96
            }
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input.split("\n")
        .into_iter()
        .tuples()
        .map(|(elf1, elf2, elf3)| {
            let mut ruck1 = elf1.chars().collect::<Vec<char>>();
            let ruck2 = elf2.chars().collect::<Vec<char>>();
            let ruck3 = elf3.chars().collect::<Vec<char>>();
            ruck1.retain(|e| ruck2.contains(e) && ruck3.contains(e));
            ruck1.iter().nth(0).unwrap().clone()
        })
        .map(|c| {
            if c.is_ascii_uppercase() {
                c as i32 - 38
            } else {
                c as i32 - 96
            }
        })
        .sum::<i32>()
}
