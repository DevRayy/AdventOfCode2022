use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day21.txt")
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

struct Monke<'a> {
    number: Option<i64>,
    operation: Option<(&'a str, &'a str, &'a str)>
}

impl <'a> Monke<'a> {
    fn yell(&self, monkes: &HashMap<&str, Monke>) -> i64 {
        if self.number.is_some() {
            return self.number.unwrap()
        }

        let (monke1, op, monke2) = self.operation.unwrap();
        match op {
            "+" => monkes.get(monke1).unwrap().yell(monkes) + monkes.get(monke2).unwrap().yell(monkes),
            "-" => monkes.get(monke1).unwrap().yell(monkes) - monkes.get(monke2).unwrap().yell(monkes),
            "*" => monkes.get(monke1).unwrap().yell(monkes) * monkes.get(monke2).unwrap().yell(monkes),
            "/" => monkes.get(monke1).unwrap().yell(monkes) / monkes.get(monke2).unwrap().yell(monkes),
            _ => unreachable!()
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Monke> {
    input.split("\n")
        .map(|line| {
            let mut splitted = line.split(": ");
            let id = splitted.next().unwrap();
            let mut splitted2 = splitted.next().unwrap().split(" ");
            if splitted2.clone().count() == 1 {
                (id, Monke{number: Some(splitted2.nth(0).unwrap().parse().unwrap()), operation: None})
            } else {
                (id, Monke{number: None, operation: Some((
                    splitted2.next().unwrap(),
                    splitted2.next().unwrap(),
                    splitted2.next().unwrap()
                    ))})
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let monkes = parse(input);

    monkes.get("root").unwrap().yell(&monkes)
}