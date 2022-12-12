use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day12.txt")
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

fn parse(input: &str) -> (HashMap<(i32, i32), i32>, (i32, i32), (i32, i32)) {
    let mut grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c as i32))
        })
        .flatten()
        .collect::<HashMap<(i32, i32), i32>>();

    let start = *grid.iter().find(|(pos, &c)| c == 'S' as i32).unwrap().0;
    grid.insert(start, 'a' as i32);
    let end = *grid.iter().find(|(pos, &c)| c == 'E' as i32).unwrap().0;
    grid.insert(end, 'z' as i32);

    (grid, start, end)
}

fn part1(input: &str) -> i32 {
    let (grid, start, end) = parse(input);

    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    let mut to_visit: VecDeque<(i32, i32)> = VecDeque::new();

    scores.insert(start, 0);
    to_visit.push_front(start);

    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let o = to_visit.pop_back().unwrap() {
        if o == end {
            return scores.get(&o).unwrap().clone()
        }

        for dir in dirs {
            let w = (o.0 + dir.0, o.1 + dir.1);
            if let Some(n) = grid.get(&w) {
                if !scores.contains_key(&w) && *grid.get(&o).unwrap() >= n - 1 {
                    to_visit.push_front(w);
                    scores.insert(w, scores.get(&o).unwrap() + 1);
                }
            }
        }
    };
    unreachable!();
}

fn part2(input: &str) -> i32 {
    let (grid, start, end) = parse(input);

    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    let mut to_visit: VecDeque<(i32, i32)> = VecDeque::new();

    scores.insert(start, 0);
    to_visit.push_front(start);

    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let o = to_visit.pop_back().unwrap() {
        if o == end {
            return scores.get(&o).unwrap().clone()
        }

        for dir in dirs {
            let w = (o.0 + dir.0, o.1 + dir.1);
            if let Some(n) = grid.get(&w) {
                if !scores.contains_key(&w) && *grid.get(&o).unwrap() >= n - 1 {
                    to_visit.push_front(w);
                    let new_score = if *n == 'a' as i32 {
                        0
                    } else {
                        scores.get(&o).unwrap() + 1
                    };
                    scores.insert(w, new_score);
                }
            }
        }
    };
    unreachable!();
}