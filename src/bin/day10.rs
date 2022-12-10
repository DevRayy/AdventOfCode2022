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
    // part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
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
    let cumsums = instructions.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    (20..instructions.len()).step_by(40)
        .map(|x| {
            cumsums[x-1] * x as i32
        })
        .sum::<i32>()
}

fn part2(input: &str) {
    let mut instructions = parse(input);
    instructions.iter_mut().fold(0, |acc, x| {
        *x += acc;
        *x
    });


}
