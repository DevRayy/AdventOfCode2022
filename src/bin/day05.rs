use std::fs;
use std::time::Instant;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day05.txt")
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

struct CrateMove {
    quantity: usize,
    from: usize,
    to: usize,
}

fn part1(input: &str) -> String {
    let splitted = input.split("\n\n");
    let mut towers = parse_towers(splitted.clone().nth(0).unwrap());
    let moves = parse_moves(splitted.clone().nth(1).unwrap());

    for m in moves {
        for _ in 0..m.quantity {
            let c = towers[m.from-1].pop().unwrap();
            towers[m.to-1].push(c);
        }
    }

    towers.iter()
        .map(|t| t.last().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let splitted = input.split("\n\n");
    let mut towers = parse_towers(splitted.clone().nth(0).unwrap());
    let moves = parse_moves(splitted.clone().nth(1).unwrap());

    for m in moves {
        let mut crane: Vec<char> = Vec::new();

        for _ in 0..m.quantity {
            let c = towers[m.from-1].pop().unwrap();
            crane.push(c);
        }

        for c in crane.iter().rev() {
            towers[m.to-1].push(*c)
        }
    }

    towers.iter()
        .map(|t| t.last().unwrap())
        .collect::<String>()
}

fn parse_towers(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.split("\n")
        .collect::<Vec<&str>>();
    let tower_count = lines.pop()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut towers: Vec<Vec<char>> = Vec::new();
    for i in 0..tower_count {
        let mut tower: Vec<char> = Vec::new();
        for line in lines.iter().rev() {
            match line.chars().nth(i*4 + 1) {
                None => continue,
                Some(c) => {
                    if !c.is_whitespace() {
                        tower.push(c);
                    }
                }
            }
        }
        towers.push(tower);
    }

    towers
}

fn parse_moves(input: &str) -> Vec<CrateMove> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            CrateMove {
                quantity: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap()
            }
        })
        .collect()
}