use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day20.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
}

fn parse(input: &str) -> Vec<(usize, i64)> {
    input.split("\n")
        .enumerate()
        .map(|(idx, line)| {
            (idx, line.parse::<i64>().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut numbers = parse(input);

    for i in 0..numbers.len() {
        let pos = numbers.iter().position(|(idx, _)| *idx==i).unwrap();

        let val = numbers[pos].1;
        numbers.remove(pos);
        let new_pos = (pos as i64 + val).rem_euclid(numbers.len() as i64) as usize;
        numbers.insert(new_pos, (i, val));
    }


    let zero_pos = numbers.iter().position(|&x| x.1 == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|&x| numbers[(zero_pos + x) % numbers.len()].1)
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut numbers = parse(input);
    let decryption_key: i64 = 811589153;
    numbers = numbers.into_iter().map(|(idx, n)| (idx, n * decryption_key)).collect();

    for _ in 0..10 {
        for i in 0..numbers.len() {
            let pos = numbers.iter().position(|(idx, _)| *idx == i).unwrap();

            let val = numbers[pos].1;
            numbers.remove(pos);
            let new_pos = (pos as i64 + val).rem_euclid(numbers.len() as i64) as usize;
            numbers.insert(new_pos, (i, val));
        }
    }


    let zero_pos = numbers.iter().position(|&x| x.1 == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|&x| numbers[(zero_pos + x) % numbers.len()].1)
        .sum()
}
