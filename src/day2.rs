use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn try_parse(s: &str) -> Option<Self> {
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

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|line| Command::try_parse(line))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(commands: &[Command]) -> u32 {
    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command {
            Command::Forward(offset) => (x + offset, y),
            Command::Down(offset) => (x, y + offset),
            Command::Up(offset) => (x, y - offset),
        });
    x * y
}

#[aoc(day2, part2)]
pub fn part2(commands: &[Command]) -> u32 {
    let (_, x, y) = commands
        .iter()
        .fold((0, 0, 0), |(aim, x, y), command| match command {
            Command::Forward(offset) => (aim, x + offset, y + aim * offset),
            Command::Down(offset) => (aim + offset, x, y),
            Command::Up(offset) => (aim - offset, x, y),
        });
    x * y
}
