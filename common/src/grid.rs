use std::cmp::{self, max, min};
use std::default::Default;
use std::str::FromStr;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn shift(&self, direction: Direction, amount: u64) -> Coordinate {
        use Direction::*;
        let amount = amount as i64;
        let (x, y) = match direction {
            Left => (self.x - amount, self.y),
            Right => (self.x + amount, self.y),
            Up => (self.x, self.y + amount),
            Down => (self.x, self.y - amount),
        };
        Coordinate { x, y }
    }

    pub fn manhattan_distance(&self) -> u64 {
        let origin = Coordinate::default();
        (origin.x - self.x).abs() as u64 + (origin.y - self.y).abs() as u64
    }

    pub fn to_stepped_coordinate(&self, steps: u64) -> SteppedCoordinate {
        SteppedCoordinate {
            coordinate: *self,
            steps,
        }
    }
}

impl cmp::Ord for Coordinate {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl cmp::PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Coordinate {
    fn default() -> Coordinate {
        Coordinate::new(0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SteppedCoordinate {
    pub coordinate: Coordinate,
    pub steps: u64,
}

impl cmp::Ord for SteppedCoordinate {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl cmp::PartialOrd for SteppedCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for SteppedCoordinate {
    type Output = Self;

    fn add(self, other: Self)-> Self {
        SteppedCoordinate {
            coordinate: self.coordinate,
            steps: self.steps + other.steps
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Direction, String> {
        use Direction::*;
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(format!("'{}' is not a valid direction", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment {
    start_coordinate: Coordinate,
    end_coordinate: Coordinate,
    start_steps: u64,
    length: u64
}

impl LineSegment {
    pub fn new(start_point: Coordinate, direction: Direction, length: u64) -> LineSegment {
        LineSegment::new_with_steps(start_point, direction, length, 0)
    }

    pub fn new_with_steps(
        start_point: Coordinate,
        direction: Direction,
        length: u64,
        steps: u64,
    ) -> LineSegment {
        LineSegment {
            start_coordinate: start_point,
            end_coordinate: start_point.shift(direction, length),
            start_steps: steps,
            length
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start_coordinate.y == self.end_coordinate.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start_coordinate.x == self.end_coordinate.x
    }

    fn ordered_xs(&self) -> (i64, i64) {
        (
            min(self.start_coordinate.x, self.end_coordinate.x),
            max(self.start_coordinate.x, self.end_coordinate.x),
        )
    }

    fn ordered_ys(&self) -> (i64, i64) {
        (
            min(self.start_coordinate.y, self.end_coordinate.y),
            max(self.start_coordinate.y, self.end_coordinate.y),
        )
    }

    pub fn intersection_point(&self, other: &LineSegment) -> Option<Coordinate> {
        if (other.is_horizontal() && self.is_horizontal())
            || (self.is_vertical() && other.is_vertical())
        {
            return None;
        }

        let (horz_line, vert_line) = if self.is_horizontal() {
            (self, other)
        } else {
            (other, self)
        };
        let horz_y = horz_line.end_coordinate.y;
        let vert_x = vert_line.start_coordinate.x;
        let (min_horz_x, max_horz_x) = horz_line.ordered_xs();
        let (min_vert_y, max_vert_y) = vert_line.ordered_ys();
        if vert_x.is_between(min_horz_x, max_horz_x) && horz_y.is_between(min_vert_y, max_vert_y) {
            Some(Coordinate::new(vert_x, horz_y))
        } else {
            None
        }
    }

    pub fn contains(&self, coord: Coordinate) -> bool {
        let ((min_x, max_x), (min_y, max_y)) = (self.ordered_xs(), self.ordered_ys());
        coord.x.is_between(min_x, max_x) && coord.y.is_between(min_y, max_y)
    }

    pub fn steps_to(&self, coord: Coordinate)-> Option<u64> {
        if !self.contains(coord) {
            None
        } else {
            Some((
                self.start_coordinate.manhattan_distance() as i64
                - coord.manhattan_distance() as i64
            ).abs() as u64)
        }
    }

    pub fn with_steps_at(&self, coord: Coordinate) -> Option<SteppedCoordinate> {
        let steps_to = self.steps_to(coord)?;
        Some(coord.to_stepped_coordinate(self.start_steps + steps_to))
    }
}

trait Between {
    fn is_between(&self, lower: Self, upper: Self) -> bool;
}

impl<T> Between for T
where
    T: Ord,
{
    fn is_between(&self, lower: T, upper: T) -> bool {
        &lower <= self && self <= &upper
    }
}

pub struct Path {
    pub line_segments: Vec<LineSegment>,
}

impl Path {
    pub fn contains(&self, coord: Coordinate)-> bool {
        self.line_segments.iter().any(|line| {
            line.contains(coord)
        })
    }
    
    pub fn steps_to(&self, coord: Coordinate)-> Option<u64> {
        if !self.contains(coord) {
            None
        } else {
            Some(self.line_segments.iter().fold((0, false), |(steps, found), line| {
                if found {
                    (steps, found)
                } else {
                    if let Some(steps_to_coord) = line.steps_to(coord) {
                        (steps + steps_to_coord, true)  
                    } else {
                        (steps + line.length, false)
                    }
                }
            }).0)
        }
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_parts = s
            .split(',')
            .map(|p| {
                let direction = p[..1].parse::<Direction>()?;
                let length = p[1..]
                    .parse::<u64>()
                    .map_err(|_| "Failed to parse u64".to_owned())?;
                Ok((direction, length))
            })
            .collect::<Result<Vec<(Direction, u64)>, String>>()?;

        let mut prev_point = Coordinate::default();
        let mut total_steps = 0;
        let mut line_segments = vec![];
        for (direction, length) in parsed_parts {
            let new_segment = LineSegment::new_with_steps(prev_point, direction, length, total_steps);
            prev_point = new_segment.end_coordinate;
            total_steps += length;
            line_segments.push(new_segment);
        }
        Ok(Path { line_segments })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_computed_endpoints() {
        let line_seg_1 = LineSegment::new(Coordinate::default(), Direction::Up, 100);
        assert!(line_seg_1.end_coordinate == Coordinate::new(0, 100));
        let line_seg_2 = LineSegment::new(Coordinate::default(), Direction::Down, 0);
        assert!(line_seg_2.end_coordinate == Coordinate::new(0, 0));
    }

    #[test]
    fn test_intersection_point_same_direction() {
        let line_seg_1 = LineSegment::new(Coordinate::default(), Direction::Up, 100);
        let line_seg_2 = LineSegment::new(Coordinate::default(), Direction::Down, 100);

        // same direction
        assert!(
            line_seg_1.intersection_point(&line_seg_2).is_none(),
            "Should be no intersection point for lines on same orientation"
        );
    }
    #[test]
    fn test_intersection_point_middle_intersection() {
        // different direction, middle intersection
        let l1 = LineSegment::new(Coordinate::new(0, -10), Direction::Up, 20);
        let l2 = LineSegment::new(Coordinate::new(-10, 0), Direction::Right, 20);

        assert!(
            l1.intersection_point(&l2) == Some(Coordinate::new(0, 0)),
            "Should be an intersection at the origin"
        );
    }
    #[test]
    fn test_intersection_point_edge_intersection() {
        // different direction, one edge intersection, horz edge
        let l1 = LineSegment::new(Coordinate::new(0, -10), Direction::Up, 20);
        let l2 = LineSegment::new(Coordinate::new(0, -5), Direction::Right, 20);
        assert!(
            l1.intersection_point(&l2) == Some(Coordinate::new(0, -5)),
            "Should be an intersection point at (0, -5)"
        );

        // different direction, one edge intersection, vert edge
        let l1 = LineSegment::new(Coordinate::new(-5, 0), Direction::Up, 20);
        let l2 = LineSegment::new(Coordinate::new(-10, 0), Direction::Right, 20);
        assert!(
            l1.intersection_point(&l2) == Some(Coordinate::new(-5, 0)),
            "Should be an intersection point at (-5, 0)"
        );
    }

    #[test]
    fn test_intersection_double_edge() {
        let l1 = LineSegment::new(Coordinate::new(0, -5), Direction::Up, 20);
        let l2 = LineSegment::new(Coordinate::new(0, -5), Direction::Right, 20);
        assert!(
            l1.intersection_point(&l2) == Some(Coordinate::new(0, -5)),
            "Should be an intersection point at (0, -5)"
        );
    }

    #[test]
    fn test_steps_end() {
        let l1 = LineSegment::new_with_steps(Coordinate::default(), Direction::Right, 25, 60);
        let c1 = l1.end_coordinate;
        assert!(l1.with_steps_at(c1) == Some(SteppedCoordinate {
            coordinate: c1,
            steps: 85
        }));
    }

    #[test]
    fn test_steps_start() {
        let l1 = LineSegment::new_with_steps(Coordinate::default(), Direction::Down, 25, 60);
        let c1 = l1.start_coordinate;
        assert!(l1.with_steps_at(c1) == Some(SteppedCoordinate {
            coordinate: c1,
            steps: 60
        }));
    }

    #[test]
    fn test_steps_mid() {
        let l1 = LineSegment::new_with_steps(Coordinate::default(), Direction::Left, 25, 60);
        let c1 = Coordinate::new(-15, 0);
        assert!(l1.with_steps_at(c1) == Some(SteppedCoordinate {
            coordinate: c1,
            steps: 75
        }));
    }

    #[test]
    fn test_steps_off() {
        let l1 = LineSegment::new_with_steps(Coordinate::default(), Direction::Up, 25, 60);
        let c1 = Coordinate::new(-100, -100);
        assert!(l1.with_steps_at(c1).is_none());
    }


}
