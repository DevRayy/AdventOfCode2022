use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::slice::SliceIndex;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
        .expect("Unable to load input file");

    // let part1_start = Instant::now();
    // let part1_ans = part1(&input);
    // println!("Part 1 time: {:.2?}", part1_start.elapsed());
    // println!("Part 1 ans : {}", part1_ans);
    //
    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
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

    let mut q: VecDeque<(Vec<&str>, usize, usize)> = VecDeque::new();
    q.push_front((vec!["AA"], 0, 0));

    let mut scores: Vec<usize> = Vec::new();

    while let Some((path, path_cost, path_score)) = q.pop_back() {
        let current_node = path.last().unwrap();
        scores.push(path_score);

        graph.get(current_node)
            .unwrap()
            .neighbours
            .iter()
            .filter(|(next_node, next_cost)| {
                !path.contains(next_node) && *next_cost + path_cost + 1 < budget
            })
            .for_each(|(next_node, next_cost)| {
                let next_path_cost = next_cost + path_cost + 1;

                let next_score = (budget - next_path_cost as usize) * graph.get(next_node).unwrap().rate;
                let next_path_score = next_score + path_score;

                let mut next_path = path.clone();
                next_path.push(next_node);

                q.push_front((next_path, next_path_cost, next_path_score));
            })
    }

    *scores.iter().max().unwrap()
}

#[derive(Clone, Eq, PartialEq)]
struct State<'a> {
    path: Vec<&'a str>,
    cost: usize,
    score: usize,
}

impl <'a> State<'a> {
    fn new(path: Vec<&'a str>, cost: usize, score: usize) -> State {
        State {path, cost, score}
    }
}

fn part2(input: &str) -> usize {
    let graph = parse(input);
    let graph = reduce_graph(&graph);
    let budget = 26;

    // let mut q: VecDeque<State> = VecDeque::new();
    // q.push_front(State::new(vec!["AA"], 0, 0));
    let mut q: VecDeque<(State, State)> = VecDeque::new();
    q.push_front((State::new(vec!["AA"], 0, 0),
                  State::new(vec!["AA"], 0, 0)));

    // let mut scores: Vec<(State, State)> = Vec::new();
    let mut max_score: usize = 0;

    while let Some((state1, state2)) = q.pop_front() {
        // scores.push((state1.clone(), state2.clone()));
        let score = state1.score + state2.score;
        if score > max_score {
            max_score = score;
            println!("max score: {}", max_score);
        }

        for s in gen_new_states(&graph, budget, &state1, &state2) {
            q.push_front((s, state2.clone()));
        }
        for s in gen_new_states(&graph, budget, &state2, &state1) {
            q.push_front((state1.clone(), s));
        }
    }
    0
}

fn gen_new_states<'a>(graph: &'a HashMap<&str, Valve>, budget: usize, state: &State<'a>, state2: &State) -> Vec<State<'a>> {
    let current_node = state.path.last().unwrap();

    graph.get(current_node)
        .unwrap()
        .neighbours
        .iter()
        .filter(|(next_node, next_cost)| {
            !state.path.contains(next_node) && !state2.path.contains(next_node) && max(state.cost + *next_cost, state2.cost) + 1 < budget
        })
        .map(|(next_node, next_cost)| {
            let next_path_cost = next_cost + state.cost + 1;

            let next_score = (budget - next_path_cost as usize) * graph.get(next_node).unwrap().rate;
            let next_path_score = next_score + state.score;

            let mut next_path = state.path.clone();
            next_path.push(next_node);

            State::new(next_path, next_path_cost, next_path_score)
        })
        .collect()
}