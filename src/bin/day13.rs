use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::Split;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day13.txt")
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

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>)
}

impl Packet {
    fn new(input: &str) -> Packet {
        if !input.starts_with("[") {
            return Packet::Integer(input.parse().unwrap());
        }

        let mut inner_packets = Vec::new();
        let mut depth = 0;
        let mut start = 1;
        for i in 1..input.len() - 1 {
            let c = input.chars().nth(i).unwrap();
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    if depth ==0 {
                        let inner = Packet::new(&input[start..i]);
                        inner_packets.push(inner);
                        start = i+1;
                    }
                }
                _ => {},
            }
        };
        if input.len() > 2 {
            inner_packets.push(Packet::new(&input[start..input.len()-1]))
        }
        Packet::List(inner_packets)
    }

    fn cmp(&self, rhs: &Self) -> Ordering {
        match (&self, rhs) {
            (Self::Integer(x), Self::Integer(y)) => {
                x.cmp(y)
            }
            (Self::List(x), Self::List(y)) => {
               for (xx, yy) in x.iter().zip(y.iter()) {
                   let r = xx.cmp(yy);
                   if r != Ordering::Equal {
                       return r
                   }
               }
                return x.len().cmp(&y.len());
            }
            (Self::Integer(x), Self::List(_)) => {
                Packet::List(vec![Packet::Integer(*x)]).cmp(rhs)
            }
            (Self::List(_), Self::Integer(y)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*y)]))
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Packet>>{
    input.split("\n\n")
        .map(|twoline| twoline.split("\n")
            .map(|line| Packet::new(line))
            .collect::<Vec<Packet>>()
        )
        .collect::<Vec<Vec<Packet>>>()
}

fn part1(input: &str) -> usize {
    let pairs = parse(input);

    pairs.iter()
        .map(|pair| pair[0].cmp(&pair[1]))
        .enumerate()
        .filter(|(_, ord)| *ord == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
}
