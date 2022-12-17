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

#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point{x, y}
    }

    fn add(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

struct Board {
    inner: Vec<Point>
}

impl Board {
    fn new() -> Board {
        Board{
            inner: vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(6, 0),
            ]
        }
    }

    fn max_height(&self) -> usize {
        self.inner.iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(0) as usize
    }

    fn is_blocked(&self, test: &Vec<Point>) -> bool {
        self.inner.iter()
            .any(|p| test.contains(p))
    }

    fn materialize(&mut self, b: &mut Vec<Point>) {
        self.inner.append(b)
    }

    fn print(&self) {
        println!("");
        for y in (0..self.max_height() as i64 + 1).rev() {
            for x in 0..7 {
                let p = Point::new(x, y);
                print!("{}", if self.inner.contains(&p) {
                    "#"
                } else {
                    "."
                });
            }
            print!("\n");
        }
    }
}

struct Brick<'a> {
    template: &'a Vec<Point>,
    origin: Point
}

impl <'a> Brick<'a> {
    fn new(t: &Vec<Point>, x: usize, y: usize) -> Brick {
        Brick {
            template: t,
            origin: Point::new(x as i64, y as i64),
        }
    }

    fn push_left(&mut self, board: &Board) {
        if self.origin.x == 0 {
            return
        }
        let move_vector = Point::new(-1, 0);
        let moved = self.moved_by(move_vector);
        let move_vector = Point::new(-1, 0);
        if !board.is_blocked(&moved) {
            self.origin.add(&move_vector);
        }
    }

    fn push_right(&mut self, board: &Board) {
        let template_len = self.template.iter()
            .map(|p| p.x)
            .max()
            .unwrap() + 1;
        if self.origin.x == 7 - template_len {
            return
        }
        let move_vector = Point::new(1, 0);
        let moved = self.moved_by(move_vector);
        let move_vector = Point::new(1, 0);
        if !board.is_blocked(&moved) {
            self.origin.add(&move_vector);
        }
    }

    fn fall(&mut self) {
        let move_vector = Point::new(0, -1);
        self.origin.add(&move_vector);
    }

    fn moved_by(&self, v: Point) -> Vec<Point> {
        self.template.iter()
            .map(|p| Point::new(p.x + v.x + self.origin.x, p.y + v.y + self.origin.y))
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let valves = input.trim_end().chars().collect::<Vec<char>>();
    let mut board = Board::new();
    let brick_templates: Vec<Vec<Point>> = vec![
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)],
        vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(1, 2)],
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)],
    ];
    let mut valve_count: usize = 0;
    let mut brick_count: usize = 0;

    let mut brick = Brick::new(
        &brick_templates[brick_count % brick_templates.len()],
        2,
        board.max_height() + 4
    );

    loop {
        let valve = valves[valve_count % valves.len()];
        valve_count += 1;

        match valve {
            '<' => { brick.push_left(&board) }
            '>' => { brick.push_right(&board) }
            _ => unreachable!()
        }

        let next_positions = brick.moved_by(Point::new(0, -1));
        let blocked = board.is_blocked(&next_positions);
        if !blocked {
            brick.fall();
            continue
        }

        let mut materialized_block = brick.moved_by(Point::new(0, 0));
        board.materialize(&mut materialized_block);

        brick_count += 1;
        brick = Brick::new(
            &brick_templates[brick_count % brick_templates.len()],
            2,
            board.max_height() + 4
        );

        if brick_count == 2022 {
            return board.max_height()
        }
    }

    unreachable!()
}