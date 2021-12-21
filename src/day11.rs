use std::fmt::Debug;

use aoc_2021::AocMap;
use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum EnergyLevel {
    Charging(u8),
    Discharged,
}

impl Debug for EnergyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Charging(l) = self {
            write!(f, "{:?}", l)
        } else {
            write!(f, "X")
        }
    }
}

type EnergyMap = AocMap<EnergyLevel>;

trait EnergyMapSimulate {
    fn simulate_time(&mut self);
    fn simulate_flashes(&mut self) -> usize;
}

impl EnergyMapSimulate for EnergyMap {
    fn simulate_time(&mut self) {
        for value in &mut self.values {
            if let EnergyLevel::Charging(l) = value {
                *value = EnergyLevel::Charging(*l + 1)
            } else {
                *value = EnergyLevel::Charging(1)
            }
        }
    }

    fn simulate_flashes(&mut self) -> usize {
        let mut num_flashes = 0;
        loop {
            let flashes = self
                .coordinates()
                .filter_map(|(x, y)| {
                    if let EnergyLevel::Charging(l) = self.get_value(x, y).unwrap() {
                        if l > 9 {
                            Some((x, y))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect_vec();

            if flashes.len() > 0 {
                num_flashes += flashes.len();
                for (flash_x, flash_y) in flashes {
                    self.set_value(flash_x, flash_y, EnergyLevel::Discharged);
                    for (offset_x, offset_y) in EnergyMap::ALL_NEIGHBORS {
                        let (x, y) = (flash_x + offset_x, flash_y + offset_y);
                        if let Some(EnergyLevel::Charging(l)) = self.get_value(x, y) {
                            self.set_value(x, y, EnergyLevel::Charging(l + 1));
                        }
                    }
                }
            } else {
                break;
            }
        }
        num_flashes
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> EnergyMap {
    AocMap::<EnergyLevel>::generator(input, |c| match c {
        c if c >= '0' && c <= '9' => Some(EnergyLevel::Charging(c as u8 - '0' as u8)),
        _ => None,
    })
}

#[aoc(day11, part1)]
pub fn part1(levels: &EnergyMap) -> usize {
    let mut levels = levels.to_owned();
    let mut total_flashes = 0;

    for _ in 0..100 {
        levels.simulate_time();
        total_flashes += levels.simulate_flashes();
    }

    total_flashes
}

#[aoc(day11, part2)]
pub fn part2(levels: &EnergyMap) -> usize {
    let mut levels = levels.to_owned();

    for iteration in 1..usize::MAX {
        levels.simulate_time();
        if levels.simulate_flashes() == levels.values.len() {
            return iteration;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223\n\
                         2745854711\n\
                         5264556173\n\
                         6141336146\n\
                         6357385478\n\
                         4167524645\n\
                         2176841721\n\
                         6882881134\n\
                         4846848554\n\
                         5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(INPUT)), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(INPUT)), 195);
    }
}
