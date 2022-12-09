use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day09.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn parse(input: &str) -> Vec<(char, usize)> {
    input.split("\n")
        .map(|line| {
            (line[0..1].chars().next().unwrap(), line[2..].parse::<usize>().unwrap())
        })
        .collect::<Vec<(char, usize)>>()
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut trail: Vec<(i32, i32)> = Vec::new();

    for (dir, count) in instructions {
        for _ in 0..count {
            move_head(&mut head, dir);
            move_tail(&mut tail, &head);
            trail.push(tail.clone());
        }
    }

    trail.iter().collect::<HashSet<_>>().len()
}

fn move_head(head: &mut (i32, i32), dir: char) {
    match dir {
        'R' => head.0 += 1,
        'L' => head.0 -= 1,
        'U' => head.1 += 1,
        'D' => head.1 -= 1,
        _ => unreachable!()
    }
}

fn move_tail(tail: &mut (i32, i32), head: &(i32, i32)) {
    let diff = (head.0 - tail.0, head.1 - tail.1);

    if diff.0 == 0 || diff.1 == 0 && (diff.0.abs() + diff.1.abs() == 2){ //move directly
        tail.0 += diff.0/2;
        tail.1 += diff.1/2;
    } else if diff.0.abs() + diff.1.abs() == 3 { //move diagonally
        tail.0 += diff.0.signum();
        tail.1 += diff.1.signum();
    }
}
