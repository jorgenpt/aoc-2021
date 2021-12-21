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
        let mut new_levels = EnergyMap {
            height: levels.height,
            width: levels.width,
            values: levels
                .values
                .into_iter()
                .map(|l| {
                    if let EnergyLevel::Charging(l) = l {
                        EnergyLevel::Charging(l + 1)
                    } else {
                        EnergyLevel::Charging(1)
                    }
                })
                .collect_vec(),
        };

        loop {
            let flashes = new_levels
                .coordinates()
                .filter_map(|(x, y)| {
                    if let EnergyLevel::Charging(l) = new_levels.get_value(x, y).unwrap() {
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
                total_flashes += flashes.len();
                for (flash_x, flash_y) in flashes {
                    new_levels.set_value(flash_x, flash_y, EnergyLevel::Discharged);
                    for (offset_x, offset_y) in EnergyMap::ALL_NEIGHBORS {
                        let (x, y) = (flash_x + offset_x, flash_y + offset_y);
                        if let Some(EnergyLevel::Charging(l)) = new_levels.get_value(x, y) {
                            new_levels.set_value(x, y, EnergyLevel::Charging(l + 1));
                        }
                    }
                }
            } else {
                break;
            }
        }

        levels = new_levels;
    }

    total_flashes
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
}
