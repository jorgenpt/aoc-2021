use std::fmt::Debug;

use aoc_2021::{AocMap, Point};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq)]
pub enum Occupancy {
    Occupied,
    Empty,
}

impl Debug for Occupancy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Occupied => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

type Transparent = AocMap<Occupancy>;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    FoldAlongX(usize),
    FoldAlongY(usize),
}
trait Day13 {
    fn fold(&self, instruction: &Instruction) -> Self;
}

impl Day13 for Transparent {
    fn fold(&self, instruction: &Instruction) -> Self {
        let size = match instruction {
            Instruction::FoldAlongX(x) => Point::new(*x, self.size.y),
            Instruction::FoldAlongY(y) => Point::new(self.size.x, *y),
        };

        let mut values = Vec::new();
        values.resize(size.x * size.y, Occupancy::Empty);
        let mut map = Self { size, values };
        for p in map.coordinates() {
            map.set(p, self.get(p));
        }

        match instruction {
            Instruction::FoldAlongX(fold_x) => {
                let fold_width = size.x.min(self.size.x - size.x - 1);
                for y in 0..size.y {
                    for x in 1..=fold_width {
                        if Occupancy::Occupied == self.get(Point::new(fold_x + x, y)) {
                            map.set(Point::new(fold_x - x, y), Occupancy::Occupied)
                        }
                    }
                }
            }

            Instruction::FoldAlongY(fold_y) => {
                let fold_height = size.y.min(self.size.y - size.y - 1);
                for y in 1..=fold_height {
                    for x in 0..size.x {
                        if Occupancy::Occupied == self.get(Point::new(x, fold_y + y)) {
                            map.set(Point::new(x, fold_y - y), Occupancy::Occupied)
                        }
                    }
                }
            }
        }

        map
    }
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (Transparent, Vec<Instruction>) {
    let mut transparent = Transparent {
        size: Point::new(0, 0),
        values: Vec::new(),
    };

    let mut instructions = Vec::new();

    let mut lines = input.lines();

    let mut points = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if let Some((x, y)) = line
            .split(',')
            .map(str::parse::<usize>)
            .filter_map(|s| s.ok())
            .collect_tuple()
        {
            transparent.size.y = transparent.size.y.max(y + 1);
            transparent.size.x = transparent.size.x.max(x + 1);
            transparent.values.resize(
                transparent.size.y as usize * transparent.size.x as usize,
                Occupancy::Empty,
            );

            points.push(Point::new(x, y));
        }
    }

    for point in points {
        transparent.set(point, Occupancy::Occupied);
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if line.starts_with("fold along x=") {
            if let Ok(x) = line["fold along x=".len()..].parse::<usize>() {
                instructions.push(Instruction::FoldAlongX(x));
            }
        } else if line.starts_with("fold along y=") {
            if let Ok(y) = line["fold along y=".len()..].parse::<usize>() {
                instructions.push(Instruction::FoldAlongY(y));
            }
        } else {
            panic!("Can't find valid instruction: {}", line);
        }
    }

    (transparent, instructions)
}

#[aoc(day13, part1)]
pub fn part1((transparent, instructions): &(Transparent, Vec<Instruction>)) -> usize {
    let folded_transparent = transparent.fold(&instructions[0]);
    folded_transparent
        .values
        .into_iter()
        .filter(|c| *c == Occupancy::Occupied)
        .count()
}

#[aoc(day13, part2)]
pub fn part2((transparent, instructions): &(Transparent, Vec<Instruction>)) -> String {
    format!(
        "\n{:?}",
        instructions
            .into_iter()
            .fold(transparent.to_owned(), |accum, instruction| accum
                .fold(instruction))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10\n\
                         0,14\n\
                         9,10\n\
                         0,3\n\
                         10,4\n\
                         4,11\n\
                         6,0\n\
                         6,12\n\
                         4,1\n\
                         0,13\n\
                         10,12\n\
                         3,4\n\
                         3,0\n\
                         8,4\n\
                         1,10\n\
                         2,14\n\
                         8,10\n\
                         9,0\n\
                         \n\
                         fold along y=7\n\
                         fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(INPUT)), 17);
    }
}
