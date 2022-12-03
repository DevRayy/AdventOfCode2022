use std::collections::HashSet;
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
            let comp1 = x.chars()
                .take(halflen)
                .collect::<HashSet<char>>();
            let comp2 = x.chars()
                .rev()
                .take(halflen)
                .collect::<HashSet<char>>();
            (comp1, comp2)
        })
        .map(|(comp1, comp2)| {
            comp1.intersection(&comp2)
                .nth(0)
                .unwrap()
                .clone()
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
            let ruck1 = elf1.chars().collect::<HashSet<char>>();
            let ruck2 = elf2.chars().collect::<HashSet<char>>();
            let ruck3 = elf3.chars().collect::<HashSet<char>>();
            (ruck1, ruck2, ruck3)
        })
        .map(|(ruck1, ruck2, ruck3)| {
            let mut base = ruck1.clone();
            base.retain(|e| ruck2.contains(e) && ruck3.contains(e));
            base.iter().nth(0).unwrap().clone()
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
