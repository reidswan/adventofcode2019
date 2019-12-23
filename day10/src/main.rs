use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::ops::Index;

type Coord = (usize, usize);

fn main() {
    let input_str = include_str!("input/input");
    let x_max = input_str.lines().next().unwrap().len();
    let points = input_str
        .chars()
        .filter_map(|a| {
            if a == '#' {
                Some(true)
            } else if a == '.' {
                Some(false)
            } else {
                None
            }
        })
        .collect::<Vec<bool>>();

    let y_max = points.len() / x_max;

    let asteroids = Plane {
        points,
        row_length: x_max,
    };

    let mut lines = HashMap::<Line, Vec<Coord>>::new();

    let size = asteroids.points.len();
    let mut max_visible = 0;
    let mut max_asteroid = None;
    let mut max_lines_considered = HashSet::new();
    for i in 0..size {
        let coord_i = asteroids.index_to_coord(i);
        if !asteroids[&coord_i] {
            continue;
        }
        let mut visible = 0;
        let mut lines_considered = HashSet::new();
        for j in 0..size {
            let coord_j = asteroids.index_to_coord(j);
            if i == j || !asteroids[&coord_j] {
                continue;
            }
            let line = Line::new(coord_i, coord_j, (x_max, y_max));
            if lines_considered.contains(&line) {
                continue;
            }
            if !lines.contains_key(&line) {
                let coords_on_line = line
                    .points_within()
                    .iter()
                    .filter(|c| asteroids[c])
                    .map(|&c| c)
                    .collect::<Vec<_>>();
                lines.insert(line, coords_on_line);
            }
            let coords = &lines[&line];
            visible += count_visible(coords, &coord_i);
            lines_considered.insert(line);
        }

        if visible > max_visible {
            max_visible = visible;
            max_asteroid = Some(coord_i);
            max_lines_considered = lines_considered;
        }
    }
    println!("Part 1 = {}", max_visible);

    let mut sorted = max_lines_considered.iter().collect::<Vec<_>>();
    sorted.sort();
    let mut asteroids = 200;
    for line in sorted.iter() {
        let points = &lines[line];
        let sub = count_visible(points, &max_asteroid.unwrap());
        if asteroids > sub {
            asteroids -= sub
        } else {
            println!("Within {:?}", points);
            break;
        }
    }
}

struct Plane<T> {
    points: Vec<T>,
    row_length: usize,
}

impl<T> Plane<T> {
    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.row_length, index / self.row_length)
    }
}

impl<T> Index<&Coord> for Plane<T> {
    type Output = T;

    fn index(&self, index: &Coord) -> &Self::Output {
        let (x, y) = index;
        let actual_index = y * self.row_length + x;
        if x >= &self.row_length || actual_index >= self.points.len() {
            panic!("{:?} is out of range!", index);
        } else {
            &self.points[actual_index]
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Line {
    lowest_point: Coord,
    dx: isize,
    dy: isize,
    plane_limits: Coord,
}

impl Line {
    fn new(p1: Coord, p2: Coord, plane_limits: Coord) -> Self {
        if p1 == p2 {
            panic!("Cannot construct a line from the same coordinate");
        }
        // ensure that dy is always positive (or dx is positive if dy == 0)
        let (p1, p2) = if p1.1 < p2.1 || (p1.1 == p2.1 && p1.0 < p2.0) {
            (p1, p2)
        } else {
            (p2, p1)
        };
        let ((x1, y1), (x2, y2)) = (p1, p2);
        let (dx, dy) = normalize_ratio(x2 as isize - x1 as isize, y2 as isize - y1 as isize);
        let (max_x, max_y) = plane_limits;
        let mut curr_x = x1 as isize;
        let mut curr_y = y1 as isize;

        // find the point within limits with y closest to 0,
        // or with x closest to 0 if dy == 0
        let within_limits =
            |x: isize, y: isize| x >= 0 && (x as usize) < max_x && y >= 0 && (y as usize) < max_y;
        while within_limits(curr_y - dy, curr_x - dx) {
            curr_y -= dy;
            curr_x -= dx;
        }

        Line {
            dx,
            dy,
            lowest_point: (curr_x as usize, curr_y as usize),
            plane_limits,
        }
    }

    /// Return a vector of all integer coordinates (x, y)
    /// on the line such that 0 <= x < max_x and 0 <= y < max_y
    fn points_within(&self) -> Vec<Coord> {
        let (max_x, max_y) = self.plane_limits;
        if self.dy == 0 {
            // horizontal line
            return (0..max_x).map(|x| (x, self.lowest_point.1)).collect();
        } else if self.dx == 0 {
            // vertical line
            return (0..max_y).map(|y| (self.lowest_point.0, y)).collect();
        }

        let mut points = vec![self.lowest_point];
        let mut curr_y = self.lowest_point.1 as isize;
        let mut curr_x = self.lowest_point.0 as isize;
        let within_limits =
            |x: isize, y: isize| x >= 0 && (x as usize) < max_x && y >= 0 && (y as usize) < max_y;

        while within_limits(curr_x + self.dx, curr_y + self.dy) {
            curr_x += self.dx;
            curr_y += self.dy;
            points.push((curr_x as usize, curr_y as usize))
        }

        points
    }
    fn quadrant(&self) -> Quadrant {
        if self.dx >= 0 && self.dy >= 0 {
            Quadrant::First
        } else if self.dx < 0 && self.dy >= 0 {
            Quadrant::Second
        } else if self.dx < 0 && self.dy < 0 {
            Quadrant::Third
        } else {
            Quadrant::Fourth
        }
    }
}

#[derive(Eq, PartialEq)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

const QUADRANT_ORDERING: [Quadrant; 4] = [
    Quadrant::Fourth,
    Quadrant::First,
    Quadrant::Second,
    Quadrant::Third,
];

impl Ord for Quadrant {
    fn cmp(&self, other: &Quadrant) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            for q in QUADRANT_ORDERING.iter() {
                if q == self {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            // satisfy the type checker, but won't be reached
            Ordering::Equal
        }
    }
}
impl PartialOrd for Quadrant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dx == 0 && other.dx == 0 {
            return Ordering::Equal
        }
        let quadrant_cmp = self.quadrant().cmp(&other.quadrant());
        if quadrant_cmp != Ordering::Equal {
            return quadrant_cmp
        } 
        let self_angle = (self.dy as f64/self.dx as f64).atan();
        let other_angle = (other.dy as f64/self.dx as f64).atan();

        if self_angle == other_angle {
            Ordering::Equal
        } else if self_angle > other_angle {
            Ordering::Greater
        } else {
            Ordering::Less
        }

    }
}

/*
              (0,-1)
                |
                |
(-1, 0)-------------------( 1, 0)
                |
                |
              (0,1)
*/

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// given a set of ordered colinear points and a point in that set
/// return how many are visible
fn count_visible(points: &Vec<(Coord)>, point: &Coord) -> usize {
    if points.len() <= 1 {
        0
    } else if point == &points[0] || point == points.last().unwrap() {
        1
    } else {
        2
    }
}

fn normalize_ratio(a: isize, b: isize) -> (isize, isize) {
    let gcd = gcd(a, b);
    (a / gcd, b / gcd)
}

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let b_next = a % b;
        a = b;
        b = b_next;
    }
    a
}
