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
    assert_eq!(part1_ans, 83056452926300);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
    assert_eq!(part2_ans, 3469704905529);
}

#[derive(Debug)]
struct Monke<'a> {
    number: Option<i64>,
    operation: Option<(&'a str, &'a str, &'a str)>,
    depends_on_human: bool,
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

const ROOT: &str = "root";
const HUMAN: &str = "humn";

fn parse(input: &str) -> HashMap<&str, Monke> {
    input.split("\n")
        .map(|line| {
            let mut splitted = line.split(": ");
            let id = splitted.next().unwrap();
            let mut splitted2 = splitted.next().unwrap().split(" ");
            if splitted2.clone().count() == 1 {
                (id, Monke{number: Some(splitted2.nth(0).unwrap().parse().unwrap()), operation: None, depends_on_human: id == HUMAN})
            } else {
                (id, Monke{number: None, operation: Some((
                    splitted2.next().unwrap(),
                    splitted2.next().unwrap(),
                    splitted2.next().unwrap()
                    )), depends_on_human: id == HUMAN})
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let monkes = parse(input);

    monkes.get(ROOT).unwrap().yell(&monkes)
}

fn part2(input: &str) -> i64 {
    let mut monkes = parse(input);
    fill_human_role(&mut monkes);

    let (should_equal, root) = find_human_branch(&monkes);
    find_human_value(&monkes, should_equal, root)
}

fn find_human_value(monkes: &HashMap<&str, Monke>, should_equal: i64, starting_monke: &Monke) -> i64 {
    let mut should_equal = should_equal;
    let mut root = starting_monke;
    loop {
        if root.operation.is_none() {
            return should_equal
        }
        let left = monkes.get(root.operation.unwrap().0).unwrap();
        let right = monkes.get(root.operation.unwrap().2).unwrap();
        match (root.operation.unwrap().1, left.depends_on_human) {
            ("+", true) => {
                should_equal = should_equal - right.yell(&monkes);
                root = left
            }
            ("+", false) => {
                should_equal = should_equal - left.yell(&monkes);
                root = right
            }
            ("-", true) => {
                should_equal = should_equal + right.yell(&monkes);
                root = left
            }
            ("-", false) => {
                should_equal = left.yell(&monkes) - should_equal;
                root = right
            }
            ("*", true) => {
                should_equal = should_equal / right.yell(&monkes);
                root = left
            }
            ("*", false) => {
                should_equal = should_equal / left.yell(&monkes);
                root = right
            }
            ("/", true) => {
                should_equal = should_equal * right.yell(&monkes);
                root = left
            }
            ("/", false) => {
                should_equal = left.yell(&monkes) / should_equal;
                root = right
            }
            (_, _) => { unreachable!() }
        }
    }
}

fn find_human_branch<'a>(monkes: &'a HashMap<&str, Monke>) -> (i64, &'a Monke<'a>) {
    let root = monkes.get("root").unwrap();
    let mut left = monkes.get(root.operation.unwrap().0).unwrap(); // depends on human
    let mut right = monkes.get(root.operation.unwrap().2).unwrap();

    if right.depends_on_human {
        (left, right) = (right, left);
    }

    (right.yell(&monkes), left)
}

fn fill_human_role(monkes: &mut HashMap<&str, Monke>) {
    let mut curr = HUMAN;
    while curr != ROOT {
        curr = monkes.iter().find(|(_, m)| {
            match m.operation {
                None => { false }
                Some(x) => { x.0 == curr || x.2 == curr }
            }
        }).unwrap().0;
        monkes.get_mut(curr).unwrap().depends_on_human = true;
    }
}