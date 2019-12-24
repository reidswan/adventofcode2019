use std::default::Default;
use std::str::{Chars, FromStr};
use std::fmt;

fn main() {
    let moons = get_parsed_input();
    part1(&moons);
    part2(&moons);
}

fn part1(src: &Vec<Moon>) {
    let mut moons = src.clone();

    let max = 1000;
    for _ in 0..max {
        step(&mut moons);
    }
    println!("Part 1: {}", moons.iter().map(|moon| moon.total_energy()).sum::<usize>())
 
}

fn part2(src: &Vec<Moon>) {
    let mut moons = src.clone();

    let (px, py, pz) = determine_periods(&mut moons);

    println!("Part 2 = {}", lcm(px, lcm(py, pz)));
}

fn lcm(a: usize, b: usize)-> usize {
    let div = gcd(a, b);
    (a/div) * b
}

fn gcd(a: usize, b: usize)-> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t
    }
    a
}

fn step(moons: &mut Vec<Moon>) {
    let select_x = |moon: &Moon| {moon.position.x};
    let select_y = |moon: &Moon| {moon.position.y};
    let select_z = |moon: &Moon| {moon.position.z};

    let modify_x = |moon: &mut Moon, new_val: isize| {
        moon.velocity.x += new_val;
        moon.position.x += moon.velocity.x;
    };
    let modify_y = |moon: &mut Moon, new_val: isize| {
        moon.velocity.y += new_val;
        moon.position.y += moon.velocity.y;
    };
    let modify_z = |moon: &mut Moon, new_val: isize| {
        moon.velocity.z += new_val;
        moon.position.z += moon.velocity.z;
    };
    
    fn sort_and_alter(moons: &mut Vec<Moon>, select: Box<dyn Fn(&Moon)-> isize>, modify: Box<dyn Fn(&mut Moon, isize)>) {
        let change_amount = moons.iter().map(|moon| {
            let moons_lt = moons.iter().filter(|other| select(other) < select(moon)).count();
            let moons_gt = moons.iter().filter(|other| select(other) > select(moon)).count();
            moons_gt as isize - moons_lt as isize
        }).collect::<Vec<isize>>();
        moons.iter_mut().zip(change_amount).for_each(|(moon, amt)| {
            modify(moon, amt);
        });
    }
    let closures: Vec<(Box<dyn Fn(&Moon)-> isize>, Box<dyn Fn(&mut Moon, isize)>)> = vec![
        (Box::new(select_x), Box::new(modify_x)),
        (Box::new(select_y), Box::new(modify_y)),
        (Box::new(select_z), Box::new(modify_z))    
    ];

    for (sele, modi) in closures.into_iter() {
        sort_and_alter(moons, sele, modi);
    }
}

fn determine_periods(moons: &mut Vec<Moon>)-> (usize, usize, usize) {
    let (mut period_x, mut period_y, mut period_z) = (None, None, None);
    let start_x = moons.iter().map(|m| m.xs()).collect::<Vec<_>>();
    let start_y = moons.iter().map(|m| m.ys()).collect::<Vec<_>>();
    let start_z = moons.iter().map(|m| m.zs()).collect::<Vec<_>>();
    let mut steps = 0;

    while period_x.is_none() || period_y.is_none() || period_z.is_none() {
        step(moons);
        steps += 1;
        let curr_x = moons.iter().map(|m| m.xs()).collect::<Vec<_>>();
        let curr_y = moons.iter().map(|m| m.ys()).collect::<Vec<_>>();
        let curr_z = moons.iter().map(|m| m.zs()).collect::<Vec<_>>();
        if let None = period_x {
            if curr_x == start_x {
                period_x = Some(steps)
            }
        }
        if let None = period_y {
            if curr_y == start_y {
                period_y = Some(steps)
            }
        }
        if let None = period_z {
            if curr_z == start_z {
                period_z = Some(steps)
            }
        }
    }

    (period_x.unwrap(), period_y.unwrap(), period_z.unwrap())
}

fn get_parsed_input()-> Vec<Moon> {
    let input = include_str!("input/input");
    input.lines().filter_map(|line| {
        line.parse::<Moon>().ok()
    }).collect()
}

impl FromStr for Moon {
    type Err = String;

    /// An extremely brittle parser
    fn from_str(src: &str) -> Result<Moon, Self::Err> {
        let mut char_src = src.chars();
        char_src.next(); // '<'
        char_src.next(); // 'x'
        char_src.next(); // '='
        let x = get_number_at_head(&mut char_src)?; // also consumes the ','
        char_src.next(); // ' '
        char_src.next(); // 'y'
        char_src.next(); // '='
        let y = get_number_at_head(&mut char_src)?; // also consumes the ','
        char_src.next(); // ' '
        char_src.next(); // 'z'
        char_src.next(); // '='
        let z = get_number_at_head(&mut char_src)?; // also consumes the '>'
        Ok(Moon {
            position: Vector::new(x, y, z),
            velocity: Vector::default(),
        })
    }
}

fn get_number_at_head(chars: &mut Chars<'_>) -> Result<isize, String> {
    let mut coll = String::new();
    let first_ch = chars.next().ok_or(String::from(
        "Empty iterator when attempting to read number",
    ))?;
    if first_ch != '-' && !first_ch.is_numeric() {
        return Err(format!("Invalid char: {}", first_ch));
    }
    coll.push(first_ch);
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            coll.push(c)
        } else {
            break;
        }
    }
    coll.parse::<isize>()
        .map_err(|_| format!("Failed to parse isize from {}", &coll))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Vector {
        Vector { x, y, z }
    }

    fn abs_sum(&self)-> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
    
    fn as_tuple(&self)-> (isize, isize, isize) {
        (self.x, self.y, self.z)
    }
}

impl Default for Vector {
    fn default() -> Vector {
        Vector { x: 0, y: 0, z: 0 }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn kinetic_energy(&self)-> usize {
        self.velocity.abs_sum()
    }

    fn potential_energy(&self)-> usize {
        self.position.abs_sum()
    }

    fn total_energy(&self)-> usize {
        self.kinetic_energy() * self.potential_energy()
    }

    fn xs(&self)-> (isize, isize) {
        (self.position.x, self.velocity.x)
    }

    fn ys(&self)-> (isize, isize) {
        (self.position.y, self.velocity.y)
    } 

    fn zs(&self)-> (isize, isize) {
        (self.position.z, self.velocity.z)
    }
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Moon {{ pos: {:?}, vel: {:?} }}", self.position.as_tuple(), self.velocity.as_tuple())
    }
}