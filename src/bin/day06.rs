use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day06.txt")
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

fn part1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    chars.windows(4)
        .position(|chars| {
            !(1..chars.len()).any(|i| chars[i..].contains(&chars[i - 1]))
        }).unwrap() + 4
}

fn part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    chars.windows(14)
        .position(|chars| {
            !(1..chars.len()).any(|i| chars[i..].contains(&chars[i - 1]))
        }).unwrap() + 14
}
