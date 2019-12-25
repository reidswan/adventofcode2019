use common::int_code_machine::{Machine, Status};
use common::iter_tools::*;
use std::fmt;
use std::io::stdin;
use std::thread::sleep_ms;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
    Score(i128),
}

#[derive(Copy, Clone, Debug)]
enum Input {
    Left,
    Right,
    Neutral,
}

impl Input {
    fn get_input() -> Self {
        let mut input_line = get_line().trim().to_ascii_lowercase();
        if input_line.is_empty() {
            Input::Neutral
        } else {
            Input::from_char(input_line.remove(0))
        }
    }

    fn to_i128(&self) -> i128 {
        match self {
            Input::Left => -1,
            Input::Right => 1,
            Input::Neutral => 0,
        }
    }

    fn from_char(src: char) -> Self {
        match src {
            'a' => Input::Left,
            'd' => Input::Right,
            _ => Input::Neutral,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;
        write!(
            f,
            "{}",
            match self {
                Empty => " ",
                Wall => "|",
                Block => "#",
                HorizontalPaddle => "_",
                Ball => "o",
                Score(i) => panic!("THE SCORE IS {} AHHHHHHHHH", i),
            }
        )
    }
}

impl Tile {
    fn from_i128(src: i128) -> Self {
        use Tile::*;
        match src {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            i => Score(i),
        }
    }

    fn to_i128(&self) -> i128 {
        use Tile::*;
        match self {
            Empty => 0,
            Wall => 1,
            Block => 2,
            HorizontalPaddle => 3,
            Ball => 4,
            Score(i) => *i,
        }
    }
}

pub fn get_parsed_input() -> String {
    String::from(include_str!("input/input"))
}

pub fn part1(src: &String) {
    let mut machine = Machine::new(src, vec![]);

    machine.run();

    let shown_tiles = machine
        .output
        .iter()
        .group(3)
        .filter(|group| match group {
            GroupedItem::Complete(group) => *group[2] == Tile::Block.to_i128(),
            _ => panic!("Incomplete tile output!"),
        })
        .count();
    println!("Part 1 = {}", shown_tiles);
}

type Coords = (usize, usize);

struct ArcadeGame {
    dimensions: Coords,
    canvas: Vec<Tile>,
    score: i128,
    paddle_coords: Coords,
    ball_coords: Coords,
}

impl ArcadeGame {
    fn new(row_len: usize, col_len: usize, tiles: Vec<(usize, usize, Tile)>) -> Self {
        let mut canvas = vec![Tile::Empty; (row_len + 1) * (col_len + 1)];
        let mut ball = None;
        let mut paddle = None;
        tiles.iter().for_each(|&(x, y, tile)| {
            canvas[Self::coord_to_index_with_size(x, y, row_len)] = tile;
            if let Tile::Ball = tile {
                ball = Some((x, y));
            } else if let Tile::HorizontalPaddle = tile {
                paddle = Some((x, y));
            };
        });

        ArcadeGame {
            dimensions: (row_len, col_len),
            canvas,
            score: 0,
            paddle_coords: paddle.unwrap(),
            ball_coords: ball.unwrap(),
        }
    }

    fn coord_to_index_with_size(x: usize, y: usize, row_length: usize) -> usize {
        row_length * y + x
    }

    fn coord_to_index(&self, x: usize, y: usize) -> usize {
        Self::coord_to_index_with_size(x, y, self.dimensions.0)
    }

    fn set_tile(&mut self, coord: (usize, usize), tile: Tile) {
        let index = self.coord_to_index(coord.0, coord.1);
        self.canvas[index] = tile;

        if let Tile::Ball = tile {
            self.ball_coords = coord
        } else if let Tile::HorizontalPaddle = tile {
            self.paddle_coords = coord
        }
    }

    fn render(&self) {
        println!(
            "{{--------------- SCORE: {} -----------------}}",
            self.score
        );
        for (i, tile) in self.canvas.iter().enumerate() {
            if i % self.dimensions.0 == 0 {
                println!();
            }
            print!("{}", tile);
        }
        println!();
    }

    fn determine_input(&self) -> Input {
        if self.ball_coords.0 < self.paddle_coords.0 {
            Input::Left
        } else if self.ball_coords.0 > self.paddle_coords.0 {
            Input::Right
        } else {
            Input::Neutral
        }
    }
}

fn get_line() -> String {
    let mut s = String::new();
    let stdin = stdin();
    stdin.read_line(&mut s).expect("Could not read input!");
    s
}

pub fn part2(src: &String) {
    let mut machine = Machine::new(src, vec![]);
    machine.memory[0] = 2;
    machine.wait_on_input();

    // get the initial canvas
    machine.run();
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut score = 0;
    let tiles = machine
        .output
        .iter()
        .group(3)
        .filter_map(|group| {
            match group {
                GroupedItem::Complete(group) => {
                    if *group[0] == -1 {
                        // score tile
                        score = *group[1];
                        return None;
                    }
                    let res = (
                        *group[0] as usize,
                        *group[1] as usize,
                        Tile::from_i128(*group[2]),
                    );
                    if res.0 > max_x {
                        max_x = res.0
                    };
                    if res.1 > max_y {
                        max_y = res.1
                    };
                    Some(res)
                }
                _ => panic!("Incomplete output!"),
            }
        })
        .collect::<Vec<_>>();
    machine.output = vec![];
    let mut game = ArcadeGame::new(max_x, max_y, tiles);
    game.score = score;

    machine.add_input(game.determine_input().to_i128());
    while let Status::Waiting = machine.run() {
        process_output(&mut machine, &mut game);
        machine.add_input(game.determine_input().to_i128());
    }
    process_output(&mut machine, &mut game);
    println!("Part 2 = {}", game.score);
}

fn process_output(machine: &mut Machine, game: &mut ArcadeGame) {
    for group in machine.output.iter().group(3) {
        match group {
            GroupedItem::Complete(group) => {
                if *group[0] == -1 {
                    game.score = *group[2]
                } else {
                    game.set_tile(
                        (*group[0] as usize, *group[1] as usize),
                        Tile::from_i128(*group[2]),
                    );
                }
            }
            _ => panic!("BAD OUTPUT!"),
        }
    }
    machine.output.clear();
}
