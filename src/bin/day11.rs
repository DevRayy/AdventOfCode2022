use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day11.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {}", part1_ans);

    // let part2_start = Instant::now();
    // part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
}

fn parse(input: &str) -> Vec<Monke> {
    input.split("\n\n")
        .map(|x| Monke::new(x))
        .collect()
}

fn part1(input: &str) -> usize {
    let mut monkes = parse(input);

    for _ in 0..20 {
        for i in 0..monkes.len() {
            let throws = monkes[i].turn();
            for (monke_idx, item) in throws {
                monkes[monke_idx].catch(item);
            }
        }
    }

    let mut activity = monkes.iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<usize>>();
    activity.sort();
    activity[activity.len()-1] * activity[activity.len()-2]
}

#[derive(Debug)]
struct Monke {
    items: Vec<u32>,
    operation_operator: char,
    operation_arg: Option<u32>,
    divisor: u32,
    target_true: usize,
    target_false: usize,

    inspection_count: usize,
}

impl Monke {
    fn new(input: &str) -> Monke {
        let mut splitted = input.split("\n");

        let items = splitted.nth(1).unwrap()[18..]
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let operation = &splitted.nth(0).unwrap()[23..];
        let operation_operator = operation[0..1].parse::<char>().unwrap();
        let operation_arg = match operation[2..].parse::<u32>() {
            Ok(x) => {Some(x)}
            Err(_) => {None}
        };
        let divisor = splitted.nth(0).unwrap()[21..].parse::<u32>().unwrap();
        let target_true = splitted.nth(0).unwrap()[29..].parse::<usize>().unwrap();
        let target_false = splitted.nth(0).unwrap()[30..].parse::<usize>().unwrap();

        Monke{
            items,
            operation_operator,
            operation_arg,
            divisor,
            target_true,
            target_false,
            inspection_count: 0,
        }
    }

    fn turn(&mut self) -> Vec<(usize, u32)> {
        let throws = (0..self.items.len())
            .map(|idx| {
                let item = self.inspect(self.items[idx]);
                let item = self.worry(item);
                let target = self.test(item);
                (target, item)
            })
            .collect::<Vec<(usize, u32)>>();
        self.items.clear();
        return throws;
    }

    fn inspect(&mut self, item: u32) -> u32 {
        self.inspection_count += 1;

        let arg = match self.operation_arg {
            None => {item}
            Some(x) => {x}
        };
        match self.operation_operator {
            '*' => item * arg,
            '+' => item + arg,
            _ => {unreachable!()}
        }
    }

    fn worry(&self, item: u32) -> u32 {
        (item as f64 / 3.0).floor() as u32
    }

    fn test(&self, item: u32) -> usize {
        if item % self.divisor == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }

    fn catch(&mut self, item: u32) {
        self.items.push(item);
    }
}
