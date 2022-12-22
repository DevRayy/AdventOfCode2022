use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day22.txt")
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

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
    Nothing
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Forward(i64),
    Rotate(char),
    Nothing
}

const DIR_UP: (i64, i64) = (-1, 0);
const DIR_DOWN: (i64, i64) = (1, 0);
const DIR_LEFT: (i64, i64) = (0, -1);
const DIR_RIGHT: (i64, i64) = (0, 1);

fn parse(input: &str) -> (HashMap<(i64, i64), Tile>, Vec<Move>) {
    let mut splitted = input.split("\n\n");
    (parse_map(splitted.next().unwrap()), parse_instructions(splitted.next().unwrap()))
}

fn parse_map(input: &str) -> HashMap<(i64, i64), Tile> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| {
                    ((i as i64, j as i64), match c {
                        '.' => Tile::Open,
                        '#' => Tile::Wall,
                        _ => Tile::Nothing,
                    })
                })
        })
        .flatten()
        .filter(|(_, tile)| *tile != Tile::Nothing)
        .collect()
}

fn parse_instructions(input: &str) -> Vec<Move> {
    let numbers = input.clone()
        .replace("R", " ")
        .replace("L", " ");
    let mut numbers = numbers.split(" ")
        .map(|s| Move::Forward(s.parse::<i64>().unwrap()));
    let mut rotations = input.chars()
        .map(|c| match c {
            'R' => Move::Rotate('R'),
            'L' => Move::Rotate('L'),
            _ => Move::Nothing,
        })
        .filter(|m| *m != Move::Nothing);
    let mut moves: Vec<Move> = Vec::new();
    while let Some(r) = rotations.next() {
        moves.push(numbers.next().unwrap());
        moves.push(r);
    };
    moves.push(numbers.next().unwrap());
    moves
}

fn part1(input: &str) -> i64 {
    let (map, moves) = parse(input);

    let min_y = map.iter()
        .filter(|(k, _)| k.0 == 0)
        .map(|(k, _)| k.1)
        .min()
        .unwrap();

    let mut pos = (0 as i64, min_y);
    let mut dir = (0 as i64, 1 as i64);

    for m in moves {
        match m {
            Move::Forward(x) => {
                for _ in 0..x {
                    let new_pos = wrap(&map, &pos, &dir);

                    if *map.get(&new_pos).unwrap() == Tile::Wall {
                        break
                    }
                    pos = new_pos;
                }
            }
            Move::Rotate(d) => { match d {
                'R' => dir = (dir.1, -dir.0),
                'L' => dir = (-dir.1, dir.0),
                _ => unreachable!()
            }}
            Move::Nothing => unreachable!()
        }
    }

    score(&pos, &dir)
}

fn part2(input: &str) -> i64 {
    let (map, moves) = parse(input);

    let min_y = map.iter()
        .filter(|(k, _)| k.0 == 0)
        .map(|(k, _)| k.1)
        .min()
        .unwrap();

    let mut pos = (0 as i64, min_y);
    let mut dir = (0 as i64, 1 as i64);

    for m in moves {
        match m {
            Move::Forward(x) => {
                for _ in 0..x {
                    let (new_dir, new_pos) = wrap3d(&map, &pos, &dir);

                    if *map.get(&new_pos).unwrap() == Tile::Wall {
                        break
                    }
                    pos = new_pos;
                    dir = new_dir;
                }
            }
            Move::Rotate(d) => { match d {
                'R' => dir = (dir.1, -dir.0),
                'L' => dir = (-dir.1, dir.0),
                _ => unreachable!()
            }}
            Move::Nothing => unreachable!()
        }
    }

    score(&pos, &dir)
}

fn wrap3d(map: &HashMap<(i64, i64), Tile>, pos: &(i64, i64), dir: &(i64, i64)) -> ((i64, i64), (i64, i64)) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let new_dir = (dir.0, dir.1);
    let block = map.get(&new_pos);

    if block.is_some() {
        return (new_dir, new_pos)
    }

    let face = face_from_point(pos, 50);

    match (face, *dir) {
        ((0, 1), DIR_UP)    => (DIR_RIGHT, (pos.1 + 100, 0)),
        ((3, 0), DIR_LEFT)  => (DIR_DOWN, (0, pos.0 - 100)),

        ((0, 1), DIR_LEFT)  => (DIR_RIGHT, (149 - pos.0, 0)),
        ((2, 0), DIR_LEFT)  => (DIR_RIGHT, (149 - pos.0, 50)),

        ((0, 2), DIR_UP)    => (DIR_UP, (199, pos.1 - 100)),
        ((3, 0), DIR_DOWN)  => (DIR_DOWN, (0, 100 - pos.1)),

        ((1, 1), DIR_LEFT)  => (DIR_DOWN, (100, pos.0 - 50)),
        ((2, 0), DIR_UP)    => (DIR_RIGHT, (pos.1 + 50, 50)),

        ((2, 1), DIR_DOWN)  => (DIR_RIGHT, (pos.1 + 100, 49)),
        ((3, 0), DIR_RIGHT) => (DIR_UP, (149, pos.0 - 100)),

        ((0, 2), DIR_RIGHT) => (DIR_LEFT, (149 - pos.0, 99)),
        ((2, 1), DIR_RIGHT) => (DIR_LEFT, (149 - pos.0, 149)),

        ((0, 2), DIR_DOWN)  => (DIR_LEFT, (pos.1 - 50, 99)),
        ((1, 1), DIR_RIGHT) => (DIR_UP, (49, pos.0 + 50)),

        (_, _) => unreachable!()
    }
}

fn wrap(map: &HashMap<(i64, i64), Tile>, pos: &(i64, i64), dir: &(i64, i64)) -> (i64, i64) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let block = map.get(&new_pos);

    if block.is_some() {
        return new_pos;
    }

    match *dir {
        DIR_DOWN => {
            *map.iter()
                .filter(|(k, _)| k.1 == pos.1)
                .min_by(|a, b| a.0.0.cmp(&b.0.0))
                .unwrap().0
        }
        DIR_UP => {
            *map.iter()
                .filter(|(k, _)| k.1 == pos.1)
                .max_by(|a, b| a.0.0.cmp(&b.0.0))
                .unwrap().0
        }
        DIR_RIGHT => {
            *map.iter()
                .filter(|(k, _)| k.0 == pos.0)
                .min_by(|a, b| a.0.1.cmp(&b.0.1))
                .unwrap().0
        }
        DIR_LEFT => {
            *map.iter()
                .filter(|(k, _)| k.0 == pos.0)
                .max_by(|a, b| a.0.1.cmp(&b.0.1))
                .unwrap().0
        }
        _ => {unreachable!()}
    }
}

fn score(pos: &(i64, i64), dir: &(i64, i64)) -> i64 {
    let dir_score = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!()
    };

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir_score
}

fn face_from_point(p: &(i64, i64), size: i64) -> (i64, i64) {
    (p.0 / size, p.1 / size)
}