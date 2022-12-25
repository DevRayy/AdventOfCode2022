use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day25.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);
}

fn parse(input: &str) -> Vec<&str> {
    input.split("\n")
        .collect()
}

fn part1(input: &str) -> String {
    let snafus = parse(input);

    let s = snafus.iter()
        .map(|s| snafu_to_dec(s))
        .sum();

    dec_to_snafu(s)
}

fn snafu_to_dec(snafu: &str) -> i64 {
    snafu.chars()
        .map(|c| match c {
            '1' => 1 as i64,
            '2' => 2 as i64,
            '0' => 0 as i64,
            '-' => -1 as i64,
            '=' => -2 as i64,
            _ => unreachable!()
        })
        .rev()
        .enumerate()
        .map(|(i, v)| 5_i64.pow(i as u32) * v)
        .sum()
}

fn dec_to_snafu(dec: i64) -> String {
    let mut snafu: Vec::<char> = Vec::new();
    let mut dec = dec;
    let mut add: i64 = 0;

    while dec > 0 || add != 0 {
        let reminder = (dec % 5) + add;
        add = if reminder > 2 {1} else {0};
        snafu.push(match reminder {
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            5 => '0',
            _ => unreachable!()
        });
        dec = dec / 5;
    }

    snafu.iter().rev().collect()
}