use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day04.txt")
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
            x.split(|c| c == ',' || c == '-')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| {
            (x[0] >= x[2] && x[1] <= x[3]) ||
                (x[2] >= x[0] && x[3] <= x[1])
        })
        .count() as i32
}

fn part2(input: &str) -> i32 {
    input.split("\n")
        .into_iter()
        .map(|x| {
            x.split(|c| c == ',' || c == '-')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| {
            !(x[1] < x[2] || x[0] > x[3])
        })
        .count() as i32
}