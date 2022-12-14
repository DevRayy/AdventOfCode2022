use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day24.txt")
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

type Point = (i64, i64);

const DIR_UP:  Point = (-1, 0);
const DIR_DOWN:  Point = (1, 0);
const DIR_RIGHT:  Point = (0, 1);
const DIR_LEFT:  Point = (0, -1);
const DIR_NOTHING: Point = (0, 0);

const DIRS: [Point; 5] = [DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_NOTHING];

fn parse(input: &str) -> HashSet<(Point, Point)> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| vec!['^', '<', '>', 'v'].contains(c))
                .map(move |(j, c)| {
                    ((i as i64, j as i64), match c {
                        '^' => DIR_UP,
                        '>' => DIR_RIGHT,
                        '<' => DIR_LEFT,
                        'v' => DIR_DOWN,
                        _ => {unreachable!()}
                    })
                })
        })
        .flatten()
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut blizzards = parse(input);
    let start: Point = (0, 1);
    let end_x = blizzards.iter()
        .map(|(b, _)| b.0)
        .max()
        .unwrap() + 1;
    let end_y = blizzards.iter()
        .map(|(b, _)| b.1)
        .max()
        .unwrap();
    let end: Point = (end_x, end_y);

    solve(&mut blizzards, start, end)
}

fn part2(input: &str) -> i64 {
    let mut blizzards = parse(input);
    let start: Point = (0, 1);
    let end_x = blizzards.iter()
        .map(|(b, _)| b.0)
        .max()
        .unwrap() + 1;
    let end_y = blizzards.iter()
        .map(|(b, _)| b.1)
        .max()
        .unwrap();
    let end: Point = (end_x, end_y);

    solve(&mut blizzards, start, end) + solve(&mut blizzards, end, start) + solve(&mut blizzards, start, end)
}

fn solve(blizzards: &mut HashSet<(Point, Point)>, start: Point, end: Point) -> i64 {
    let mut positions: HashSet<Point> = HashSet::from([start]);
    let b_start = (min(start.0, end.0), min(start.1, end.1));
    let b_end = (max(start.0, end.0), max(start.1, end.1));
    let mut i = 0;
    loop {
        let (b, blizzards_set) = step_blizzards(&blizzards, b_start, b_end);
        *blizzards = b;
        positions = step_positions(&blizzards_set, &positions, start, end);
        i += 1;
        if positions.contains(&end) {
            break i
        }
    }
}

fn step_positions(blizzards: &HashSet<Point>, positions: &HashSet<Point>, start: Point, end: Point) -> HashSet<Point> {
    positions.iter()
        .cartesian_product(DIRS.iter())
        .map(|(p, d)| (p.0 + d.0, p.1 + d.1))
        .filter(|&p| {
            if p == start || p == end {
                return true
            };
            if p.0 == start.0 || p.0 == end.0 || p.0 == min(start.0, end.0) - 1 || p.0 == max(start.0, end.0) + 1{
                return false
            };
            if p.1 == min(start.1, end.1) - 1 || p.1 == max(start.1, end.1) + 1 {
                return false
            };
            return true
        })
        .filter(|p| {
            !blizzards.contains(p)
        })
        .collect()
}

fn step_blizzards(blizzards: &HashSet<(Point, Point)>, start: Point, end: Point) -> (HashSet<(Point, Point)>, HashSet<Point>) {
    let it = blizzards.iter()
        .map(|(p, dir)| {
            let mut new_point = (p.0 + dir.0, p.1 + dir.1);
            if new_point.0 == end.0 {
                new_point.0 = start.0 + 1
            }
            if new_point.0 == start.0 {
                new_point.0 = end.0 - 1
            }
            if new_point.1 == end.1 + 1 {
                new_point.1 = start.1
            }
            if new_point.1 == start.1 - 1 {
                new_point.1 = end.1
            }

            (new_point, *dir)
        });
    (it.clone().collect(), it.clone().map(|(b, _)| b).collect())
}
