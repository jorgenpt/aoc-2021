use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(tuple: (u32, u32)) -> Self {
        Self {
            x: tuple.0 - 1,
            y: tuple.1 - 1,
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
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
                if p1.x == p2.x || p1.y == p2.y {
                    max_x = max_x.max(p1.x).max(p2.x);
                    max_y = max_y.max(p1.y).max(p2.y);
                    Some(Line { p1, p2 })
                } else {
                    None
                }
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
        let (y_min, y_max) = if line.p1.y < line.p2.y {
            (line.p1.y, line.p2.y)
        } else {
            (line.p2.y, line.p1.y)
        };
        for y in y_min..=y_max {
            let y_coord = y * width;
            let (x_min, x_max) = if line.p1.x < line.p2.x {
                (line.p1.x, line.p2.x)
            } else {
                (line.p2.x, line.p1.x)
            };
            for x in x_min..=x_max {
                let coord = (y_coord + x) as usize;
                if let Overlap::One = points[coord] {
                    num_points_with_multiple_overlaps += 1
                }
                points[coord].increment();
            }
        }
    }

    println!("{}", num_points_with_multiple_overlaps);
}
