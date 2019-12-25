use common::int_code_machine::Machine;
use std::collections::HashSet;

pub fn get_parsed_input()-> String {
    String::from(include_str!("input/input"))
}

pub fn part1(input: &String) {
    let panels = run_robot(input, PanelColor::Black);

    println!("Part 1: {}", panels.painted_panels.len());
}

pub fn part2(input: &String) {
    let panels = run_robot(input, PanelColor::White);

    println!("Part 2:");
    render(&panels.white_panels, panels.canvas_limits);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PanelColor {
    Black, White
}

impl PanelColor {
    fn from_i128(src: i128)-> Self {
        match src {
            0 => PanelColor::Black,
            1 => PanelColor::White, 
            _ => panic!("Invalid PanelColor numeral: {}", src)
        }
    }
    
    fn to_i128(&self)-> i128 {
        match self {
            Self::Black => 0,
            Self::White => 1    
        }
    }
}

enum RobotFacing {
    Up,
    Down,
    Left,
    Right,
}

impl RobotFacing {
    fn translate(&self, from: (i128, i128)) -> (i128, i128) {
        use RobotFacing::*;
        let (x, y) = from;
        match self {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }

    fn to_i8(&self) -> i8 {
        use RobotFacing::*;
        match self {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }

    fn from_i8(src: i8) -> Self {
        use RobotFacing::*;
        match src {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("Invalid direction number: {}", src),
        }
    }

    fn rotate(&self, direction: i128) -> Self {
        if direction != 0 && direction != 1 {
            panic!("Invalid direction: {}", direction)
        }
        let direction = if direction == 0 { -1 } else { 1 };
        // we +4 here to maintain positivity
        // e.g. (0 - 1) % 4 -> -1 (cross)
        // (0 - 1 + 4) % 4 -> 3 (tick)
        let new_facing = (self.to_i8() + direction + 4) % 4;
        Self::from_i8(new_facing)
    }
}

#[derive(Debug)]
struct CanvasLimits {
    x_min: i128,
    x_max: i128, 
    y_min: i128,
    y_max: i128
}

impl CanvasLimits {
    fn new() -> Self {
        CanvasLimits {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0
        }
    }

    fn update(&mut self, robot_position: (i128, i128)) {
        let (x, y) = robot_position;
        
        if x < 0 && x < self.x_min {
            self.x_min = x
        } else if x > 0 && x > self.x_max {
            self.x_max = x
        }

        if y < 0 && y < self.y_min {
            self.y_min = y
        } else if y > 0 && y > self.y_max {
            self.y_max = y
        }
    }
}

struct PaintedPanelDescription {
    white_panels: HashSet<(i128, i128)>,
    painted_panels: HashSet<(i128, i128)>,
    canvas_limits: CanvasLimits
}

impl PaintedPanelDescription {
    fn new()-> Self {
        PaintedPanelDescription {
            white_panels: HashSet::new(),
            painted_panels: HashSet::new(),
            canvas_limits: CanvasLimits::new()
        }
    }
}

fn run_robot(input: &str, start_color: PanelColor)-> PaintedPanelDescription {
    let mut panels = PaintedPanelDescription::new();
    if &start_color == &PanelColor::White {
        panels.white_panels.insert((0,0));
    }
    let mut robot = Machine::new(input, vec![start_color.to_i128()]);
    robot.wait_on_input();
    let mut robot_position = (0i128, 0i128);
    let mut current_facing = RobotFacing::Up;
    robot.run();
    while !robot.output.is_empty() {
        if robot.output.len() < 2 {
            panic!("Robot didn't output enough actions!")
        }
        let direction = robot.output.pop().unwrap();
        let color = PanelColor::from_i128(robot.output.pop().unwrap());
        match (color, panels.white_panels.contains(&robot_position)) {
            (PanelColor::Black, true) => {
                panels.white_panels.remove(&robot_position);
                panels.painted_panels.insert(robot_position);
            }
            (PanelColor::White, false) => {
                panels.white_panels.insert(robot_position);
                panels.painted_panels.insert(robot_position);
            }
            _ => ()
        } 
        current_facing = current_facing.rotate(direction);
        robot_position = current_facing.translate(robot_position);
        panels.canvas_limits.update(robot_position);
        let next_input = if panels.white_panels.contains(&robot_position) {
            PanelColor::White.to_i128()
        } else {
            PanelColor::Black.to_i128()
        };
        robot.add_input(next_input);
        robot.run();
    }
    panels
}

fn coordinate_to_index(coord: &(i128, i128), canvas_limits: &CanvasLimits)-> usize {
    let CanvasLimits {x_min, x_max, y_min, y_max} = canvas_limits;
    let (x, y) = coord;

    if x < x_min || x > x_max || y < y_min || y > y_max {
        panic!("Limits: {:?}, coord: {:?}", canvas_limits, coord);
    }

    ((y - y_min) * (x_max - x_min) + (x - x_min)) as usize
}

fn render(white_panel_set: &HashSet<(i128, i128)>, canvas_limits: CanvasLimits) {
    let CanvasLimits {x_min, x_max, y_min, y_max} = canvas_limits;
    let elements_count: usize = ((x_max - x_min + 1) * (y_max - y_min + 1)) as usize;
    let mut canvas_vec: Vec<PanelColor> = vec![PanelColor::Black; elements_count];
    for white_panel_coord in white_panel_set.iter() {
        canvas_vec[coordinate_to_index(white_panel_coord, &canvas_limits)] = PanelColor::White
    }

    let row_len = (x_max - x_min) as usize;

    for i in 0..elements_count {
        match &canvas_vec[i] {
            PanelColor::Black => print!("  "),
            PanelColor::White => print!("||"),
        }
        if (i + 1) % row_len == 0 {
            println!()
        }
    }
    println!();
}
