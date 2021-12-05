use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    let file = File::open("day2.txt").unwrap();
    let reader = BufReader::new(file);

    let commands = reader.lines().filter_map(|line| {
        line.map(|s| {
            if let Some((command, offset)) = s.split_whitespace().collect_tuple() {
                let offset = offset.parse().unwrap();
                match command {
                    "forward" => Some((offset, 0)),
                    "down" => Some((0, offset)),
                    "up" => Some((0, -offset)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .ok()
        .flatten()
    });

    let (x, y) = commands
        .reduce(|(x, y), (delta_x, delta_y)| ((x + delta_x), (y + delta_y)))
        .unwrap();
    println!("X * Y: {}", x * y);
}
