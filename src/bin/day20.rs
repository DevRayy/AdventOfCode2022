use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day20.txt")
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

fn parse(input: &str) -> Vec<i64> {
    input.split("\n")
        .map(|line| {
            line.parse::<i64>().unwrap()
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut numbers = parse(input);

    let order = ordering(&numbers);
    mix(&mut numbers, &order);

    let zero_pos = numbers.iter().position(|&x| x == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|&x| numbers[(zero_pos + x) % numbers.len()])
        .sum()
}

// fn part2(input: &str) -> i64 {
//     let mut numbers = parse(input);
//     let encryption_key = 811589153;
//     numbers = numbers.into_iter()
//         .map(|n| (n.0 * encryption_key, n.1))
//         .collect();
//
//     println!("{:?}", numbers);
//     for _ in 0..2 {
//         numbers = numbers.into_iter()
//             .map(|n| (n.0, n.0 != 0))
//             .collect();
//         mix(&mut numbers);
//         println!("{:?}", numbers);
//     }
//     let zero_pos = numbers.iter().position(|x| x.0 == 0).unwrap();
//     [1000, 2000, 3000].iter()
//         .map(|x| numbers[(zero_pos + x) % numbers.len()].0)
//         .sum()
// }

fn ordering(numbers: &Vec<i64>) -> Vec<(usize, usize)> {
    let mut order: Vec<(usize, usize)> = Vec::new();
    let mut numbers = numbers.iter()
        .map(|&n| (n, n!=0))
        .collect::<Vec<(i64, bool)>>();

    loop {
        for i in 0..numbers.len() {
            if numbers[i].1 == false {
                continue
            }

            let val = numbers[i].0;
            numbers.remove(i);
            let new_pos = (i as i64 + val).rem_euclid(numbers.len() as i64) as usize;
            numbers.insert(new_pos, (val, false));
            order.push((i, new_pos));
            break;
        }
        if numbers.iter().filter(|x| x.1 == false).count() == numbers.len() {
            break
        }
    }

    order
}

fn mix(numbers: &mut Vec<i64>, order: &Vec<(usize, usize)>) {
    for (i, j) in order {
        let val = numbers[*i];
        numbers.remove(*i);
        numbers.insert(*j, val);
    }
}
