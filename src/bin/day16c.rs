use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
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
struct Valve<'a> {
    rate: usize,
    neighbours: HashMap<&'a str, usize>,
}

impl <'a> Valve<'a> {
    fn new(rate: usize, neighbours_raw: &str) -> Valve {
        Valve{
            rate,
            neighbours: neighbours_raw.split(", ").map(|x| (x, 1)).collect(),
        }
    }

    fn expanded(&self, graph: &HashMap<&str, Valve<'a>>, id: &str) -> Valve {
        let mut new_neighbours: HashMap<&str, usize> = self.neighbours.clone();
        loop {
            let mut neighbours_to_add: HashMap<&str, usize> = HashMap::new();
            for (n, cost) in &new_neighbours {
                graph.get(n)
                    .unwrap()
                    .neighbours
                    .iter()
                    .filter(|(n2, _)| **n2 != id && !new_neighbours.contains_key(*n2))
                    .for_each(|(n2, cost2)| {
                        neighbours_to_add.insert(n2, cost + cost2);
                    });
            }
            if neighbours_to_add.len() == 0 {
                break;
            }
            new_neighbours.extend(neighbours_to_add);
        }
        Valve{
            rate: self.rate,
            neighbours: new_neighbours,
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    let re = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let id = caps.get(1).unwrap().as_str();
            let valve = Valve::new(
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str()
            );
            (id, valve)
        })
        .collect()
}


fn reduce_graph<'a>(graph: &'a HashMap<&str, Valve>) -> HashMap<&'a str, Valve<'a>> {
    let mut graph = graph.iter()
        .map(|(id, valve)| (*id, valve.expanded(&graph, *id)))
        .collect::<HashMap<&str, Valve>>();

    let keys_to_del = graph.iter()
        .filter(|(id, v)| v.rate == 0 && **id != "AA") //because AA is starting point
        .map(|(id, _)| *id)
        .collect::<Vec<&str>>();

    graph.iter_mut()
        .for_each(|(_, v)| v.neighbours
            .retain(|id, _| !keys_to_del.contains(id) && *id != "AA")
        );

    graph.retain(|id, _| !keys_to_del.contains(id));
    graph
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
    let graph = reduce_graph(&graph);
    let budget = 30;

    graph.iter().for_each(|(k, v)| println!("{}: {:?}", k, v));

    let mut q: VecDeque<Vec<&str>> = VecDeque::new();
    q.push_front(vec!["AA"]);

    let mut searchspace: Vec<Vec<&str>> = Vec::new();

    while let Some(path) = q.pop_back() {
        let current_node = path.last().unwrap();
        let mut pushed = false;
        for (next_node, _) in &graph.get(current_node).unwrap().neighbours {
            if path.contains(&next_node) {
                continue
            }
            let mut next_path = path.clone();
            if !is_in_budget(&next_path, &graph, budget) {
                continue
            }
            next_path.push(next_node);
            q.push_front(next_path);
            pushed = true;
        }
        if !pushed {
            searchspace.push(path);
        }
    }

    // println!("{:?}", searchspace);
    println!("searchspace len: {:?}", searchspace.len());

    searchspace.iter()
        .map(|path| evaluate(path, &graph, budget))
        .max()
        .unwrap()
}

fn is_in_budget(path: &Vec<&str>, graph: &HashMap<&str, Valve>, max_budget: i32) -> bool {
    path.iter()
        .tuples()
        .map(|(current, next)| {
            1 + *graph.get(current).unwrap().neighbours.get(next).unwrap() as i32
        })
        .sum::<i32>() <= max_budget
}

fn evaluate(path: &Vec<&str>, graph: &HashMap<&str, Valve>, budget: i32) -> usize {
    let mut budget: i32 = budget;
    let mut current_node = "AA";
    let mut score: usize = 0;

    for destination in &path[1..] {
        let cost = 1 + graph.get(current_node).unwrap().neighbours.get(destination).unwrap();
        current_node = destination;
        budget -= cost as i32;
        score += budget as usize * graph.get(destination).unwrap().rate;
    }

    score
}

