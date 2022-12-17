use std::cmp::{max, min};
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day17.txt")
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

#[derive(Copy, Clone)]
struct BrickTemplate {
    len: usize,
    top: [usize; 4],
    bottom: [usize; 4],
}

struct Brick<'a> {
    template: &'a BrickTemplate,
    pos_x: usize,
    pos_y: usize,
}

impl <'a>Brick<'a> {
    fn new(t :&BrickTemplate, x: usize, y: usize) -> Brick {
        Brick{
            template: t,
            pos_x: x,
            pos_y: y,
        }
    }

    fn push_left(&mut self, board: &[usize; 7]) {
        if self.pos_x == 0 {
            return
        }
        let l = self.render_bottom_left();
        let blocked = board.iter()
            .enumerate()
            .any(|(i, y)| *y == l[i]);
        if !blocked {
            self.pos_x = max(self.pos_x - 1, 0);
        }
    }

    fn push_right(&mut self, board: &[usize; 7]) {
        if self.pos_x == 7 - self.template.len {
            return
        }
        let r = self.render_bottom_right();
        let blocked = board.iter()
            .enumerate()
            .any(|(i, y)| *y == r[i]);
        if !blocked {
            self.pos_x = min(self.pos_x + 1, 7 - self.template.len);
        }
    }

    fn fall(&mut self) {
        self.pos_y -= 1;
    }

    fn top(&self) -> [Option<usize>; 7] {
        let mut top: [Option<usize>; 7] = [None, None, None, None, None, None, None];
        for x in 0..self.template.len {
            top[x + self.pos_x] = Some(self.template.top[x]);
        }
        for x in 0..top.len() {
            match top[x] {
                None => {},
                Some(y) => { top[x] = Some(y + self.pos_y)}
            }
        }

        top
    }

    fn render_bottom(&self) -> [usize; 7] {
        let mut bottom: [usize; 7] = [9, 9, 9, 9, 9, 9, 9];
        for x in 0..self.template.len {
            bottom[x + self.pos_x] = self.template.bottom[x];
        }
        for x in 0..bottom.len() {
            bottom[x] += self.pos_y;
        }

        bottom
    }

    fn render_bottom_right(&self) -> [usize; 7] {
        let mut bottom: [usize; 7] = [9, 9, 9, 9, 9, 9, 9];
        for x in 0..self.template.len {
            bottom[x + self.pos_x + 1] = self.template.bottom[x];
        }
        for x in 0..bottom.len() {
            bottom[x] += self.pos_y;
        }

        bottom
    }

    fn render_bottom_left(&self) -> [usize; 7] {
        let mut bottom: [usize; 7] = [9, 9, 9, 9, 9, 9, 9];
        for x in 0..self.template.len {
            bottom[x + self.pos_x - 1] = self.template.bottom[x];
        }
        for x in 0..bottom.len() {
            bottom[x] += self.pos_y;
        }

        bottom
    }

    fn render_next_bottom(&self) -> [usize; 7] {
        let mut bottom: [usize; 7] = [9, 9, 9, 9, 9, 9, 9];
        for x in 0..self.template.len {
            bottom[x + self.pos_x] = self.template.bottom[x];
        }
        for x in 0..bottom.len() {
            bottom[x] += (self.pos_y - 1);
        }

        bottom
    }
}

fn part1(input: &str) -> usize {
    let valves = input.trim_end().chars().collect::<Vec<char>>();
    let mut board: [usize; 7] = [0, 0, 0, 0, 0, 0, 0];
    let bricks: [BrickTemplate; 5] = [
      BrickTemplate {len: 4, top: [0, 0, 0, 0], bottom: [0, 0, 0, 0]},
      BrickTemplate {len: 3, top: [1, 2, 1, 0], bottom: [1, 0, 1, 0]},
      BrickTemplate {len: 3, top: [0, 0, 2, 0], bottom: [0, 0, 0, 0]},
      BrickTemplate {len: 1, top: [3, 0, 0, 0], bottom: [0, 0, 0, 0]},
      BrickTemplate {len: 2, top: [1, 1, 0, 0], bottom: [0, 0, 0, 0]},
    ];

    let mut valve_count: usize = 0;
    let mut brick_count: usize = 0;

    let mut brick = Brick::new(
        &bricks[brick_count % bricks.len()],
        2,
        board.iter().max().unwrap() + 4
    );

    loop {
        match valves[valve_count % valves.len()] {
            '<' => { brick.push_left(&board) }
            '>' => { brick.push_right(&board) }
            _ => unreachable!()
        }
        valve_count += 1;

        //FALL
        let next_floor = brick.render_next_bottom();

        let blocked = board.iter()
            .enumerate()
            .any(|(i, y)| *y == next_floor[i]);

        if !blocked {
            brick.fall();
            continue;
        }

        let top = brick.top();

        for x in 0..board.len() {
            if let Some(y) = top[x] {
                board[x] = y;
            }
        }

        brick_count += 1;
        brick = Brick::new(
            &bricks[brick_count % bricks.len()],
            2,
            board.iter().max().unwrap() + 4 //TODO maybe +4?
        );

        println!("{:?}", board);

        if brick_count == 2024 {
            return *board.iter().max().unwrap()
        }

    }
    unreachable!()
}