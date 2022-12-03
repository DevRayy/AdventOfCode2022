use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day03.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> i32 {
    input.split("\n")
        .into_iter()
        .map(|x| {
            let comp1 = x.chars()
                .take(x.len()/2)
                .collect::<HashSet<char>>();
            let comp2 = x.chars()
                .rev()
                .take(x.len()/2)
                .collect::<HashSet<char>>();
            let item = comp1.intersection(&comp2)
                .nth(0)
                .unwrap();

            if item.is_ascii_uppercase() {
                (*item as i8 - 38) as i32
            } else {
                (*item as i8 - 96) as i32
            }
        }).sum::<i32>()
}