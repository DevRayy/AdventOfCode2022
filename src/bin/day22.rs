use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day22.txt")
        .expect("Unable to load input file");

    // let part1_start = Instant::now();
    // let part1_ans = part1(&input);
    // println!("Part 1 time: {:.2?}", part1_start.elapsed());
    // println!("Part 1 ans : {}", part1_ans);

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
    let mut numbers = input.clone()
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

    // for m in moves {
    //     match m {
    //         Move::Forward(x) => {
    //             for _ in 0..x {
    //                 let new_pos = wrap(&map, &pos, &dir);
    //
    //                 if *map.get(&new_pos).unwrap() == Tile::Wall {
    //                     break
    //                 }
    //                 pos = new_pos;
    //             }
    //         }
    //         Move::Rotate(d) => { match d {
    //             'R' => dir = (dir.1, -dir.0),
    //             'L' => dir = (-dir.1, dir.0),
    //             _ => unreachable!()
    //         }}
    //         Move::Nothing => unreachable!()
    //     }
    // }

    score(&pos, &dir)
}

fn wrap(map: &HashMap<(i64, i64), Tile>, pos: &(i64, i64), dir: &(i64, i64)) -> (i64, i64) {
    let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let block = map.get(&new_pos);

    if block.is_none() {
        match dir {
            (1, 0) => {
                new_pos = *map.iter()
                    .filter(|(k, _)| k.1 == pos.1)
                    .min_by(|a, b| a.0.0.cmp(&b.0.0))
                    .unwrap().0
            }
            (-1, 0) => {
                new_pos = *map.iter()
                    .filter(|(k, _)| k.1 == pos.1)
                    .max_by(|a, b| a.0.0.cmp(&b.0.0))
                    .unwrap().0
            }
            (0, 1) => {
                new_pos = *map.iter()
                    .filter(|(k, _)| k.0 == pos.0)
                    .min_by(|a, b| a.0.1.cmp(&b.0.1))
                    .unwrap().0
            }
            (0, -1) => {
                new_pos = *map.iter()
                    .filter(|(k, _)| k.0 == pos.0)
                    .max_by(|a, b| a.0.1.cmp(&b.0.1))
                    .unwrap().0
            }
            _ => {}
        }
    }
    new_pos
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