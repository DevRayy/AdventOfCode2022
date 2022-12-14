use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day17.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans : {}", part1_ans);
    assert_eq!(part1_ans, 3048);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans : {:.2?}", part2_ans);
    assert_eq!(part2_ans, 1504093567249);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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
}

struct Brick {
    template: Vec<Point>,
    origin: Point
}

impl Brick {
    fn new(t: Vec<Point>, x: usize, y: usize) -> Brick {
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
    let brick_templates: Vec<Vec<Point>> = vec![
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)],
        vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(1, 2)],
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)],
    ];

    solve_for_max_bricks(&valves, &brick_templates, 2022)
}

fn part2(input: &str) -> usize {
    let valves = input.trim_end().chars().collect::<Vec<char>>();
    let brick_templates: Vec<Vec<Point>> = vec![
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)],
        vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(1, 2)],
        vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
        vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)],
    ];

    let target_brick_count = 1000000000000 as usize;
    let checkpoint = valves.len();

    let (brick_count1, brick_count2, height1, height2) = solve_for_checkpoint(&valves, &brick_templates, checkpoint);

    let brick_count_diff = brick_count2 - brick_count1;
    let height_diff = height2 - height1;

    let checkpointable_height = (target_brick_count / brick_count_diff) * height_diff;

    let remaining_brick_count = target_brick_count % brick_count_diff;
    let remainder_height = solve_for_max_bricks(&valves, &brick_templates, remaining_brick_count);

    checkpointable_height + remainder_height
}

fn solve_for_max_bricks(valves: &Vec<char>, brick_templates: &Vec<Vec<Point>>, max_bricks: usize) -> usize {
    let mut board = Board::new();
    let mut valve_count: usize = 0;
    let mut brick_count: usize = 0;

    let mut brick = Brick::new(
        brick_templates[brick_count % brick_templates.len()].clone(),
        2,
        board.max_height() + 4
    );

    loop {
        simstep(&valves,
                &brick_templates,
                &mut board,
                &mut valve_count,
                &mut brick_count,
                &mut brick
        );

        if brick_count == max_bricks {
            return board.max_height()
        }
    }
}

fn solve_for_checkpoint(valves: &Vec<char>, brick_templates: &Vec<Vec<Point>>, checkpoint: usize) -> (usize, usize, usize, usize) {
    let mut board = Board::new();
    let mut valve_count: usize = 0;
    let mut brick_count: usize = 0;

    let mut brick = Brick::new(
        brick_templates[brick_count % brick_templates.len()].clone(),
        2,
        board.max_height() + 4
    );

    for _ in 0..checkpoint {
        simstep(&valves,
                &brick_templates,
                &mut board,
                &mut valve_count,
                &mut brick_count,
                &mut brick
        );
    }
    let brick_count1 = brick_count;
    let height1 = board.max_height();

    for _ in 0..checkpoint {
        simstep(&valves,
                &brick_templates,
                &mut board,
                &mut valve_count,
                &mut brick_count,
                &mut brick
        );
    }
    let brick_count2 = brick_count;
    let height2 = board.max_height();
    return (brick_count1, brick_count2, height1, height2)
}

fn simstep(valves: &Vec<char>,
           brick_templates: &&Vec<Vec<Point>>,
           board: &mut Board,
           valve_count: &mut usize,
           brick_count: &mut usize,
           brick: &mut Brick
) {
    let valve = valves[*valve_count % valves.len()];
    *valve_count += 1;

    match valve {
        '<' => { brick.push_left(&board) }
        '>' => { brick.push_right(&board) }
        _ => unreachable!()
    }

    let next_positions = brick.moved_by(Point::new(0, -1));
    let blocked = board.is_blocked(&next_positions);
    if !blocked {
        brick.fall();
        return
    }

    let mut materialized_block = brick.moved_by(Point::new(0, 0));
    board.materialize(&mut materialized_block);

    *brick_count += 1;
    *brick = Brick::new(
        brick_templates[*brick_count % brick_templates.len()].clone(),
        2,
        board.max_height() + 4
    );
}