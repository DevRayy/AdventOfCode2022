use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day07.txt")
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

fn part1(input: &str) -> usize {
    let mut dir: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<&str> = Vec::new();

    for line in input.split("\n") {
        let line = line.strip_prefix("$ ").unwrap_or(line);
        let mut splitted = line.split(" ");
        match splitted.next().unwrap() {
            "cd" => {
                match splitted.next().unwrap() {
                    "/" => current_path = Vec::new(),
                    ".." => {current_path.pop();},
                    d => current_path.push(d)
                };
            },
            "ls" => {},
            "dir" => {},
            filesize => {
                for i in 0..current_path.len()+1 {
                    let path = current_path[..i].iter().join("/");
                    let count = dir.entry(path).or_insert(0);
                    *count += filesize.parse::<usize>().unwrap();
                }
            },
        }
    };
    dir.iter()
        .filter(|(_, &v)| v <= 100000)
        .map(|(_, &v)| v)
        .sum::<usize>()
}