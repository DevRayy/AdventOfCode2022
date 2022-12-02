use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day02.txt")
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

fn part2(input: &str) -> i32 {
    // A - 1, rock
    // B - 2, paper
    // C - 3, scissors

    // X - lose
    // Y - draw
    // Z - win

    input.split("\n")
        .into_iter()
        .map(|x| match x {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => {println!("{}", x); unreachable!()}
        })
        .sum::<i32>()
}

fn part1(input: &str) -> i32 {
    // A, X - rock
    // B, Y - paper
    // C, Z - scissors
    input.split("\n")
        .into_iter()
        .map(|x| match x {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => {println!("{}", x); unreachable!()}
        })
        .sum::<i32>()
}
