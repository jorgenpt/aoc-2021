use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(tuple: (u32, u32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Slope {
    Up,
    Down,
}

impl Slope {
    fn apply(&self, y_start: u32, step: u32) -> u32 {
        match self {
            Self::Up => y_start + step,
            Self::Down => (y_start as i64 - step as i64) as u32,
        }
    }
}

#[derive(Debug)]
enum Line {
    Horizontal {
        x_min: u32,
        x_max: u32,
        y: u32,
    },
    Vertical {
        x: u32,
        y_min: u32,
        y_max: u32,
    },
    Diagonal {
        x_min: u32,
        x_max: u32,
        y_start: u32,
        slope: Slope,
    },
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        let (x_min, x_max, y_start, y_end) = if p1.x < p2.x {
            (p1.x, p2.x, p1.y, p2.y)
        } else {
            (p2.x, p1.x, p2.y, p1.y)
        };
        let (y_min, y_max) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };
        if p1.y == p2.y {
            Self::Horizontal {
                x_min,
                x_max,
                y: p1.y,
            }
        } else if p1.x == p2.x {
            Self::Vertical {
                x: p1.x,
                y_min,
                y_max,
            }
        } else if x_max - x_min == y_max - y_min {
            Self::Diagonal {
                x_min,
                x_max,
                y_start,
                slope: if y_start > y_end {
                    Slope::Down
                } else {
                    Slope::Up
                },
            }
        } else {
            panic!("Unsupported line type");
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Overlap {
    None,
    One,
    Multiple,
}

impl Overlap {
    fn increment(&mut self) {
        *self = match self {
            Overlap::None => Overlap::One,
            Overlap::One => Overlap::Multiple,
            Overlap::Multiple => Overlap::Multiple,
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("day5.txt").unwrap());

    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let lines = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let components = line.split(" -> ").map(|comp| {
                comp.split(",")
                    .map(str::parse::<u32>)
                    .filter_map(|v| v.ok())
            });
            if let Some((v1, v2)) = components.collect_tuple() {
                let p1 = Point::new(v1.collect_tuple().unwrap());
                let p2 = Point::new(v2.collect_tuple().unwrap());
                max_x = max_x.max(p1.x).max(p2.x);
                max_y = max_y.max(p1.y).max(p2.y);
                let line = Line::new(p1, p2);
                Some(line)
            } else {
                None
            }
        })
        .filter_map(|line| line)
        .collect::<Vec<_>>();

    let mut points = Vec::new();
    let width = max_x + 1;
    let height = max_y + 1;
    points.resize((height * width) as usize, Overlap::None);

    let mut num_points_with_multiple_overlaps = 0u16;
    for line in lines {
        let mark_point = |(x, y)| {
            let coord = (y * width + x) as usize;
            if let Overlap::One = points[coord] {
                num_points_with_multiple_overlaps += 1
            }
            points[coord].increment();
        };

        match line {
            Line::Horizontal { x_min, x_max, y } => {
                (x_min..=x_max).zip(repeat(y)).for_each(mark_point)
            }
            Line::Vertical { x, y_min, y_max } => repeat(x).zip(y_min..=y_max).for_each(mark_point),
            Line::Diagonal {
                x_min,
                x_max,
                y_start,
                slope,
            } => {
                let points = (x_min..=x_max)
                    .enumerate()
                    .map(|(step, x)| (x, slope.apply(y_start, step as u32)))
                    .collect::<Vec<_>>();

                points.into_iter().for_each(mark_point);
            }
        };
    }

    println!(
        "Points with multiple thermal vent clouds: {}",
        num_points_with_multiple_overlaps
    );
}
