use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day18.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans : {:.2?}", part2_ans);
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

    cubes.iter()
        .map(|cube| {
            6 - cubes.iter()
                .map(|cube2| manhattan(cube, cube2))
                .filter(|dist| *dist == 1)
                .count()
        })
        .sum()
}

fn manhattan(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}