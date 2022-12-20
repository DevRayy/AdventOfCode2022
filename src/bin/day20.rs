use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day20.txt")
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

fn parse(input: &str) -> Vec<(i64, bool)> {
    input.split("\n")
        .map(|line| {
            let v = line.parse::<i64>().unwrap();
            (v, v != 0)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut numbers = parse(input);

    mix(&mut numbers);
    let zero_pos = numbers.iter().position(|x| x.0 == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|x| numbers[(zero_pos + x) % numbers.len()].0)
        .sum()
}

fn mix(numbers: &mut Vec<(i64, bool)>) {
    loop {
        for i in 0..numbers.len() {
            if numbers[i].1 == false {
                continue
            }

            let val = numbers[i].0;
            numbers.remove(i);
            let new_pos = (i as i64 + val).rem_euclid(numbers.len() as i64) as usize;
            numbers.insert(new_pos, (val, false));
            break;
        }
        if numbers.iter().filter(|x| x.1 == false).count() == numbers.len() {
            break
        }
    }
}
