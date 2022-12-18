use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::slice::SliceIndex;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day16.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);
    assert_eq!(part1_ans, 1728);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
    assert_eq!(part2_ans, 2304)
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

    let mut scores: Vec<State> = Vec::new();

    while let Some((path, path_cost, path_score)) = q.pop_back() {
        let current_node = path.last().unwrap();
        scores.push(State::new(path.clone(), path_cost, path_score));

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

    scores.into_iter()
        .map(|s| s.score)
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let graph = parse(input);
    let graph = reduce_graph(&graph);
    let budget = 26;

    let mut q: VecDeque<(Vec<&str>, usize, usize)> = VecDeque::new();
    q.push_front((vec!["AA"], 0, 0));

    let mut scores: Vec<State> = Vec::new();

    while let Some((path, path_cost, path_score)) = q.pop_back() {
        let current_node = path.last().unwrap();
        scores.push(State::new(path.clone(), path_cost, path_score));

        graph.get(current_node)
            .unwrap()
            .neighbours
            .iter()
            .filter(|(next_node, next_cost)| {
                !path.contains(next_node) && *next_cost + path_cost + 1 < budget
            })
            .for_each(|(next_node, next_cost)| {
                let mut next_path = path.clone();
                next_path.push(next_node);
                let next_path_cost = next_cost + path_cost + 1;

                let next_score = (budget - next_path_cost as usize) * graph.get(next_node).unwrap().rate;
                let next_path_score = next_score + path_score;

                q.push_front((next_path, next_path_cost, next_path_score));
            })
    }

    // let mut scores = scores.iter()
    //     .filter()
    //     .sorted_by(|a, b| a.1.cmp(&b.1))
    //     .collect::<Vec<_>>();

    // scores = scores.iter()
    //     .filter(|s| s.sco)
    let max_score = scores.iter()
        .map(|s| s.score)
        .max()
        .unwrap();
    scores = scores.into_iter()
        .filter(|s| s.score > max_score * 2 / 3) //FIXME magic numbers
        .collect();
    scores.sort_by(|a, b| a.score.cmp(&b.score).reverse());
    scores.iter()
        .permutations(2)
        .filter(|x| !x[0].path[1..].iter().any(|y| x[1].path.contains(y)))
        .map(|x| x[0].score + x[1].score)
        .max()
        .unwrap()
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
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