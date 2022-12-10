use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day10.txt")
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

fn parse(input: &str) -> Vec<i32> {
    let mut adds: Vec<i32> = Vec::new();
    adds.push(1);
    input.split("\n")
        .for_each(|line| {
            adds.push(0);
            if line.starts_with("a") {
                adds.push(line[5..].parse::<i32>().unwrap())
            }
        });
    adds
}

fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    (20..instructions.len()).step_by(40)
        .map(|x| instructions[0..x].iter().sum::<i32>() * x as i32)
        .sum::<i32>()
}
