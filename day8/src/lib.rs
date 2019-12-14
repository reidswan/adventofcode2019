use std::fmt;
use std::ops::{Add, Index};

fn to_color(c: char) -> Color {
    use Color::*;
    match c {
        '0' => Black,
        '1' => White,
        '2' => Transparent,
        _ => panic!("Not a color digit: {}", c),
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
    Transparent,
}

impl Add for Color {
    type Output = Color;

    /// Not commutative
    fn add(self, other: Self) -> Self::Output {
        use Color::*;
        if self == Transparent {
            other
        } else {
            self
        }
    }
}

struct Layers<'a> {
    layers: &'a [Color],
    width: usize,
    height: usize,
    layers_count: usize,
}

impl<'a> Layers<'a> {
    pub fn new(src: &'a [Color], width: usize, height: usize) -> Self {
        Layers {
            layers: src,
            width,
            height,
            layers_count: src.len() / (width * height),
        }
    }

    pub fn iter(&self) -> LayerIterator {
        LayerIterator {
            layers: self,
            index: 0,
        }
    }

    fn collapse(&self) -> Vec<Color> {
        use Color::*;
        let mut result_layer = vec![Transparent; self.width * self.height];

        for layer in self.iter() {
            for i in 0..result_layer.len() {
                result_layer[i] = result_layer[i] + layer[i]
            }
        }

        result_layer
    }
}

impl<'a> fmt::Display for Layers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let final_layer = self.collapse();
        // capacity: 2 chars per color + 1 newline per row + 1 for shits and giggles
        let mut result_string = String::with_capacity(2 * final_layer.len() + self.height + 1);
        for (index, color) in final_layer.iter().enumerate() {
            result_string.push_str(match color {
                Color::White => "||",
                _ => "  ",
            });
            if index % self.width == self.width - 1 {
                result_string.push_str("\n");
            }
        }
        write!(f, "{}", result_string)
    }
}

impl<'a> Index<usize> for Layers<'a> {
    type Output = [Color];

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.layers_count {
            panic!(
                "Index {} is out of range of {} layers",
                index, self.layers_count
            );
        }
        let offset = self.width * self.height * index;
        let limit = self.width * self.height * (index + 1);

        &self.layers[offset..limit]
    }
}

struct LayerIterator<'a> {
    layers: &'a Layers<'a>,
    index: usize,
}

impl<'a> Iterator for LayerIterator<'a> {
    type Item = &'a [Color];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.layers.layers_count {
            None
        } else {
            self.index += 1;
            Some(&self.layers[self.index - 1])
        }
    }
}

fn count_colors(colors: &[Color]) -> ColorCount {
    use Color::*;

    let mut counts = ColorCount {
        black: 0,
        white: 0,
        transparent: 0,
    };

    for color in colors {
        match color {
            Black => counts.black += 1,
            White => counts.white += 1,
            Transparent => counts.transparent += 1,
        }
    }

    counts
}

struct ColorCount {
    black: usize,
    white: usize,
    transparent: usize,
}

pub fn get_parsed_input() -> Vec<Color> {
    let input = include_str!("input/input").trim();
    input.chars().map(|a| to_color(a)).collect()
}

pub fn part1(colors: &Vec<Color>) {
    let (width, height) = (25, 6);
    let layers = Layers::new(colors, width, height);
    let result = layers
        .iter()
        .fold(None, |acc, slice| {
            let counts = count_colors(slice);
            if let Some(ColorCount { black, .. }) = acc {
                if black < counts.black {
                    return acc;
                }
            }
            Some(counts)
        })
        .unwrap();

    println!("Part 1 = {}", result.white * result.transparent);
}

pub fn part2(colors: &Vec<Color>) {
    let (width, height) = (25, 6);
    println!("Part 2 = \n{}", Layers::new(colors, width, height));
}
