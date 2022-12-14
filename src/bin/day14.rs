use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day14.txt")
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

#[derive(PartialEq)]
enum Block {
    Rock,
    Water,
}

fn parse(input: &str) -> HashMap<(u64, u64), Block> {
    let paths = input.split("\n")
        .map(|line| line.split(" -> ")
            .map(|point| {
                let mut splitted = point.split(",");
                (splitted.next().unwrap().parse::<u64>().unwrap(),
                 splitted.next().unwrap().parse::<u64>().unwrap())
            })
            .collect::<Vec<(u64, u64)>>()
        )
        .collect::<Vec<Vec<(u64, u64)>>>();

    let mut blocks = HashMap::<(u64, u64), Block>::new();

    for path in paths {
        for pair in path.windows(2) {
            let x_from = min(pair[0].0, pair[1].0);
            let x_to = max(pair[0].0, pair[1].0);
            for x in x_from..x_to+1 {
                let y_from = min(pair[0].1, pair[1].1);
                let y_to = max(pair[0].1, pair[1].1);
                for y in y_from..y_to+1 {
                    blocks.insert((x, y), Block::Rock);
                }
            }
        }
    }

    blocks
}

fn part1(input: &str) -> usize {
    let mut blocks = parse(input);
    let origin = (500 as u64, 0 as u64);

    let abbys_level = blocks.keys()
        .map(|(x, y)| y)
        .max()
        .unwrap() + 1;

    'outer: loop {
        let mut sand = origin.clone();
        loop {
            if sand.1 >= abbys_level {
                break 'outer;
            }
            let new_sand = (sand.0, sand.1 + 1);
            if !blocks.contains_key(&new_sand) {
                sand = new_sand;
                continue;
            }

            let new_sand = (sand.0-1, sand.1 + 1);
            if !blocks.contains_key(&new_sand) {
                sand = new_sand;
                continue;
            }

            let new_sand = (sand.0 + 1, sand.1 + 1);
            if !blocks.contains_key(&new_sand) {
                sand = new_sand;
                continue;
            }

            blocks.insert(sand, Block::Water);
            break;
        }
    }

    blocks.values().filter(|&k| *k == Block::Water).count()
}
