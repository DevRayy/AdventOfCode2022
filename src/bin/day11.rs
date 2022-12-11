use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day11.txt")
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

fn parse(input: &str) -> Vec<Monke> {
    input.split("\n\n")
        .map(|x| Monke::new(x))
        .collect()
}

fn part1(input: &str) -> usize {
    let mut monkes = parse(input);

    solve(&mut monkes, 20, 3)
}

fn part2(input: &str) -> usize {
    let mut monkes = parse(input);

    solve(&mut monkes, 10000, 1)
}

fn solve(monkes: &mut Vec<Monke>, steps: usize, worry_coeff: u64) -> usize {
    let divisor_product = monkes.iter()
        .map(|m| m.divisor)
        .reduce(|a, b| a * b)
        .unwrap();


    for _ in 0..steps {
        for i in 0..monkes.len() {
            let throws = monkes[i].turn(divisor_product, worry_coeff);
            for (monke_idx, item) in throws {
                monkes[monke_idx].catch(item);
            }
        }
    }

    let mut activity = monkes.iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<usize>>();
    activity.sort();
    activity[activity.len() - 1] * activity[activity.len() - 2]
}

#[derive(Debug)]
struct Monke {
    items: Vec<u64>,
    operation_operator: char,
    operation_arg: Option<u64>,
    divisor: u64,
    target_true: usize,
    target_false: usize,

    inspection_count: usize,
}

impl Monke {
    fn new(input: &str) -> Monke {
        let mut splitted = input.split("\n");

        let items = splitted.nth(1).unwrap()[18..]
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let operation = &splitted.nth(0).unwrap()[23..];
        let operation_operator = operation[0..1].parse::<char>().unwrap();
        let operation_arg = match operation[2..].parse::<u64>() {
            Ok(x) => {Some(x)}
            Err(_) => {None}
        };
        let divisor = splitted.nth(0).unwrap()[21..].parse::<u64>().unwrap();
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

    fn turn(&mut self, divisor_product: u64, worry_coeff: u64) -> Vec<(usize, u64)> {
        let throws = (0..self.items.len())
            .map(|idx| {
                let item = self.inspect(self.items[idx]);
                let item = self.worry(item, divisor_product, worry_coeff);
                let target = self.test(item);
                (target, item)
            })
            .collect::<Vec<(usize, u64)>>();
        self.items.clear();
        return throws;
    }

    fn inspect(&mut self, item: u64) -> u64 {
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

    fn worry(&self, item: u64, divisor_product: u64, coeff: u64) -> u64 {
        (item % divisor_product) / coeff
    }

    fn test(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }

    fn catch(&mut self, item: u64) {
        self.items.push(item);
    }
}
