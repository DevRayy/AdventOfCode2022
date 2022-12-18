use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;
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

fn part1(input: &str) -> usize {
    let graph = parse(input);

    let graph = reduce_graph(&graph);

    let destinations = graph.iter()
        .filter(|(k, v)| **k != "AA")
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>();

    let mut max_score: usize = 0;
    for no_of_permutations in 1..destinations.len()+1 {
        println!("permutation len = {}", no_of_permutations);

        for combination in destinations.iter().cloned().permutations(no_of_permutations) {
            if let Some(score) = evaluate_combination(&combination, &graph) {
                if score > max_score {
                    max_score = score;
                    println!("new max {}", max_score);
                }
            }
        }
    }

    0
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
            .retain(|id, _| !keys_to_del.contains(id))
        );

    graph.retain(|id, _| !keys_to_del.contains(id));
    graph
}

fn evaluate_combination(path: &Vec<&str>, graph: &HashMap<&str, Valve>) -> Option<usize> {
    let mut budget: i32 = 30;
    let mut current_node = "AA";
    let mut score: usize = 0;

    for destination in path {
        let cost = 1 + graph.get(current_node).expect("1").neighbours.get(destination).expect("2");
        current_node = destination;
        budget -= cost as i32;
        if budget < 0 {
            return None
        }
        score += budget as usize * graph.get(destination).unwrap().rate;
    }

    Some(score)
}