use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn try_parse(s: String) -> Option<Self> {
        if let Some((command, offset)) = s.split_whitespace().collect_tuple() {
            let offset = offset.parse().unwrap();
            match command {
                "forward" => Some(Command::Forward(offset)),
                "down" => Some(Command::Down(offset)),
                "up" => Some(Command::Up(offset)),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn main() {
    let file = File::open("day2.txt").unwrap();
    let reader = BufReader::new(file);

    let commands = reader
        .lines()
        .filter_map(|line| line.map(Command::try_parse).ok().flatten())
        .collect::<Vec<_>>();

    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command {
            Command::Forward(offset) => (x + offset, y),
            Command::Down(offset) => (x, y + offset),
            Command::Up(offset) => (x, y - offset),
        });
    println!("Part 1, X * Y: {}", x * y);

    let (_, x, y) = commands
        .iter()
        .fold((0, 0, 0), |(aim, x, y), command| match command {
            Command::Forward(offset) => (aim, x + offset, y + aim * offset),
            Command::Down(offset) => (aim + offset, x, y),
            Command::Up(offset) => (aim - offset, x, y),
        });
    println!("Part 2, X * Y: {}", x * y);
}
